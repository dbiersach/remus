#!/usr/bin/env bash
# Build the separate native x86-64 Linux engines sequentially.
set -euo pipefail
missing_only=false
case ${1:-} in
    --missing-only) missing_only=true; shift ;;
    --help|-h) printf '%s\n' 'Usage: bash utils/linux/build_linux_engines.sh [--missing-only]'; exit 0 ;;
esac
(($# == 0)) || { printf '%s\n' 'Unknown argument. Use --help.' >&2; exit 1; }
[[ $(uname -s) == Linux && $(uname -m) == x86_64 ]] || { printf '%s\n' 'Build on native x86-64 Linux.' >&2; exit 1; }
script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
workshop_root=$(cd -- "$script_dir/../.." && pwd -P)
verify_engine() {
    local output
    output=$(printf 'uci\nisready\nquit\n' | timeout 20s "$program")
    for expected in "id name Remus$number" uciok readyok; do
        grep -Fxq -- "$expected" <<< "$output" || { printf 'Missing %s from %s\n' "$expected" "$program" >&2; exit 1; }
    done
    printf 'Verified Remus%s identity, uciok, and readyok (no game).\n' "$number"
}
for number in 5 6 7 8; do
    destination=$workshop_root/rust_engines/linux/Remus$number
    program=$destination/remus$number
    if $missing_only && [[ -f $program && ! -L $program ]] && [[ $(head -c 4 -- "$program") == $'\x7fELF' ]]; then
        chmod +x -- "$program"
        verify_engine
        printf 'Using %s\n' "$program"
        continue
    fi
    command -v cargo >/dev/null || { printf '%s\n' 'Install Rust with rustup and the Debian build-essential package to build native engines.' >&2; exit 1; }
    printf 'Building native Remus%s\n' "$number"
    (
        cd -- "$destination"
        cargo build --locked --release --target x86_64-unknown-linux-gnu --target-dir "$workshop_root/rust_engines/linux/target/remus$number"
    )
    mkdir -p -- "$destination"
    install -m 755 -- "$workshop_root/rust_engines/linux/target/remus$number/x86_64-unknown-linux-gnu/release/remus$number" "$program"
    [[ $(head -c 4 -- "$program") == $'\x7fELF' ]] || { printf 'Not an ELF executable: %s\n' "$program" >&2; exit 1; }
    verify_engine
    printf 'Saved %s\n' "$program"
done
