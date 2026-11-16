#!/usr/bin/env bash
set -euo pipefail
script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
exec uv run --project "$script_dir/../.." "$script_dir/install_lucas_engines.py" "$@"
