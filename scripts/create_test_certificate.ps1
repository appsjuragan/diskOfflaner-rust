# Create Test Certificate for Code Signing
# WARNING: This creates a SELF-SIGNED certificate suitable for TESTING ONLY
# For production, purchase a certificate from a trusted CA

param(
    [string]$CertName = "DiskOfflaner Test Certificate",
    [string]$Publisher = "CN=DiskOfflaner Development",
    [int]$ValidYears = 2
)

$ErrorColor = "Red"
$SuccessColor = "Green"
$InfoColor = "Cyan"
$WarningColor = "Yellow"

Write-Host "`n=== Create Test Code Signing Certificate ===" -ForegroundColor $InfoColor
Write-Host "`nWARNING: This creates a SELF-SIGNED certificate" -ForegroundColor $WarningColor
Write-Host "Self-signed certificates will show security warnings on user machines!" -ForegroundColor $WarningColor
Write-Host "Only use for testing and internal distribution.`n" -ForegroundColor $WarningColor

Write-Host "Certificate Details:" -ForegroundColor $InfoColor
Write-Host "  Name: $CertName" -ForegroundColor $InfoColor
Write-Host "  Publisher: $Publisher" -ForegroundColor $InfoColor
Write-Host "  Valid for: $ValidYears years`n" -ForegroundColor $InfoColor

# Auto-confirm for non-interactive runs if needed, but here we ask
$response = Read-Host "Do you want to continue? (yes/no)"
if ($response -ne "yes") {
    Write-Host "Cancelled" -ForegroundColor $WarningColor
    exit 0
}

Write-Host "`nCreating certificate..." -ForegroundColor $InfoColor

try {
    # Create the certificate
    $cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject $Publisher -FriendlyName $CertName -CertStoreLocation "Cert:\CurrentUser\My" -NotAfter (Get-Date).AddYears($ValidYears) -KeyUsage DigitalSignature -KeyAlgorithm RSA -KeyLength 2048 -HashAlgorithm SHA256

    Write-Host "Certificate created successfully!" -ForegroundColor $SuccessColor
    Write-Host "`nCertificate Details:" -ForegroundColor $InfoColor
    Write-Host "  Subject: $($cert.Subject)" -ForegroundColor $InfoColor
    Write-Host "  Thumbprint: $($cert.Thumbprint)" -ForegroundColor $InfoColor
    Write-Host "  Valid: $($cert.NotBefore) to $($cert.NotAfter)" -ForegroundColor $InfoColor
    
    # Export certificate to file (without private key, for distribution)
    $certPath = "DiskOfflaner_TestCert.cer"
    Export-Certificate -Cert $cert -FilePath $certPath | Out-Null
    Write-Host "`nCertificate exported to: $certPath" -ForegroundColor $SuccessColor
    
    # Trust the certificate (required for local testing)
    Write-Host "`nInstalling certificate to Trusted Root..." -ForegroundColor $InfoColor
    Write-Host "NOTE: You may see a security warning. Click 'Yes' to continue." -ForegroundColor $WarningColor
    
    $store = New-Object System.Security.Cryptography.X509Certificates.X509Store("Root", "CurrentUser")
    $store.Open("ReadWrite")
    $store.Add($cert)
    $store.Close()
    
    Write-Host "Certificate installed to Trusted Root" -ForegroundColor $SuccessColor
    
    Write-Host "`n=== SUCCESS ===" -ForegroundColor $SuccessColor
    Write-Host "Test certificate created and installed!`n" -ForegroundColor $SuccessColor
    
    Write-Host "Next steps:" -ForegroundColor $InfoColor
    Write-Host "1. Sign your executable:" -ForegroundColor $InfoColor
    Write-Host "   .\scripts\sign_release.ps1 -CertThumbprint $($cert.Thumbprint)" -ForegroundColor $InfoColor
    Write-Host "`n2. OR just run (it will auto-detect the certificate):" -ForegroundColor $InfoColor
    Write-Host "   .\scripts\sign_release.ps1" -ForegroundColor $InfoColor
    
    Write-Host "`nIMPORTANT NOTES:" -ForegroundColor $WarningColor
    Write-Host "- This is a SELF-SIGNED certificate (not trusted by default)" -ForegroundColor $WarningColor
    Write-Host "- Users will see security warnings unless they manually trust this cert" -ForegroundColor $WarningColor
    Write-Host "- For production, purchase a certificate from DigiCert, Sectigo, etc." -ForegroundColor $WarningColor
    Write-Host "- Certificate file: $certPath" -ForegroundColor $WarningColor
    Write-Host ""
    
}
catch {
    Write-Host "`nERROR: Failed to create certificate" -ForegroundColor $ErrorColor
    Write-Host $_.Exception.Message -ForegroundColor $ErrorColor
    exit 1
}
