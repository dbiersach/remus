#!/usr/bin/env bash
set -euo pipefail
engine_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
cd -- "$engine_dir"
exec "$engine_dir/../../.venv/bin/python" "$engine_dir/uci.py" "$@"
