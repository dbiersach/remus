$ErrorActionPreference = 'Stop'
& uv run --project (Join-Path $PSScriptRoot '..') (Join-Path $PSScriptRoot 'install_lucas_engines.py') @args
exit $LASTEXITCODE
