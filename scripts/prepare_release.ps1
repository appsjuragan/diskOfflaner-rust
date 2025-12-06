# Release Preparation Script
# Run this script before creating a new release

Write-Host "================================" -ForegroundColor Cyan
Write-Host "DiskOfflaner Release Preparation" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""

# Step 1: Format code
Write-Host "[1/7] Formatting code with rustfmt..." -ForegroundColor Yellow
cargo fmt --all
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Code formatting failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Code formatted successfully" -ForegroundColor Green
Write-Host ""

# Step 2: Run clippy
Write-Host "[2/7] Running clippy linter..." -ForegroundColor Yellow
cargo clippy --all-targets --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Clippy found issues!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Clippy checks passed" -ForegroundColor Green
Write-Host ""

# Step 3: Run tests
Write-Host "[3/7] Running tests..." -ForegroundColor Yellow
cargo test --all
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Tests failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ All tests passed" -ForegroundColor Green
Write-Host ""

# Step 4: Security audit
Write-Host "[4/7] Running security audit..." -ForegroundColor Yellow
cargo audit
if ($LASTEXITCODE -ne 0) {
    Write-Host "WARNING: Security audit found issues (check if they're critical)" -ForegroundColor Yellow
}
else {
    Write-Host "✓ Security audit passed" -ForegroundColor Green
}
Write-Host ""

# Step 5: Build debug
Write-Host "[5/7] Building debug version..." -ForegroundColor Yellow
cargo build
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Debug build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Debug build successful" -ForegroundColor Green
Write-Host ""

# Step 6: Build release
Write-Host "[6/7] Building release version..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Release build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Release build successful" -ForegroundColor Green
Write-Host ""

# Step 7: Display binary info
Write-Host "[7/7] Release information..." -ForegroundColor Yellow
$binary = "target\release\diskofflaner.exe"
if (Test-Path $binary) {
    $size = (Get-Item $binary).Length
    $sizeMB = [math]::Round($size / 1MB, 2)
    Write-Host "Binary location: $binary" -ForegroundColor Cyan
    Write-Host "Binary size: $sizeMB MB" -ForegroundColor Cyan
}
Write-Host ""

Write-Host "================================" -ForegroundColor Cyan
Write-Host "✓ All checks passed!" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Review CHANGELOG.md and update if needed"
Write-Host "2. Update version in Cargo.toml if needed"
Write-Host "3. Sign the binary: .\scripts\sign_release.ps1"
Write-Host "4. Create a git tag: git tag -a v1.0.3 -m 'Release v1.0.3'"
Write-Host "5. Push to GitHub: git push origin v1.0.3"
Write-Host "6. Create GitHub release and upload binaries"
Write-Host ""
