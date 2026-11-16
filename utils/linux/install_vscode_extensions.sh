#!/usr/bin/env bash
# Install the workshop extensions for the current user's VS Code profile.
# Optional code CLI arguments support isolated preparation checks, for example:
# bash utils/linux/install_vscode_extensions.sh --extensions-dir /tmp/remus-extensions
set -euo pipefail

export NODE_NO_WARNINGS=1
command -v code >/dev/null || { printf '%s\n' 'VS Code command "code" is missing from PATH.' >&2; exit 1; }
installed=$(code "$@" --list-extensions)

install_extension() {
    code "$@" --install-extension "$extension" --force
}

remove_extension_if_present() {
    if grep -Fxiq -- "$extension" <<< "$installed"; then
        code "$@" --uninstall-extension "$extension" --force
    fi
}

for extension in anthropic.claude-code charliermarsh.ruff davidanson.vscode-markdownlint ms-python.debugpy; do
    install_extension "$@"
done
extension=ms-python.isort
remove_extension_if_present "$@"
for extension in ms-python.python ms-python.vscode-pylance; do
    install_extension "$@"
done
installed=$(code "$@" --list-extensions)
extension=ms-python.vscode-python-envs
remove_extension_if_present "$@"
for extension in oderwat.indent-rainbow streetsidesoftware.code-spell-checker tomoki1207.pdf usernamehw.errorlens; do
    install_extension "$@"
done
