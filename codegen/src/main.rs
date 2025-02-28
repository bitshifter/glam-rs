use anyhow::{bail, Context};
use clap::{arg, command};
use rustfmt_wrapper::rustfmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use tera::{from_value, to_value};

// use outputs::build_output_pairs;

const GLAM_ROOT: &str = "..";
const CONFIG_FILE: &str = "codegen.json";

#[derive(Serialize, Deserialize)]
struct Config {
    version: u32,
    templates: BTreeMap<String, Template>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Template {
    properties: BTreeMap<String, Option<serde_json::Value>>,
    outputs: BTreeMap<String, Output>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Output {
    properties: BTreeMap<String, serde_json::Value>,
}

impl Config {
    fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    fn build_output_pairs(&self) -> anyhow::Result<BTreeMap<String, tera::Context>> {
        match self.version {
            1 => self.build_output_pairs_v1(),
            _ => Err(anyhow::Error::msg("Unexpected config file version")),
        }
    }

    fn build_output_pairs_v1(&self) -> anyhow::Result<BTreeMap<String, tera::Context>> {
        let mut output_pairs = BTreeMap::new();
        for (template_path, template) in self.templates.iter() {
            for (output_path, output) in template.outputs.iter() {
                let mut context = tera::Context::new();
                context.insert("template_path", template_path);
                for (prop_key, prop_value) in template.properties.iter() {
                    if let Some(prop_override) = output.properties.get(prop_key) {
                        context.insert(prop_key, prop_override);
                    } else {
                        // TODO: error message
                        if let Some(prop_value) = prop_value {
                            context.insert(prop_key, prop_value);
                        } else {
                            return Err(anyhow::Error::msg("Missing property override"));
                        }
                    }
                }
                output_pairs.insert(output_path.clone(), context);
            }
        }
        Ok(output_pairs)
    }

    // fn from(output_pairs: std::collections::HashMap<&str, tera::Context>) -> Option<Config> {
    //     let mut config = Config::new();
    //     for (output_path, tera_context) in output_pairs.into_iter() {
    //         let json_context = tera_context.into_json();
    //         let context_map = json_context.as_object()?;
    //         let template_path = context_map.get("template_path")?.as_str()?;
    //         if !config.templates.contains_key(template_path) {
    //             let mut template = Template::default();
    //             for (property, value) in context_map.iter() {
    //                 if property == "template_path" {
    //                     continue;
    //                 }
    //                 if value.is_boolean() {
    //                     template.properties.insert(
    //                         property.clone(), Some(serde_json::Value::Bool(false))
    //                     );
    //                 } else if value.is_i64() {
    //                     template
    //                         .properties
    //                         .insert(property.clone(), None);
    //                 } else if value.is_string() {
    //                     template
    //                         .properties
    //                         .insert(property.clone(), None);
    //                 } else {
    //                     unimplemented!();
    //                 }
    //             }
    //             config.templates.insert(template_path.to_string(), template);
    //         }
    //         let mut output = Output::default();
    //         for (property, value) in context_map.iter() {
    //             if property == "template_path" {
    //                 continue;
    //             }
    //             if let Some(bool_value) = value.as_bool() {
    //                 if bool_value {
    //                     output
    //                         .properties
    //                         .insert(property.clone(), serde_json::Value::Bool(bool_value));
    //                 }
    //             } else if let Some(i64_value) = value.as_i64() {
    //                 output.properties.insert(
    //                     property.clone(),
    //                     serde_json::Value::Number(i64_value.into()),
    //                 );
    //             } else if let Some(str_value) = value.as_str() {
    //                 output.properties.insert(
    //                     property.clone(),
    //                     serde_json::Value::String(str_value.to_string()),
    //                 );
    //             } else {
    //                 unimplemented!();
    //             }
    //         }
    //         config
    //             .templates
    //             .get_mut(template_path)?
    //             .outputs
    //             .insert(output_path.to_string(), output);
    //     }
    //     Some(config)
    // }
}

fn is_modified(repo: &git2::Repository, output_path: &str) -> anyhow::Result<bool> {
    match repo.status_file(Path::new(output_path)) {
        Ok(status) => Ok(status.is_wt_modified()),
        Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(false),
        Err(e) => Err(e).with_context(|| format!("git file status failed for {output_path}")),
    }
}

fn generate_file(
    tera: &tera::Tera,
    context: &tera::Context,
    template_path: &str,
) -> anyhow::Result<String> {
    tera.render(template_path, context)
        .context("tera render error")
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
        .arg(arg!(-d - -dump))
        .get_matches();

    let force = matches.is_present("force");
    let stdout = matches.is_present("stdout");
    let fmt_output = !matches.is_present("nofmt");
    let output_path_glob = matches.value_of("GLOB");
    let check = matches.is_present("check");
    let verbose = matches.is_present("verbose");
    // let dump: bool = matches.is_present("dump");

    if stdout && output_path_glob.is_none() {
        // TODO: What if the glob matches multiple files?
        bail!("specify a single file to output to stdout.");
    }

    let glob = if let Some(output_path_glob) = output_path_glob {
        Some(
            globset::Glob::new(output_path_glob)
                .context("failed to compile glob")?
                .compile_matcher(),
        )
    } else {
        None
    };

    let mut tera = tera::Tera::new("templates/**/*.rs.tera").context("tera parsing error(s)")?;
    tera.register_filter(
        "snake_case",
        |value: &tera::Value, _: &_| -> tera::Result<tera::Value> {
            let input = from_value::<String>(value.clone())?;
            let mut iter = input.chars();

            let mut string = String::new();

            if let Some(first) = iter.next() {
                string.push(first.to_ascii_lowercase());

                for char in iter {
                    if char.is_uppercase() {
                        string.push('_');
                    }
                    string.push(char.to_ascii_lowercase());
                }
            }
            tera::Result::Ok(to_value(string)?)
        },
    );

    let config = Config::from_file(Path::new(GLAM_ROOT).join(CONFIG_FILE))?;
    let output_pairs = config.build_output_pairs()?;
    // if dump {
    //     let config = Config::from(output_pairs);
    //     // for output_pair in output_pairs.into_iter() {
    //     //     config.outputs.insert(output_pair.0.to_string(), output_pair.1.into_json());
    //     // }
    //     let json_string = serde_json::to_string_pretty(&config)?;
    //     println!("{}", json_string);
    //     return Ok(());
    // }

    let repo = git2::Repository::open(GLAM_ROOT).context("failed to open git repo")?;
    let workdir = repo.workdir().unwrap();

    let mut output_paths = vec![];
    if let Some(glob) = glob {
        for k in output_pairs.keys() {
            if glob.is_match(k) {
                output_paths.push(k);
            }
        }
        if output_paths.is_empty() {
            bail!("no matching output paths found for '{}'.", glob.glob());
        };
    } else {
        for k in output_pairs.keys() {
            output_paths.push(k);
        }
    };

    output_paths.sort();

    let mut output_differences = 0;
    for output_path in output_paths {
        if !check {
            println!("generating {output_path}");
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
            output_str = rustfmt(&output_str).context("rustfmt failed")?;
        }

        let full_output_path = workdir.join(output_path);

        let output_dir = full_output_path.parent().unwrap();
        std::fs::create_dir_all(output_dir)?;

        if check {
            match std::fs::read_to_string(&full_output_path) {
                Ok(original_str) => {
                    if output_str != original_str {
                        println!("'{output_path}' is different");
                        output_differences += 1;
                    } else if verbose {
                        println!("'{output_path}' is the same");
                    }
                }
                Err(e) => {
                    println!("{output_path} could not be opened or read: {e}");
                    output_differences += 1;
                }
            };
            continue;
        }

        if stdout {
            print!("{output_str}");
            continue;
        }

        std::fs::write(&full_output_path, output_str)
            .with_context(|| format!("failed to write {}", full_output_path.display()))?;
    }

    if check && output_differences > 0 {
        bail!("{output_differences} files were different");
    }

    Ok(())
}
