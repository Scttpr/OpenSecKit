#!/usr/bin/env python3
"""
Validate YAML frontmatter in OpenSecKit templates.
Extracted from .github/workflows/doc-validation.yml
"""

import frontmatter
import subprocess
import sys

required_keys = ['title', 'constitutional_principle', 'ssdlc_phase']
failed = False

# Use git ls-files to get only versioned files (respects .gitignore)
result = subprocess.run(
    ['git', 'ls-files', 'templates/**/*.md', 'domaines/**/*.md'],
    capture_output=True,
    text=True
)
files = result.stdout.strip().split('\n') if result.stdout else []

print(f"🔍 Audit {len(files)} templates...")

for file in files:
    if "_example-" in file or "README.md" in file:
        continue

    try:
        with open(file, 'r', encoding='utf-8') as f:
            post = frontmatter.load(f)

        missing = [key for key in required_keys if key not in post.metadata]

        if missing:
            print(f"❌ {file}: Missing keys {missing}")
            failed = True
    except Exception as e:
        print(f"❌ {file}: Parsing error - {e}")
        failed = True

if failed:
    sys.exit(1)
print("✅ Validation OK.")
