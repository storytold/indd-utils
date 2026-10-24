---
name: indd-to-idml
description: Convert Adobe InDesign (.indd, .indt) files to IDML (.idml) without InDesign using storytold/indd-utils on macOS, Linux, and Windows. Checks whether indd and its dependencies (git, rustup, rustc >= 1.95 per Cargo.toml) are already installed before building or converting. Use when asked to convert .indd or .indt files to .idml, inspect InDesign files, or run indd-utils.
---

# Converting INDD and INDT Files to IDML (`indd-utils`)

Converts native Adobe InDesign documents (`.indd`) and templates (`.indt`) to
IDML packages (`.idml`) without InDesign using
[`storytold/indd-utils`](https://github.com/storytold/indd-utils).

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
                       |        |    - Ask user before installing tools |
                       |        |    - Install `indd` to ~/.cargo/bin   |
                       |        +-----------------+---------------------+
                       |                          |
                       +------------+-------------+
                                    |
                                    v
+-----------------------------------------------------------------------+
| 3. Convert & Verify                                                   |
|    - `indd info <in.indd>`                                            |
|    - `indd convert <in.indd> <out.idml>`                              |
|    - Test IDML zip integrity (`designmap.xml`, `Spreads/`, `Stories/`)|
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

## Step 2: Check Dependencies and Install `indd`

Building `indd-utils` from source requires:
- `git`
- Rust `>= 1.95.0` (`rust-version = "1.95"` in [`Cargo.toml`](../../Cargo.toml),
  required for `let_chains` stabilized in 1.88 and `if_let_guard` stabilized in
  1.95).

**Ask the user before installing:** Never install or update a toolchain
(`rustup`, MSVC Build Tools) or system package (`apt`, `winget`) without
confirmation. Show the user the exact commands that will run and wait for
approval before executing them.

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
4. Install `indd` into `~/.cargo/bin/indd`:
   ```bash
   env RUSTC="$HOME/.cargo/bin/rustc" "$HOME/.cargo/bin/cargo" install --git https://github.com/storytold/indd-utils.git
   ```
   *(If already inside an `indd-utils` checkout, use `--path .` instead of `--git ...`.)*

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
4. Install `indd` into `~/.cargo/bin/indd`:
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
4. Install `indd.exe` into `%USERPROFILE%\.cargo\bin\indd.exe`:
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

2. Convert the file to IDML:
   ```bash
   <INDD_BIN> convert "/path/to/input.indd" "/path/to/output.idml"
   ```
   - If the user does not give an output path, write to the same directory and
     basename with an `.idml` extension.
   - Non-fatal conversion issues print to `stderr` as `warning: ...`.

---

## Step 4: Verify and Report Output

1. Test the output zip archive integrity (`mimetype`, `designmap.xml`,
   `Spreads/`, `Stories/`, `MasterSpreads/`, `Resources/`):
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

2. If warnings were printed or the user asks what was skipped, run `indd audit`:
   ```bash
   <INDD_BIN> audit "/path/to/input.indd"
   ```

3. Report the converted file location clearly to the user:
   - Show the full path to the saved `.idml` file (using a clickable
     `[filename.idml](file:///absolute/path/to/filename.idml)` link when
     supported) along with its file size.
   - Include the source InDesign version from `indd info` and note whether the
     conversion finished cleanly or emitted any warnings.
