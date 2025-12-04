# Verify Digital Signature
# This script verifies the digital signature of diskofflaner.exe

param(
    [string]$ExePath = ".\target\release\diskofflaner.exe"
)

$ErrorColor = "Red"
$SuccessColor = "Green"
$InfoColor = "Cyan"
$WarningColor = "Yellow"

Write-Host "`n=== DiskOfflaner Signature Verification ===" -ForegroundColor $InfoColor
Write-Host "Executable: $ExePath`n" -ForegroundColor $InfoColor

# Check if file exists
if (-not (Test-Path $ExePath)) {
    Write-Host "ERROR: File not found: $ExePath" -ForegroundColor $ErrorColor
    exit 1
}

# Get file info
$fileInfo = Get-Item $ExePath
Write-Host "=== File Information ===" -ForegroundColor $InfoColor
Write-Host "Path: $($fileInfo.FullName)" -ForegroundColor $InfoColor
Write-Host "Size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB ($($fileInfo.Length) bytes)" -ForegroundColor $InfoColor
Write-Host "Modified: $($fileInfo.LastWriteTime)" -ForegroundColor $InfoColor
Write-Host "SHA256: $((Get-FileHash $ExePath -Algorithm SHA256).Hash)" -ForegroundColor $InfoColor

# Check signature using PowerShell
Write-Host "`n=== Signature Check (PowerShell) ===" -ForegroundColor $InfoColor
$signature = Get-AuthenticodeSignature $ExePath

Write-Host "Status: " -NoNewline
switch ($signature.Status) {
    "Valid" { 
        Write-Host $signature.Status -ForegroundColor $SuccessColor 
    }
    "NotSigned" { 
        Write-Host $signature.Status -ForegroundColor $WarningColor
        Write-Host "`nThe executable is not digitally signed." -ForegroundColor $WarningColor
        Write-Host "To sign it, run: .\scripts\sign_release.ps1" -ForegroundColor $InfoColor
        exit 1
    }
    default { 
        Write-Host $signature.Status -ForegroundColor $ErrorColor 
    }
}

Write-Host "Status Message: $($signature.StatusMessage)" -ForegroundColor $InfoColor

if ($signature.SignerCertificate) {
    Write-Host "`n=== Signer Certificate ===" -ForegroundColor $InfoColor
    Write-Host "Subject: $($signature.SignerCertificate.Subject)" -ForegroundColor $InfoColor
    Write-Host "Issuer: $($signature.SignerCertificate.Issuer)" -ForegroundColor $InfoColor
    Write-Host "Serial Number: $($signature.SignerCertificate.SerialNumber)" -ForegroundColor $InfoColor
    Write-Host "Thumbprint: $($signature.SignerCertificate.Thumbprint)" -ForegroundColor $InfoColor
    Write-Host "Valid From: $($signature.SignerCertificate.NotBefore)" -ForegroundColor $InfoColor
    Write-Host "Valid Until: $($signature.SignerCertificate.NotAfter)" -ForegroundColor $InfoInfo
    
    $daysUntilExpiry = ($signature.SignerCertificate.NotAfter - (Get-Date)).Days
    if ($daysUntilExpiry -lt 30) {
        Write-Host "WARNING: Certificate expires in $daysUntilExpiry days!" -ForegroundColor $WarningColor
    } else {
        Write-Host "Days Until Expiry: $daysUntilExpiry" -ForegroundColor $InfoColor
    }
}

if ($signature.TimeStamperCertificate) {
    Write-Host "`n=== Timestamp Information ===" -ForegroundColor $InfoColor
    Write-Host "Timestamp: $($signature.TimeStamperCertificate.NotBefore)" -ForegroundColor $InfoColor
    Write-Host "Timestamp Authority: $($signature.TimeStamperCertificate.Subject)" -ForegroundColor $InfoColor
    Write-Host "✓ Signature will remain valid after certificate expiry" -ForegroundColor $SuccessColor
} else {
    Write-Host "`nWARNING: No timestamp found!" -ForegroundColor $WarningColor
    Write-Host "Signature will become invalid when certificate expires" -ForegroundColor $WarningColor
}

# Check with signtool if available
Write-Host "`n=== Signature Check (signtool) ===" -ForegroundColor $InfoColor
try {
    $null = Get-Command signtool -ErrorAction Stop
    Write-Host "Running: signtool verify /pa /v $ExePath`n" -ForegroundColor $InfoColor
    & signtool verify /pa /v $ExePath
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "`n✓ signtool verification: PASSED" -ForegroundColor $SuccessColor
    } else {
        Write-Host "`n⚠ signtool verification: FAILED (Exit code: $LASTEXITCODE)" -ForegroundColor $WarningColor
        Write-Host "This may be normal for self-signed certificates" -ForegroundColor $WarningColor
    }
} catch {
    Write-Host "signtool not found in PATH (optional)" -ForegroundColor $WarningColor
}

# Summary
Write-Host "`n=== Summary ===" -ForegroundColor $InfoColor
if ($signature.Status -eq "Valid") {
    Write-Host "✓ Executable is properly signed" -ForegroundColor $SuccessColor
    
    # Check if self-signed
    if ($signature.SignerCertificate.Issuer -eq $signature.SignerCertificate.Subject) {
        Write-Host "`n⚠ NOTE: This is a SELF-SIGNED certificate" -ForegroundColor $WarningColor
        Write-Host "Users may see security warnings unless they trust this certificate" -ForegroundColor $WarningColor
        Write-Host "For production, use a certificate from a trusted CA" -ForegroundColor $WarningColor
    } else {
        Write-Host "Certificate is issued by a Certificate Authority" -ForegroundColor $InfoColor
    }
    
    if ($signature.TimeStamperCertificate) {
        Write-Host "✓ Signature is timestamped" -ForegroundColor $SuccessColor
    } else {
        Write-Host "⚠ Missing timestamp (sign with /t parameter)" -ForegroundColor $WarningColor
    }
} else {
    Write-Host "✗ Signature verification failed: $($signature.Status)" -ForegroundColor $ErrorColor
    Write-Host "Reason: $($signature.StatusMessage)" -ForegroundColor $ErrorColor
}

Write-Host ""
