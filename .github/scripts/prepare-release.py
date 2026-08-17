#!/usr/bin/env python3
"""Stamp the release version into CHANGELOG.md on the release-plz PR.

glam's changelog is curated by hand in the `## [Unreleased]` section. This
script performs the mechanical parts of the release:

* inserts a `## [<new>] - <date>` header between `## [Unreleased]` and the
  curated entries (so the entries end up under the new version),
* rewrites the `[Unreleased]:` compare link to `<new>...HEAD`,
* inserts the `[<new>]: <prev>...<new>` compare link.

The script is idempotent: if it finds a version header / compare link it
previously inserted (i.e. one that doesn't match the latest released
version), it replaces it instead of inserting a duplicate. This handles
release-plz updating the PR with a different version bump.

Usage: prepare-release.py <new-version> <prev-version>

The repo URL is read from the REPO_URL environment variable, e.g.
"https://github.com/bitshifter/glam-rs".
"""

import datetime
import os
import re
import sys

CHANGELOG = "CHANGELOG.md"

HEADER_RE = re.compile(r"^## \[([^\]]+)\]")
UNRELEASED_HEADER = "## [Unreleased]"
UNRELEASED_REF_RE = re.compile(r"^\[Unreleased\]: \S+$")
REF_RE = re.compile(r"^\[([^\]]+)\]: \S+$")


def fail(message):
    sys.exit(f"error: {message}")


def main():
    if len(sys.argv) != 3:
        fail("usage: prepare-release.py <new-version> <prev-version>")
    new_ver, prev_ver = sys.argv[1], sys.argv[2]
    repo_url = os.environ.get("REPO_URL")
    if not repo_url:
        fail("REPO_URL environment variable is not set")
    if new_ver == prev_ver:
        fail(f"new version {new_ver} matches latest release tag {prev_ver}")

    today = datetime.date.today().isoformat()
    new_header = f"## [{new_ver}] - {today}\n"
    new_ref = f"[{new_ver}]: {repo_url}/compare/{prev_ver}...{new_ver}\n"

    with open(CHANGELOG) as f:
        lines = f.readlines()

    out = []
    i = 0
    header_done = False
    ref_done = False
    while i < len(lines):
        line = lines[i]
        i += 1

        if not header_done and line.rstrip("\n") == UNRELEASED_HEADER:
            out.append(line)
            # Keep the blank lines separating sections.
            while i < len(lines) and lines[i].strip() == "":
                out.append(lines[i])
                i += 1
            if i >= len(lines):
                fail("unexpected end of file after '## [Unreleased]'")
            m = HEADER_RE.match(lines[i])
            if m and m.group(1)[0].isdigit() and m.group(1) != prev_ver:
                # A release header we inserted previously (stale version or
                # date): replace it, keeping everything below it as is.
                out.append(new_header)
                i += 1
                # Drop the blank lines after the old header; re-add one.
                while i < len(lines) and lines[i].strip() == "":
                    i += 1
                out.append("\n")
            else:
                # Curated entries or the previous release's header: insert
                # the new version header in front of them.
                out.append(new_header)
                out.append("\n")
            header_done = True
            continue

        if not ref_done and UNRELEASED_REF_RE.match(line):
            out.append(f"[Unreleased]: {repo_url}/compare/{new_ver}...HEAD\n")
            if i >= len(lines):
                fail("unexpected end of file after '[Unreleased]:' link")
            m = REF_RE.match(lines[i])
            if m and m.group(1)[0].isdigit() and m.group(1) != prev_ver:
                # A compare link we inserted previously: replace it.
                out.append(new_ref)
                i += 1
            else:
                out.append(new_ref)
            ref_done = True
            continue

        out.append(line)

    if not header_done:
        fail(f"could not find '{UNRELEASED_HEADER}' in {CHANGELOG}")
    if not ref_done:
        fail(f"could not find '[Unreleased]:' compare link in {CHANGELOG}")

    with open(CHANGELOG, "w") as f:
        f.writelines(out)


if __name__ == "__main__":
    main()
