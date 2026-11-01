#!/bin/sh
# Convert every INDD and INDT file in a folder to IDML.
#
# Usage: tools/convert_folder.sh <input-folder> <output-folder>
#
# Builds the converter (needs a recent stable Rust), then writes
# <output-folder>/<name>.idml for each <input-folder>/<name>.indd or .indt.
# Warnings of a conversion go to <output-folder>/<name>.warnings.txt (the
# file is removed when there are none). A file that does not convert is
# reported as FAILED, and the script goes on with the next one. It exits
# with 1 if any file failed.
set -eu
if [ $# -ne 2 ]; then
    echo "usage: $0 <input-folder> <output-folder>" >&2
    exit 2
fi
in=$1
out=$2
root=$(cd "$(dirname "$0")/.." && pwd)
(cd "$root" && cargo build --release --quiet)
bin="$root/target/release/indd"
mkdir -p "$out"
converted=0
failed=0
for f in "$in"/*.indd "$in"/*.INDD "$in"/*.indt "$in"/*.INDT; do
    [ -f "$f" ] || continue
    base=$(basename "$f")
    name=${base%.*}
    warnings="$out/$name.warnings.txt"
    if "$bin" convert "$f" "$out/$name.idml" 2>"$warnings"; then
        converted=$((converted + 1))
        if [ -s "$warnings" ]; then
            echo "$base: converted, $(wc -l <"$warnings" | tr -d ' ') warning(s)"
        else
            rm -f "$warnings"
            echo "$base: converted"
        fi
    else
        failed=$((failed + 1))
        rm -f "$out/$name.idml"
        echo "FAILED: $base (see $warnings)"
    fi
done
echo "$converted converted, $failed failed"
[ "$failed" -eq 0 ]
