#!/usr/bin/env bash
# Prepare a disposable student clone on Debian-based x86-64 Linux.
set -euo pipefail
shopt -s nullglob dotglob

fail() { printf '%s\n' "$*" >&2; exit 1; }
usage() {
    cat <<'HELP'
Usage: bash utils/linux/setup_student.sh --student-copy --lucas-dir PATH [options]

Run as the classroom user, in a fresh clone, with Lucas Chess closed.
  --student-copy           Authorize removal of solutions and private material.
  --dry-run                Preview preparation and cleanup without making changes.
  --lucas-dir PATH         Lucas installation containing bin/LucasR.
  --lucas-config-dir PATH  Initialized __Config__ folder, for custom user data.
  --extensions-dir PATH    Isolated VS Code extension directory for testing.
  --user-data-dir PATH     Isolated VS Code user-data directory for testing.
  --help                   Show this help.

Requires Bash, uv, VS Code's code command, and procps (pgrep).
Missing native engines are built sequentially with Cargo and a C toolchain.
Install Rust with rustup and Debian's build-essential package before setup.
Native engines live in rust_engines/linux/RemusN/remusN (N = 5 through 8).
Windows executables are not used. No games are started.
HELP
}

student_copy=false
dry_run=false
lucas_dir=
lucas_config_dir=
code_arguments=()
while (($#)); do
    case "$1" in
        --student-copy) student_copy=true; shift ;;
        --dry-run) dry_run=true; shift ;;
        --help|-h) usage; exit 0 ;;
        --lucas-dir|--lucas-config-dir|--extensions-dir|--user-data-dir)
            (($# >= 2)) && [[ -n $2 ]] || fail "Missing value for $1"
            case "$1" in
                --lucas-dir) lucas_dir=$2 ;;
                --lucas-config-dir) lucas_config_dir=$2 ;;
                *) code_arguments+=("$1" "$2") ;;
            esac
            shift 2 ;;
        *) fail "Unknown option: $1. Use --help." ;;
    esac
done

$student_copy || fail 'Use --student-copy only in a fresh student clone. Add --dry-run to preview.'
[[ ! -L ${BASH_SOURCE[0]} ]] || fail 'Run the actual setup script, not a symbolic link.'
script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
workshop_root=$(cd -- "$script_dir/../.." && pwd -P)
[[ $script_dir == "$workshop_root/utils/linux" ]] || fail 'The utils/linux folder must not be a symbolic link.'
[[ ! -e $workshop_root/admin/workshop_paths.py ]] || fail 'This is the protected designer workspace. Prepare a separate student clone.'
if [[ -e $workshop_root/.workshop-student.json ]]; then
    printf '%s\n' 'This student workspace is already prepared. Existing student code is preserved.'
    exit 0
fi

# Resolve every cleanup target and reject links in any ancestor or descendant.
check_child() {
    local relative=$1 path part resolved linked
    [[ -n $relative && $relative != /* ]] || fail "Invalid workshop path: $relative"
    path=$workshop_root
    local -a parts
    IFS=/ read -r -a parts <<< "$relative"
    for part in "${parts[@]}"; do
        [[ -n $part && $part != . && $part != .. ]] || fail "Invalid workshop path: $relative"
        path=$path/$part
        [[ ! -L $path ]] || fail "Cleanup refuses symbolic links: $path"
    done
    resolved=$(realpath -m -- "$path")
    [[ $resolved == "$workshop_root/"* ]] || fail "Path leaves the workshop clone: $path"
    if [[ -d $path ]]; then
        linked=$(find "$path" -type l -print -quit)
        [[ -z $linked ]] || fail "Cleanup refuses a folder containing symbolic links: $path"
    fi
}

for required in pyproject.toml uv.lock .python-version Remus.code-workspace utils/linux/install_vscode_extensions.sh utils/linux/install_lucas_engines.py utils/linux/build_linux_engines.sh; do
    check_child "$required"
    [[ -f $workshop_root/$required ]] || fail "Not a complete workshop clone: missing $required"
done
check_child utils/linux/setup_student.sh
check_child .workshop-student.json
check_child rust_engines
engine_folders=()
cleanup_paths=()
for number in 1 2 3 4; do
    days=("$workshop_root/Day $number - "*)
    ((${#days[@]} == 1)) && [[ -d ${days[0]} ]] || fail "Expected one Day $number folder."
    relative=${days[0]#"$workshop_root/"}/Remus$number
    check_child "$relative"
    folder=$workshop_root/$relative
    for name in "Remus$number.md" "remus$number.bat" "remus$number.sh" "verify_remus$number.py"; do
        [[ -f $folder/$name ]] || fail "Missing supplied file: $folder/$name"
    done
    if ((number >= 2)); then
        [[ -f $folder/piece_square_tables.json ]] || fail "Missing supplied evaluation tables: $folder"
    fi
    if ((number >= 3)); then
        [[ -f $folder/opening_book.txt ]] || fail "Missing supplied opening book: $folder"
    fi
    engine_folders+=("$folder")
    for file in "$folder/"*.py; do
        [[ ${file##*/} == "verify_remus$number.py" ]] || cleanup_paths+=("$file")
    done
    [[ ! -e $folder/__pycache__ ]] || cleanup_paths+=("$folder/__pycache__")
done
private_names=(.venv-ft .pytest_cache .ruff_cache .git .archive instructor admin engines_student engines_instructor science_instructor tmp .github .githooks .claude .codex .agents AGENTS.md CLAUDE.md .gitignore .gitattributes)
for name in "${private_names[@]}"; do
    check_child "$name"
    [[ ! -e $workshop_root/$name ]] || cleanup_paths+=("$workshop_root/$name")
done
for path in "${cleanup_paths[@]}"; do
    check_child "${path#"$workshop_root/"}"
done

if $dry_run; then
    printf '%s\n' 'Preview: require initialized, closed Lucas Chess; install extensions; run uv sync; build missing native Rust engines; check supplied Linux launchers; register all eight engines; clean up this clone; delete this setup script.'
    for folder in "${engine_folders[@]}"; do
        printf 'Keep the guide, verifier, launchers, and supplied data in %s\n' "$folder"
    done
    for path in "${cleanup_paths[@]}" "$script_dir/setup_student.sh"; do
        printf 'Would remove: %s\n' "$path"
    done
    exit 0
fi

[[ $(uname -s) == Linux && $(uname -m) == x86_64 ]] || fail 'Run setup on x86-64 Linux.'
((EUID != 0)) || fail 'Run setup as the classroom user, without sudo.'
for command in uv code pgrep; do
    command -v "$command" >/dev/null || fail "Required command is missing from PATH: $command"
done
[[ -n $lucas_dir ]] || fail 'Specify --lucas-dir with the native Lucas Chess installation folder.'
lucas_dir=$(realpath -e -- "$lucas_dir")
[[ -x $lucas_dir/bin/LucasR ]] || fail 'Install native Lucas Chess, open it once, then close it. Expected bin/LucasR.'
if [[ -z $lucas_config_dir ]]; then
    [[ ! -f $lucas_dir/lc.folder && -z ${LUCASCHESS_HOME:-} ]] || fail 'Custom Lucas user data detected. Specify its __Config__ folder with --lucas-config-dir.'
    lucas_config_dir=$lucas_dir/UserData/__Config__
fi
lucas_config_dir=$(realpath -e -- "$lucas_config_dir")
[[ -d $lucas_config_dir ]] || fail 'Open Lucas Chess once to initialize its configuration, then close it.'
for path in "$lucas_dir" "$lucas_config_dir"; do
    for cleanup in "${cleanup_paths[@]}"; do
        [[ $path != "$cleanup" && $path != "$cleanup/"* ]] || fail "Lucas files must be outside cleanup targets: $path"
    done
done
assert_lucas_closed() {
    local status=0
    pgrep -f '(^|/)(LucasR|LucasR\.py)([[:space:]]|$)' >/dev/null || status=$?
    ((status == 1)) || fail 'Close Lucas Chess before setup (or resolve the process-check error). No student files were removed.'
}
assert_lucas_closed

cd -- "$workshop_root"
bash "$script_dir/install_vscode_extensions.sh" "${code_arguments[@]}"
uv sync
[[ -x $workshop_root/.venv/bin/python ]] || fail 'uv sync did not create .venv/bin/python.'
bash "$script_dir/build_linux_engines.sh" --missing-only
for number in 1 2 3 4; do
    folder=${engine_folders[number-1]}
    check_child "${folder#"$workshop_root/"}/remus$number.sh"
    chmod +x -- "$folder/remus$number.sh"
done
assert_lucas_closed
"$workshop_root/.venv/bin/python" "$script_dir/install_lucas_engines.py" --lucas-dir "$lucas_dir" --config-dir "$lucas_config_dir"

# Check the entire removal list again before the first deletion.
for path in "${cleanup_paths[@]}" "$script_dir/setup_student.sh"; do
    check_child "${path#"$workshop_root/"}"
done
for path in "${cleanup_paths[@]}"; do
    rm -rf -- "$path"
done
printf '%s\n' '{"prepared": true, "layout": 1}' > "$workshop_root/.workshop-student.json"
rm -- "$script_dir/setup_student.sh"
printf '%s\n' 'Workshop setup complete. Remus1 through Remus8 are registered in Lucas Chess. Python folders retain guides, verifiers, launchers, and supplied data. Type the engine modules from the period screenshots.'
