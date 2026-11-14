$ErrorActionPreference = 'Stop'
$env:NODE_NO_WARNINGS = "1"
$installed = @(& code --list-extensions)
if ($LASTEXITCODE -ne 0) { throw "Unable to list VS Code extensions." }
& code --install-extension anthropic.claude-code --force
if ($LASTEXITCODE -ne 0) { throw "VS Code extension installation failed." }
& code --install-extension charliermarsh.ruff --force
if ($LASTEXITCODE -ne 0) { throw "VS Code extension installation failed." }
& code --install-extension davidanson.vscode-markdownlint --force
if ($LASTEXITCODE -ne 0) { throw "VS Code extension installation failed." }
& code --install-extension ms-python.debugpy --force
if ($LASTEXITCODE -ne 0) { throw "VS Code extension installation failed." }
if ($installed -contains 'ms-python.isort') {
    & code --uninstall-extension ms-python.isort --force
    if ($LASTEXITCODE -ne 0) { throw "VS Code extension removal failed." }
}
& code --install-extension ms-python.python --force
if ($LASTEXITCODE -ne 0) { throw "VS Code extension installation failed." }
& code --install-extension ms-python.vscode-pylance --force
if ($LASTEXITCODE -ne 0) { throw "VS Code extension installation failed." }
if ($installed -contains 'ms-python.vscode-python-envs') {
    & code --uninstall-extension ms-python.vscode-python-envs --force
    if ($LASTEXITCODE -ne 0) { throw "VS Code extension removal failed." }
}
& code --install-extension oderwat.indent-rainbow --force
if ($LASTEXITCODE -ne 0) { throw "VS Code extension installation failed." }
& code --install-extension streetsidesoftware.code-spell-checker --force
if ($LASTEXITCODE -ne 0) { throw "VS Code extension installation failed." }
& code --install-extension tomoki1207.pdf --force
if ($LASTEXITCODE -ne 0) { throw "VS Code extension installation failed." }
& code --install-extension usernamehw.errorlens --force
if ($LASTEXITCODE -ne 0) { throw "VS Code extension installation failed." }
