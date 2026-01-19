#!/bin/bash
set -e

echo "🔧 Setting up OpenSecKit development environment..."

# Python virtual environment for frontmatter validation
if [ ! -d ".venv" ]; then
    echo "🐍 Creating Python virtual environment..."
    python3 -m venv .venv
    source .venv/bin/activate
    pip install -r config/requirements-dev.txt
else
    echo "✅ Python virtual environment already exists"
    echo "   Activate with: source .venv/bin/activate"
fi

chmod +x scripts/*.sh scripts/*.py 2>/dev/null || true

# Install pre-commit hook
if [ -d ".git" ]; then
    echo "🪝 Installing Git pre-commit hook..."
    cp scripts/pre-commit .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    echo "✅ Pre-commit hook installed"
else
    echo "⚠️  Not a git repository - skipping pre-commit hook installation"
fi

echo ""
echo "✅ Development environment ready!"
echo ""
echo "📝 Usage:"
echo "  cd cli && cargo test       # Run Rust tests"
echo "  cd cli && cargo clippy     # Lint Rust code"
echo "  cd cli && cargo fmt        # Format Rust code"
echo ""
echo "🐍 Python commands (activate venv first):"
echo "  source .venv/bin/activate"
echo "  python3 scripts/validate-frontmatter.py"
