#!/bin/bash
# Installation script for git hooks

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_DIR="$(git rev-parse --git-dir 2>/dev/null)"

if [ -z "$GIT_DIR" ]; then
    echo "Error: Not in a git repository"
    exit 1
fi

echo "Installing git hooks..."

# Copy pre-commit hook
if [ -f "$GIT_DIR/hooks/pre-commit" ]; then
    echo "Warning: pre-commit hook already exists. Creating backup..."
    cp "$GIT_DIR/hooks/pre-commit" "$GIT_DIR/hooks/pre-commit.backup"
fi

cp "$SCRIPT_DIR/pre-commit" "$GIT_DIR/hooks/pre-commit"
chmod +x "$GIT_DIR/hooks/pre-commit"

echo "Git hooks installed successfully!"
echo ""
echo "The pre-commit hook will now:"
echo "  - Check code formatting (cargo fmt)"
echo "  - Run clippy lints (cargo clippy)"
echo ""
echo "To skip hooks temporarily, use: git commit --no-verify"
