# Code Signing Quick Reference

## Quick Start (Testing)

### 1. Create Test Certificate
```powershell
.\scripts\create_test_certificate.ps1
```

### 2. Sign Executable
```powershell
cargo build --release
.\scripts\sign_release.ps1
```

### 3. Verify Signature
```powershell
.\scripts\verify_signature.ps1
```

## Quick Start (Production)

### 1. Obtain Certificate
Purchase from:
- DigiCert: https://www.digicert.com/code-signing
- Sectigo: https://sectigo.com/ssl-certificates-tls/code-signing
- SSL.com: https://www.ssl.com/certificates/code-signing/

### 2. Import Certificate
```powershell
# Double-click the .pfx file or:
certutil -f -p YOUR_PASSWORD -importpfx "path\to\certificate.pfx"
```

### 3. Build and Sign
```powershell
cargo build --release
.\scripts\sign_release.ps1
```

## Common Commands

### List Installed Certificates
```powershell
Get-ChildItem -Path Cert:\CurrentUser\My -CodeSigningCert
```

### Sign with Specific Certificate
```powershell
.\scripts\sign_release.ps1 -CertThumbprint "YOUR_THUMBPRINT_HERE"
```

### Sign with Custom Timestamp Server
```powershell
.\scripts\sign_release.ps1 -TimestampServer "http://timestamp.sectigo.com"
```

### Manual Signing (Advanced)
```powershell
signtool sign /sha1 THUMBPRINT /t http://timestamp.digicert.com /fd SHA256 /v .\target\release\diskofflaner.exe
```

### Verify Signature
```powershell
# Using PowerShell
Get-AuthenticodeSignature .\target\release\diskofflaner.exe

# Using signtool
signtool verify /pa /v .\target\release\diskofflaner.exe

# Using script
.\scripts\verify_signature.ps1
```

## Timestamp Servers

| Provider | URL |
|----------|-----|
| DigiCert | http://timestamp.digicert.com |
| Sectigo | http://timestamp.sectigo.com |
| GlobalSign | http://timestamp.globalsign.com |

## File Locations

| File | Purpose |
|------|---------|
| `scripts/sign_release.ps1` | Main signing script |
| `scripts/create_test_certificate.ps1` | Create test certificate |
| `scripts/verify_signature.ps1` | Verify signature |
| `scripts/github_actions_example.yml` | CI/CD example |
| `.agent/code_signing_guide.md` | Full documentation |

## Troubleshooting

### "signtool not found"
```powershell
# Find signtool
Get-ChildItem "C:\Program Files (x86)\Windows Kits\" -Recurse -Filter signtool.exe

# Add to PATH (replace version)
$env:Path += ";C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64"
```

### "No certificates found"
```powershell
# Check installed certificates
Get-ChildItem Cert:\CurrentUser\My

# Create test certificate
.\scripts\create_test_certificate.ps1

# Or import your .pfx file
certutil -importpfx "path\to\cert.pfx"
```

### SmartScreen Warnings
- Normal for new certificates
- Builds reputation over time (weeks/months)
- Self-signed certificates always show warnings
- Use commercial certificate for production

## Best Practices

✅ **Always timestamp your signatures**
- Use `/t` or `/tr` parameter
- Signature remains valid after certificate expires

✅ **Use SHA256 for signing**
- More secure than SHA1
- Required by modern Windows

✅ **Keep certificates secure**
- Strong passwords
- Hardware tokens when possible
- Never commit to git

✅ **Test on clean VMs**
- Verify SmartScreen behavior
- Check signature validation
- Test auto-updates if applicable

## Cost Estimates

| Certificate Type | Annual Cost | Best For |
|------------------|-------------|----------|
| Standard Code Signing | $200-$400 | Small projects |
| EV Code Signing | $400-$600 | Immediate SmartScreen trust |
| Self-Signed | Free | Testing only |

## Resources

- [Microsoft Code Signing Guide](https://docs.microsoft.com/en-us/windows/win32/seccrypto/using-signtool-to-sign-a-file)
- [DigiCert Resources](https://www.digicert.com/support/tools/code-signing)
- Full guide: `.agent/code_signing_guide.md`
