[CmdletBinding(SupportsShouldProcess)]
param(
    [switch]$StudentCopy
)

$ErrorActionPreference = 'Stop'
$workshopRoot = [IO.Path]::GetFullPath($PSScriptRoot).TrimEnd('\', '/')
$readyFile = Join-Path $workshopRoot '.workshop-student.json'

if (-not $StudentCopy) {
    throw 'Run setup_student.ps1 -StudentCopy only in the cloned student workspace. Add -WhatIf to preview.'
}
if (Test-Path -LiteralPath (Join-Path $workshopRoot 'admin/workshop_paths.py')) {
    throw 'This is the protected designer workspace. Prepare a separate student clone.'
}
if (Test-Path -LiteralPath $readyFile) {
    Write-Host 'This student workspace is already prepared. Existing student code is preserved.'
    return
}
foreach ($requiredFile in @('pyproject.toml', 'uv.lock', '.python-version', 'Remus.code-workspace', 'install_vscode_extensions.ps1')) {
    if (-not (Test-Path -LiteralPath (Join-Path $workshopRoot $requiredFile) -PathType Leaf)) {
        throw "Not a complete workshop clone: missing $requiredFile"
    }
}

function Get-WorkshopChild([string]$RelativePath) {
    $resolved = [IO.Path]::GetFullPath((Join-Path $workshopRoot $RelativePath))
    if (-not $resolved.StartsWith($workshopRoot + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) {
        throw "Path leaves the workshop clone: $RelativePath"
    }
    if (Test-Path -LiteralPath $resolved) {
        $item = Get-Item -LiteralPath $resolved -Force
        if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) {
            throw "Cleanup refuses links and junctions: $RelativePath"
        }
    }
    $ancestor = Split-Path -Parent $resolved
    while ($ancestor -ne $workshopRoot) {
        if (Test-Path -LiteralPath $ancestor) {
            if ((Get-Item -LiteralPath $ancestor -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) {
                throw "Cleanup refuses a linked ancestor: $RelativePath"
            }
        }
        $ancestor = Split-Path -Parent $ancestor
    }
    return $resolved
}

$engineFolders = @()
for ($number = 1; $number -le 4; $number++) {
    $days = @(Get-ChildItem -LiteralPath $workshopRoot -Directory | Where-Object { $_.Name -like "Day $number - *" })
    if ($days.Count -ne 1) { throw "Expected one Day $number folder." }
    $folder = Get-WorkshopChild ($days[0].Name + "\Remus$number")
    foreach ($name in @("Remus$number.md", "remus$number.bat", "verify_remus$number.py")) {
        if (-not (Test-Path -LiteralPath (Join-Path $folder $name) -PathType Leaf)) { throw "Missing supplied file: $name" }
    }
    if ($number -ge 2 -and -not (Test-Path -LiteralPath (Join-Path $folder 'piece_square_tables.json'))) { throw 'Missing supplied evaluation tables.' }
    if ($number -ge 3 -and -not (Test-Path -LiteralPath (Join-Path $folder 'opening_book.txt'))) { throw 'Missing supplied opening book.' }
    $linked = @(Get-ChildItem -LiteralPath $folder -Force -Recurse | Where-Object { $_.Attributes -band [IO.FileAttributes]::ReparsePoint })
    if ($linked.Count) { throw "Cleanup refuses links or junctions in $folder" }
    $engineFolders += $folder
}
foreach ($number in 5..8) {
    if (-not (Test-Path -LiteralPath (Get-WorkshopChild "rust_engines\Remus$number\remus$number.exe") -PathType Leaf)) {
        throw "Missing supplied Remus$number executable."
    }
}

# Resolve and validate every deletion before running external setup commands.
$privateNames = @('.venv-ft', '.pytest_cache', '.ruff_cache', '.git', '.archive', 'instructor', 'admin', 'engines_student', 'engines_instructor', 'science_instructor', 'tmp', '.github', '.githooks', '.claude', '.codex', '.agents', 'AGENTS.md', 'CLAUDE.md', '.gitignore', '.gitattributes')
$cleanupPaths = @()
foreach ($name in $privateNames) {
    $path = Get-WorkshopChild $name
    if (Test-Path -LiteralPath $path) {
        $linked = @(Get-ChildItem -LiteralPath $path -Force -Recurse -ErrorAction Stop | Where-Object { $_.Attributes -band [IO.FileAttributes]::ReparsePoint })
        if ($linked.Count) { throw "Cleanup refuses a folder containing links or junctions: $name" }
        $cleanupPaths += $path
    }
}

if ($WhatIfPreference) {
    Write-Host 'Preview: install VS Code extensions, run uv sync, remove temporary engine modules and private materials, then delete this setup script.'
    foreach ($folder in $engineFolders) { Write-Host "Keep the guide, version verifier, launcher, and supplied data in $folder" }
    foreach ($path in $cleanupPaths) { Write-Host "Would remove: $path" }
    return
}

Get-Command uv -ErrorAction Stop | Out-Null
Get-Command code -ErrorAction Stop | Out-Null
Push-Location -LiteralPath $workshopRoot
try {
    if ($PSCmdlet.ShouldProcess($workshopRoot, 'Prepare this student clone, remove solutions and private material, and delete the setup script on success')) {
        & (Join-Path $workshopRoot 'install_vscode_extensions.ps1')
        & uv sync
        if ($LASTEXITCODE -ne 0) { throw "uv sync failed with exit code $LASTEXITCODE. No student files were removed." }
    }
    else { return }

    foreach ($number in 1..4) {
        $folder = $engineFolders[$number - 1]
        foreach ($file in Get-ChildItem -LiteralPath $folder -File -Filter '*.py') {
            if ($file.Name -eq "verify_remus$number.py") { continue }
            $relative = $file.FullName.Substring($workshopRoot.Length + 1)
            $checked = Get-WorkshopChild $relative
            Remove-Item -LiteralPath $checked -Force
        }
        $cache = Get-WorkshopChild ($folder.Substring($workshopRoot.Length + 1) + '\__pycache__')
        if (Test-Path -LiteralPath $cache) {
            Remove-Item -LiteralPath $cache -Recurse -Force
        }
    }
    foreach ($path in $cleanupPaths) {
        Remove-Item -LiteralPath $path -Recurse -Force
    }
    @{ prepared = $true; layout = 1 } | ConvertTo-Json | Set-Content -LiteralPath $readyFile -Encoding utf8
    Write-Host 'Workshop setup complete. The Python engine folders contain their guides, verifiers, launchers, and supplied data. Type the engine modules from the period screenshots.'
}
finally { Pop-Location }

# This is reached only when every setup step above succeeds.
Remove-Item -LiteralPath (Get-WorkshopChild 'setup_student.ps1') -Force
