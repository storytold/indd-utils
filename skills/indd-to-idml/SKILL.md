---
name: indd-to-idml
description: Convert Adobe InDesign (.indd, .indt) files to IDML (.idml) without InDesign using storytold/indd-utils on macOS, Linux, and Windows. Checks whether indd and its dependencies (git, rustup, rustc >= 1.95 per Cargo.toml) are already installed before building or converting, asks before fetching URLs or writing outside the working directory, and reports converter limitations and unread objects. Use when asked to convert .indd or .indt files to .idml, inspect InDesign files, or run indd-utils.
---

# Converting INDD and INDT Files to IDML (`indd-utils`)

Converts native Adobe InDesign documents (`.indd`) and templates (`.indt`) to
IDML packages (`.idml`) without InDesign using
[`storytold/indd-utils`](https://github.com/storytold/indd-utils).

## Scope, Fidelity, and Filesystem Rules

- **Clean-room reverse-engineered reader (no fidelity guarantee):** INDD is an
  undocumented proprietary format. Every fact `indd-utils` relies on was found
  by comparing sample INDD files with IDML exports
  ([`CLEANROOM.md`](../../CLEANROOM.md), [`docs/format/`](../../docs/format/)).
  The output is **not** identical to InDesign's own IDML export; never promise
  complete fidelity, even when a conversion emits zero warnings.
- **Supported versions and known gaps** (see [`README.md`](../../README.md)
  [Supported versions](../../README.md#supported-versions) and
  [What does not convert](../../README.md#what-does-not-convert)):
  - **InDesign 1.x:** Not supported (different container).
  - **InDesign 3.0–7.5:** Converts, with document preferences (3.0–7.5), object
    style text frame settings (3.0–4.0), and the page number style (3.0) omitted
    with warnings.
  - **InDesign 8.0–21.6:** Converts, except tables from InDesign 9.2 are not
    read.
  - **Unconverted across versions:** Conditional text, QR codes, flex objects,
    transparency of placed graphics, most non-gradient transparency effects
    (bevel, satin, directional/basic feather, drop shadow offsets), hyperlink
    sources spanning paragraphs or holding anchored objects, XML
    attributes/comments/PIs/DTDs, and attributes whose value does not vary
    across public samples.
- **Commands, network fetches, and writing outside the working directory:**
  - Ask the user for confirmation and show the exact commands before fetching any
    URL (`curl https://sh.rustup.rs`, `cargo install --git ...`), installing or
    updating any toolchain or system package (`rustup`, MSVC Build Tools, `apt`,
    `winget`), or installing a binary into `~/.cargo/bin`
    (`%USERPROFILE%\.cargo\bin`).
  - During conversion, only read the user's input `.indd`/`.indt` file and write
    the single requested `.idml` output file. Never overwrite an existing file
    or write to another directory without asking first.

```text
+-----------------------------------------------------------------------+
| 1. Locate `indd` binary (PATH -> ~/.cargo/bin -> ./target/release)    |
+-----------------------------------+-----------------------------------+
                                    |
                     Found? --------+-------- Not Found?
                       |                          |
                       v                          v
                       |        +---------------------------------------+
                       |        | 2. Check Dependencies (OS-specific)   |
                       |        |    - Check git & rustc >= 1.95.0      |
                       |        |    - Ask user before fetching/writing |
                       |        |    - Build or install `indd`          |
                       |        +-----------------+---------------------+
                       |                          |
                       +------------+-------------+
                                    |
                                    v
+-----------------------------------------------------------------------+
| 3. Convert, Audit & Report                                            |
|    - `indd info <in.indd>`                                            |
|    - `indd convert <in.indd> <out.idml>`                              |
|    - Test IDML zip integrity (`unzip -t` / `ZipFile::OpenRead`)       |
|    - `indd audit <in.indd>` (report unread objects & converter limits)|
+-----------------------------------------------------------------------+
```

## Step 1: Check for an Existing `indd` Binary

Check whether `indd` is already built before checking toolchains or compiling
from source.

### macOS and Linux

Check in order:

1. `which indd`
2. `$HOME/.cargo/bin/indd`
3. `./target/release/indd` (if running inside an `indd-utils` checkout)

```bash
which indd || ls -la "$HOME/.cargo/bin/indd" "./target/release/indd"
```

### Windows (PowerShell)

Check in order:

1. `Get-Command indd -ErrorAction SilentlyContinue`
2. `$env:USERPROFILE\.cargo\bin\indd.exe`
3. `.\target\release\indd.exe` (if running inside an `indd-utils` checkout)

```powershell
Test-Path "$env:USERPROFILE\.cargo\bin\indd.exe", ".\target\release\indd.exe"
```

If `indd` exists in any of these locations, skip Step 2 and go to Step 3.

---

## Step 2: Check Dependencies and Build or Install `indd`

Building `indd-utils` from source requires:
- `git`
- Rust `>= 1.95.0` (`rust-version = "1.95"` in [`Cargo.toml`](../../Cargo.toml),
  required for `let_chains` stabilized in 1.88 and `if_let_guard` stabilized in
  1.95). The crate has zero external dependencies in `Cargo.toml`.

**Ask the user before fetching, installing, or writing outside the working directory:**
Never fetch from a remote URL (`https://sh.rustup.rs`,
`https://github.com/storytold/indd-utils.git`), install or update a toolchain
(`rustup`, MSVC Build Tools) or system package (`apt`, `winget`), or write a
binary into `~/.cargo/bin` (`%USERPROFILE%\.cargo\bin`) without confirmation.
Show the exact commands that will run and wait for approval before executing
them. If already inside an `indd-utils` checkout, prefer building in place with
`cargo build --release` (writes `./target/release/indd` inside the working
directory with no network access) unless the user wants `indd` installed to
`~/.cargo/bin`.

**Compiler PATH shadowing on macOS and Linux:** Homebrew
(`/opt/homebrew/bin/rustc`) and system package managers (`/usr/bin/rustc`)
often place an older `rustc` ahead of `~/.cargo/bin/rustc` in `PATH`. Check
`~/.cargo/bin/rustc --version` directly and pass
`RUSTC="$HOME/.cargo/bin/rustc"` when running `cargo`.

### macOS

1. Check `git` and `rustc`:
   ```bash
   git --version
   "$HOME/.cargo/bin/rustc" --version
   ```
2. If `rustup` is not installed, show the command and ask the user before running:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
   ```
3. If `rustc` is older than `1.95.0`, show the command and ask the user before updating `stable`:
   ```bash
   "$HOME/.cargo/bin/rustup" update stable
   ```
4. Build or install `indd` (show the command and ask the user before fetching from GitHub or writing to `~/.cargo/bin`):
   - Inside an `indd-utils` checkout (writes only to `./target/release/indd`):
     ```bash
     env RUSTC="$HOME/.cargo/bin/rustc" "$HOME/.cargo/bin/cargo" build --release
     ```
   - Otherwise, install from GitHub into `~/.cargo/bin/indd`:
     ```bash
     env RUSTC="$HOME/.cargo/bin/rustc" "$HOME/.cargo/bin/cargo" install --git https://github.com/storytold/indd-utils.git
     ```

### Linux

1. Check `git`, C linker (`cc`), and `rustc`:
   ```bash
   git --version
   cc --version
   "$HOME/.cargo/bin/rustc" --version
   ```
   *(On Debian/Ubuntu, if `git` or `cc` is missing, show `sudo apt-get update && sudo apt-get install -y git build-essential curl` and ask the user before running.)*
2. If `rustup` is not installed, show the command and ask the user before running:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
   ```
3. If `rustc` is older than `1.95.0`, show the command and ask the user before updating `stable`:
   ```bash
   "$HOME/.cargo/bin/rustup" update stable
   ```
4. Build or install `indd` (show the command and ask the user before fetching from GitHub or writing to `~/.cargo/bin`):
   - Inside an `indd-utils` checkout (writes only to `./target/release/indd`):
     ```bash
     env RUSTC="$HOME/.cargo/bin/rustc" "$HOME/.cargo/bin/cargo" build --release
     ```
   - Otherwise, install from GitHub into `~/.cargo/bin/indd`:
     ```bash
     env RUSTC="$HOME/.cargo/bin/rustc" "$HOME/.cargo/bin/cargo" install --git https://github.com/storytold/indd-utils.git
     ```

### Windows (PowerShell)

1. Check `git` and `rustc`:
   ```powershell
   git --version
   & "$env:USERPROFILE\.cargo\bin\rustc.exe" --version
   ```
2. If `git`, `rustup`, or the MSVC linker is missing, show the required `winget` commands and ask the user before running:
   ```powershell
   winget install --id Git.Git -e --source winget
   winget install --id Rustlang.Rustup -e --source winget
   winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
   ```
3. If `rustc` is older than `1.95.0`, show the command and ask the user before updating `stable`:
   ```powershell
   & "$env:USERPROFILE\.cargo\bin\rustup.exe" update stable
   ```
4. Build or install `indd.exe` (show the command and ask the user before fetching from GitHub or writing to `%USERPROFILE%\.cargo\bin`):
   - Inside an `indd-utils` checkout (writes only to `.\target\release\indd.exe`):
     ```powershell
     $env:RUSTC = "$env:USERPROFILE\.cargo\bin\rustc.exe"
     & "$env:USERPROFILE\.cargo\bin\cargo.exe" build --release
     ```
   - Otherwise, install from GitHub into `%USERPROFILE%\.cargo\bin\indd.exe`:
     ```powershell
     $env:RUSTC = "$env:USERPROFILE\.cargo\bin\rustc.exe"
     & "$env:USERPROFILE\.cargo\bin\cargo.exe" install --git https://github.com/storytold/indd-utils.git
     ```

---

## Step 3: Convert INDD or INDT to IDML

Using the resolved binary path (`<INDD_BIN>`):

1. Inspect the file header (InDesign version, byte order, database pages, and
   object count):
   ```bash
   <INDD_BIN> info "/path/to/input.indd"
   ```
   *(If `indd info` reports InDesign 1.x, stop and report that 1.x uses a
   different container format that `indd-utils` does not support.)*

2. Convert the file to IDML:
   ```bash
   <INDD_BIN> convert "/path/to/input.indd" "/path/to/output.idml"
   ```
   - If the user does not give an output path, default to the same directory and
     basename as the input file with an `.idml` extension. Ask the user before
     overwriting an existing `.idml` file or writing to any other directory.
   - Non-fatal conversion issues print to `stderr` as `warning: ...`.

---

## Step 4: Verify Archive Integrity, Audit Unread Data, and Report

1. Test the output zip archive integrity (`mimetype`, `designmap.xml`,
   `Spreads/`, `Stories/`, `MasterSpreads/`, `Resources/`). Note that this
   checks only that the `.idml` zip container is intact, not that every
   InDesign feature was converted:
   - macOS and Linux:
     ```bash
     unzip -t "/path/to/output.idml"
     ```
   - Windows (PowerShell — decompresses every entry stream to verify CRC-32 integrity):
     ```powershell
     Add-Type -AssemblyName System.IO.Compression.FileSystem
     $z = [IO.Compression.ZipFile]::OpenRead("C:\path\to\output.idml")
     try { foreach ($e in $z.Entries) { $s = $e.Open(); $s.CopyTo([IO.Stream]::Null); $s.Close() } } finally { $z.Dispose() }
     ```

2. Audit unread objects and attributes with `indd audit`:
   ```bash
   <INDD_BIN> audit "/path/to/input.indd"
   ```
   `indd audit` lists database classes, chunks, attribute IDs, and codes in the
   document that the converter does not read (including items that do not emit
   `stderr` warnings during `indd convert`).

3. Report the result accurately to the user:
   - Show the full path to the saved `.idml` file (using a clickable
     `[filename.idml](file:///absolute/path/to/filename.idml)` link when
     supported) along with its file size.
   - Include the source InDesign version from `indd info`, any `stderr` warnings
     from `indd convert`, and a summary of unread items from `indd audit`.
   - Do not promise 100% layout or formatting fidelity; mention any relevant
     version or feature caveats from [Scope, Fidelity, and Filesystem Rules](#scope-fidelity-and-filesystem-rules).
