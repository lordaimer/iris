#!/bin/sh

HOOK_DIR=".git/hooks"
SCRIPT_DIR="scripts/hooks"

FORCE=false

for arg in "$@"; do
    case $arg in
        -f|--force)
            FORCE=true
            ;;
    esac
done

echo "Installing git hooks..."

if [ ! -d "$HOOK_DIR" ]; then
    echo "Error: .git directory not found. Are you in the project root?"
    exit 1
fi

if [ -e "$HOOK_DIR/pre-push" ] && [ "$FORCE" = "false" ]; then
    echo "pre-push hook already exists. Use -f or --force to overwrite."
    echo "Skipping installation."
else
    cp "$SCRIPT_DIR/pre-push" "$HOOK_DIR/pre-push"
    chmod +x "$HOOK_DIR/pre-push"
    echo "pre-push hook installed successfully."
fi
