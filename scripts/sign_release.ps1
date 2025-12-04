# Sign Release Executable
# This script signs the diskofflaner.exe with a code signing certificate

param(
    [string]$CertThumbprint = "",
    [string]$TimestampServer = "http://timestamp.digicert.com",
    [string]$ExePath = ".\target\release\diskofflaner.exe"
)

# Colors for output
$ErrorColor = "Red"
$SuccessColor = "Green"
$InfoColor = "Cyan"
$WarningColor = "Yellow"

Write-Host "`n=== DiskOfflaner Code Signing Script ===" -ForegroundColor $InfoColor
Write-Host "Executable: $ExePath`n" -ForegroundColor $InfoColor

# Check if executable exists
if (-not (Test-Path $ExePath)) {
    Write-Host "ERROR: Executable not found at: $ExePath" -ForegroundColor $ErrorColor
    Write-Host "Please build the release first: cargo build --release" -ForegroundColor $WarningColor
    exit 1
}

# Check if signtool is available
try {
    $signtool = (Get-Command signtool -ErrorAction Stop).Source
    Write-Host "signtool.exe found in PATH" -ForegroundColor $SuccessColor
}
catch {
    Write-Host "signtool.exe not in PATH, searching..." -ForegroundColor $WarningColor
    $signtool = Get-ChildItem "C:\Program Files (x86)\Windows Kits" -Recurse -Filter "signtool.exe" -ErrorAction SilentlyContinue | 
    Where-Object { $_.FullName -like "*\x64\*" } | 
    Select-Object -First 1 -ExpandProperty FullName
    
    if ($signtool) {
        Write-Host "Found signtool: $signtool" -ForegroundColor $SuccessColor
    }
    else {
        Write-Host "ERROR: signtool.exe not found" -ForegroundColor $ErrorColor
        Write-Host "Please install Windows SDK" -ForegroundColor $WarningColor
        exit 1
    }
}

# If certificate thumbprint not provided, try to find one
if ([string]::IsNullOrWhiteSpace($CertThumbprint)) {
    Write-Host "`nSearching for code signing certificates..." -ForegroundColor $InfoColor
    
    $certs = Get-ChildItem -Path Cert:\CurrentUser\My -CodeSigningCert | 
    Where-Object { $_.NotAfter -gt (Get-Date) }
    
    if ($certs.Count -eq 0) {
        Write-Host "ERROR: No valid code signing certificates found" -ForegroundColor $ErrorColor
        Write-Host "`nAvailable options:" -ForegroundColor $WarningColor
        Write-Host "1. Purchase a certificate from a trusted CA" -ForegroundColor $WarningColor
        Write-Host "2. Create a test certificate: .\scripts\create_test_certificate.ps1" -ForegroundColor $WarningColor
        exit 1
    }
    
    Write-Host "`nFound $($certs.Count) certificate(s):" -ForegroundColor $SuccessColor
    for ($i = 0; $i -lt $certs.Count; $i++) {
        $cert = $certs[$i]
        Write-Host "`n[$i] Subject: $($cert.Subject)" -ForegroundColor $InfoColor
        Write-Host "    Thumbprint: $($cert.Thumbprint)" -ForegroundColor $InfoColor
        Write-Host "    Expires: $($cert.NotAfter)" -ForegroundColor $InfoColor
        Write-Host "    Issuer: $($cert.Issuer)" -ForegroundColor $InfoColor
    }
    
    if ($certs.Count -eq 1) {
        $CertThumbprint = $certs[0].Thumbprint
        Write-Host "`nUsing certificate: $($certs[0].Subject)" -ForegroundColor $SuccessColor
    }
    else {
        Write-Host "`nPlease specify which certificate to use:" -ForegroundColor $WarningColor
        Write-Host "  .\scripts\sign_release.ps1 -CertThumbprint YOUR_THUMBPRINT" -ForegroundColor $WarningColor
        exit 1
    }
}

# Verify certificate exists
$cert = Get-ChildItem -Path Cert:\CurrentUser\My | Where-Object { $_.Thumbprint -eq $CertThumbprint }
if (-not $cert) {
    Write-Host "ERROR: Certificate with thumbprint $CertThumbprint not found" -ForegroundColor $ErrorColor
    exit 1
}

Write-Host "`n=== Certificate Details ===" -ForegroundColor $InfoColor
Write-Host "Subject: $($cert.Subject)" -ForegroundColor $InfoColor
Write-Host "Issuer: $($cert.Issuer)" -ForegroundColor $InfoColor
Write-Host "Thumbprint: $($cert.Thumbprint)" -ForegroundColor $InfoColor
Write-Host "Valid: $($cert.NotBefore) to $($cert.NotAfter)" -ForegroundColor $InfoColor

# Check if already signed
Write-Host "`nChecking existing signature..." -ForegroundColor $InfoColor
$existingSig = Get-AuthenticodeSignature $ExePath
if ($existingSig.Status -eq "Valid") {
    Write-Host "WARNING: Executable is already signed" -ForegroundColor $WarningColor
    Write-Host "Existing signature: $($existingSig.SignerCertificate.Subject)" -ForegroundColor $WarningColor
    
    # Auto-confirm for non-interactive runs if needed, but here we ask
    $response = Read-Host "Do you want to re-sign? (yes/no)"
    if ($response -ne "yes") {
        Write-Host "Signing cancelled" -ForegroundColor $WarningColor
        exit 0
    }
}

# Sign the executable
Write-Host "`n=== Signing Executable ===" -ForegroundColor $InfoColor
Write-Host "Timestamp server: $TimestampServer" -ForegroundColor $InfoColor

$signArgs = @(
    "sign",
    "/sha1", $CertThumbprint,
    "/t", $TimestampServer,
    "/fd", "SHA256",
    "/v",
    $ExePath
)

Write-Host "`nRunning: signtool sign ..." -ForegroundColor $InfoColor
Write-Host ""

& $signtool @signArgs

if ($LASTEXITCODE -ne 0) {
    Write-Host "`nERROR: Signing failed with exit code $LASTEXITCODE" -ForegroundColor $ErrorColor
    exit $LASTEXITCODE
}

# Verify the signature
Write-Host "`n=== Verifying Signature ===" -ForegroundColor $InfoColor

& $signtool verify /pa /v $ExePath

if ($LASTEXITCODE -ne 0) {
    Write-Host "`nWARNING: Signature verification failed" -ForegroundColor $WarningColor
    Write-Host "This may be normal for self-signed certificates" -ForegroundColor $WarningColor
}
else {
    Write-Host "`nSignature verified successfully!" -ForegroundColor $SuccessColor
}

# Display signature details
Write-Host "`n=== Signature Details ===" -ForegroundColor $InfoColor
$signature = Get-AuthenticodeSignature $ExePath
if ($signature.Status -eq "Valid") {
    Write-Host "Status: $($signature.Status)" -ForegroundColor $SuccessColor
}
else {
    Write-Host "Status: $($signature.Status)" -ForegroundColor $WarningColor
}
Write-Host "Signer: $($signature.SignerCertificate.Subject)" -ForegroundColor $InfoColor
if ($signature.TimeStamperCertificate) {
    Write-Host "Timestamp: $($signature.TimeStamperCertificate.NotBefore)" -ForegroundColor $InfoColor
}
else {
    Write-Host "Timestamp: None" -ForegroundColor $WarningColor
}

# Get file info
$fileInfo = Get-Item $ExePath
Write-Host "`n=== File Information ===" -ForegroundColor $InfoColor
Write-Host "Path: $($fileInfo.FullName)" -ForegroundColor $InfoColor
Write-Host "Size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor $InfoColor
Write-Host "Modified: $($fileInfo.LastWriteTime)" -ForegroundColor $InfoColor

Write-Host "`n=== SUCCESS ===" -ForegroundColor $SuccessColor
Write-Host "The executable has been signed successfully!" -ForegroundColor $SuccessColor
