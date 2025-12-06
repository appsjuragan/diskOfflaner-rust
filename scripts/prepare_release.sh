#!/bin/bash
# Release Preparation Script for Linux
# Run this script before creating a new release

set -e

echo "================================"
echo "DiskOfflaner Release Preparation"
echo "================================"
echo ""

# Step 1: Format code
echo "[1/7] Formatting code with rustfmt..."
cargo fmt --all
echo "✓ Code formatted successfully"
echo ""

# Step 2: Run clippy
echo "[2/7] Running clippy linter..."
cargo clippy --all-targets --all-features -- -D warnings
echo "✓ Clippy checks passed"
echo ""

# Step 3: Run tests
echo "[3/7] Running tests..."
cargo test --all
echo "✓ All tests passed"
echo ""

# Step 4: Security audit
echo "[4/7] Running security audit..."
if cargo audit; then
    echo "✓ Security audit passed"
else
    echo "WARNING: Security audit found issues (check if they're critical)"
fi
echo ""

# Step 5: Build debug
echo "[5/7] Building debug version..."
cargo build
echo "✓ Debug build successful"
echo ""

# Step 6: Build release
echo "[6/7] Building release version..."
cargo build --release
echo "✓ Release build successful"
echo ""

# Step 7: Display binary info
echo "[7/7] Release information..."
if [ -f "target/release/diskofflaner" ]; then
    size=$(du -h "target/release/diskofflaner" | cut -f1)
    echo "Binary location: target/release/diskofflaner"
    echo "Binary size: $size"
fi
echo ""

echo "================================"
echo "✓ All checks passed!"
echo "================================"
echo ""
echo "Next steps:"
echo "1. Review CHANGELOG.md and update if needed"
echo "2. Update version in Cargo.toml if needed"
echo "3. Create a git tag: git tag -a v1.0.3 -m 'Release v1.0.3'"
echo "4. Push to GitHub: git push origin v1.0.3"
echo "5. Create GitHub release and upload binaries"
echo ""
