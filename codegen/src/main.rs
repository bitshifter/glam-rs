mod outputs;

use anyhow::bail;
use clap::{arg, command};
use rustfmt_wrapper::rustfmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use outputs::build_output_pairs;

const GLAM_ROOT: &str = "..";

fn is_modified(repo: &git2::Repository, output_path: &str) -> anyhow::Result<bool> {
    match repo.status_file(Path::new(output_path)) {
        Ok(status) => {
            Ok(status.is_wt_modified())
            // if status.is_wt_modified() {
            //     bail!("{} is already modified, revert or stash your changes.", output_path);
            // }
        }
        Err(e) => {
            if e.code() == git2::ErrorCode::NotFound {
                Ok(false)
            } else {
                bail!("git file status failed for {}: {}", output_path, e);
            }
        }
    }
}

fn generate_file(
    tera: &tera::Tera,
    context: &tera::Context,
    template_path: &str,
) -> anyhow::Result<String> {
    let buffer = match tera.render(template_path, context) {
        Ok(output) => output,
        Err(e) => {
            bail!("tera error encountered: {}", e);
        }
    };

    Ok(buffer)
}

fn main() -> anyhow::Result<()> {
    // Change into `./codegen` dir for convenience to the user
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR"))?;

    let matches = command!()
        .arg(arg!([GLOB]))
        .arg(arg!(-f - -force))
        .arg(arg!(-s - -stdout))
        .arg(arg!(-n - -nofmt))
        .arg(arg!(--check))
        .arg(arg!(-v - -verbose))
        .get_matches();

    let force = matches.is_present("force");
    let stdout = matches.is_present("stdout");
    let fmt_output = !matches.is_present("nofmt");
    let output_path_glob = matches.value_of("GLOB");
    let check = matches.is_present("check");
    let verbose = matches.is_present("verbose");

    if stdout && output_path_glob.is_none() {
        bail!("specify a single file to output to stdout.");
    }

    let glob = if let Some(output_path_glob) = output_path_glob {
        match globset::Glob::new(output_path_glob) {
            Ok(glob) => Some(glob.compile_matcher()),
            Err(e) => bail!("failed to compile glob: {}", e),
        }
    } else {
        None
    };
    let tera = match tera::Tera::new("templates/**/*.rs.tera") {
        Ok(t) => t,
        Err(e) => bail!("Parsing error(s): {}", e),
    };

    let repo = match git2::Repository::open(GLAM_ROOT) {
        Ok(repo) => repo,
        Err(e) => bail!("failed to open git repo: {}", e),
    };
    let workdir = repo.workdir().unwrap();

    let output_pairs = build_output_pairs();

    let mut output_paths = vec![];
    if let Some(glob) = glob {
        for k in output_pairs.keys() {
            if glob.is_match(k) {
                output_paths.push(k)
            }
        }
        if output_paths.is_empty() {
            bail!("no matching output paths found for '{}'.", glob.glob());
        };
    } else {
        for k in output_pairs.keys() {
            output_paths.push(k)
        }
    };

    output_paths.sort();

    let mut output_differences = 0;
    for output_path in output_paths {
        if !check {
            println!("generating {}", output_path);
        }

        let context = output_pairs.get(output_path).unwrap();
        let template_path = context.get("template_path").unwrap().as_str().unwrap();

        if !(check || force || stdout) && is_modified(&repo, output_path)? {
            bail!(
                "{} is already modified, use  `-f` to force overwrite or revert local changes.",
                output_path
            );
        }

        let mut output_str = generate_file(&tera, context, template_path)?;

        if fmt_output || check {
            match rustfmt(&output_str) {
                Ok(output) => output_str = output,
                Err(e) => {
                    bail!("rustfmt error encountered: {}", e);
                }
            }
        }

        let full_output_path = workdir.join(output_path);

        if check {
            match File::open(&full_output_path) {
                Ok(mut file) => {
                    use std::io::Read;
                    let mut original_str = String::new();
                    if let Err(e) = file.read_to_string(&mut original_str) {
                        println!("'{output_path}' could not be read: {e}");
                        output_differences += 1;
                    } else if output_str != original_str {
                        println!("'{output_path}' is different");
                        output_differences += 1;
                    } else if verbose {
                        println!("'{output_path}' is the same");
                    }
                }
                Err(e) => {
                    println!("{output_path} could not be opened: {e}");
                    output_differences += 1;
                }
            };
            continue;
        }

        if stdout {
            print!("{}", output_str);
            continue;
        }

        let mut output_file = match File::create(&full_output_path) {
            Ok(file) => file,
            Err(e) => {
                bail!("failed to create {}: {}", full_output_path.display(), e);
            }
        };

        if let Err(e) = write!(output_file, "{}", output_str) {
            bail!("failed to write output: {}", e);
        }
    }

    if check && output_differences > 0 {
        bail!("{output_differences} files were different");
    }

    Ok(())
}
