#!/bin/bash
set -e

echo "🔧 Setting up OpenSecKit development environment..."

if [ ! -d "node_modules" ]; then
    echo "📦 Installing Node.js dependencies locally..."
    npm install
else
    echo "✅ Node.js dependencies already installed"
fi

if [ ! -d ".venv" ]; then
    echo "🐍 Creating Python virtual environment..."
    python3 -m venv .venv
    source .venv/bin/activate
    pip install -r requirements-dev.txt
else
    echo "✅ Python virtual environment already exists"
    echo "   Activate with: source .venv/bin/activate"
fi

chmod +x scripts/*.sh scripts/*.py 2>/dev/null || true

if [ -d ".git" ]; then
    echo "🪝 Installing Git pre-commit hook..."
    cp scripts/pre-commit .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    echo "✅ Pre-commit hook installed (runs 'npm test' before each commit)"
else
    echo "⚠️  Not a git repository - skipping pre-commit hook installation"
fi

echo ""
echo "✅ Development environment ready!"
echo ""
echo "📝 Usage:"
echo "  npm run lint              # Run all checks"
echo "  npm run check:links:all   # Check markdown links"
echo "  npm run check:frontmatter # Validate YAML frontmatter"
echo "  npm run check:rust        # Lint Rust code"
echo ""
echo "🐍 Python commands (activate venv first):"
echo "  source .venv/bin/activate"
echo "  python3 scripts/validate-frontmatter.py"
