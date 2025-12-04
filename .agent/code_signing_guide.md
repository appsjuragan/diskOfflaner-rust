# Code Signing Guide for DiskOfflaner

**Purpose**: Digitally sign the release executable to establish trust and avoid Windows SmartScreen warnings.

## Prerequisites

### 1. Obtain a Code Signing Certificate

You have two options:

#### Option A: Commercial Certificate (Recommended for Public Distribution)
Purchase from a trusted Certificate Authority (CA):
- **DigiCert** - $400-$600/year (most trusted)
- **Sectigo (Comodo)** - $200-$400/year
- **GlobalSign** - $300-$500/year
- **SSL.com** - $200-$400/year

**Benefits:**
- Immediate trust by Windows SmartScreen
- No security warnings for users
- Builds reputation over time

**Process:**
1. Purchase certificate from CA
2. Complete identity verification (may require business documents)
3. Receive certificate file (.pfx or .p12)
4. Install on signing machine

#### Option B: Self-Signed Certificate (For Testing/Internal Use)
**⚠️ Warning**: Self-signed certificates will still show warnings on user machines unless they manually trust your certificate.

**Use cases:**
- Internal distribution only
- Testing the signing process
- Development builds

## Installation Steps

### Step 1: Install Windows SDK (for signtool.exe)

**Option A: Install via Visual Studio**
- Install Visual Studio with "Desktop development with C++"
- signtool.exe will be in: `C:\Program Files (x86)\Windows Kits\10\bin\<version>\x64\`

**Option B: Standalone Windows SDK**
```powershell
# Download from: https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/
# Or use winget:
winget install Microsoft.WindowsSDK
```

### Step 2: Add signtool to PATH

```powershell
# Find signtool location
Get-ChildItem -Path "C:\Program Files (x86)\Windows Kits\" -Recurse -Filter "signtool.exe" | Select-Object FullName

# Add to PATH (replace <version> with actual version)
$env:Path += ";C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64"
```

### Step 3: Import Your Certificate

**For .pfx file:**
```powershell
# Double-click the .pfx file and follow the import wizard
# OR use certutil:
certutil -f -p YOUR_PASSWORD -importpfx "path\to\certificate.pfx"
```

**For self-signed certificate (testing only):**
```powershell
# Run the create_test_certificate.ps1 script (see below)
```

## Signing the Executable

### Manual Signing

```powershell
# Navigate to the project directory
cd f:\source_code\rust-diskofflaner

# Sign the executable
signtool sign /sha1 CERTIFICATE_THUMBPRINT /t http://timestamp.digicert.com /fd SHA256 /v .\target\release\diskofflaner.exe

# Verify signature
signtool verify /pa /v .\target\release\diskofflaner.exe
```

**Parameters explained:**
- `/sha1 THUMBPRINT` - Certificate thumbprint (get from Certificate Manager)
- `/t URL` - Timestamp server (proves when code was signed)
- `/fd SHA256` - Use SHA256 digest algorithm
- `/v` - Verbose output
- `/pa` - Verify against Windows policies

### Automated Signing Script

Use the provided `sign_release.ps1` script (see below).

```powershell
# Run the signing script
.\scripts\sign_release.ps1
```

## Finding Your Certificate Thumbprint

```powershell
# List all code signing certificates
Get-ChildItem -Path Cert:\CurrentUser\My -CodeSigningCert | Format-List Subject, Thumbprint, NotAfter

# OR open Certificate Manager
certmgr.msc
# Navigate to Personal > Certificates
# Double-click your certificate > Details > Thumbprint
```

## Timestamp Servers

Always use a timestamp server when signing. This ensures your signature remains valid even after the certificate expires.

**Recommended timestamp servers:**
- DigiCert: `http://timestamp.digicert.com`
- Sectigo: `http://timestamp.sectigo.com`
- GlobalSign: `http://timestamp.globalsign.com`
- RFC 3161: `http://timestamp.comodoca.com/rfc3161` (use /tr instead of /t)

## Verification

After signing, verify the signature:

```powershell
# Check signature exists
signtool verify /pa /v .\target\release\diskofflaner.exe

# View signature details
Get-AuthenticodeSignature .\target\release\diskofflaner.exe | Format-List *
```

**Good signature output:**
```
Successfully verified: diskofflaner.exe
SignedCM: Valid
SignerAlgorithm: sha256RSA
Status: Valid
```

## Building Reputation with SmartScreen

Even with a valid certificate, Microsoft SmartScreen may show warnings initially. To build reputation:

1. **Use the same certificate** for all releases
2. **Don't change company name** or certificate details
3. **Distribute widely** - more downloads = better reputation
4. **Time** - reputation builds over weeks/months
5. **Report false positives** to Microsoft if needed

## Integration with Build Process

### Option 1: Manual Process
1. Build release: `cargo build --release`
2. Run signing script: `.\scripts\sign_release.ps1`
3. Verify signature
4. Distribute signed executable

### Option 2: Automated (CI/CD)
For GitHub Actions or other CI/CD, you'll need to:
1. Store certificate securely (Azure Key Vault, GitHub Secrets)
2. Install signtool in CI environment
3. Sign as part of release workflow

See `signing_workflow_example.yml` for GitHub Actions example.

## Security Best Practices

✅ **DO:**
- Store certificate files securely with strong passwords
- Use hardware tokens (USB) for certificate storage when possible
- Timestamp all signed executables
- Keep certificate private key secure
- Revoke certificate immediately if compromised

❌ **DON'T:**
- Share certificate files or passwords
- Commit certificates to version control
- Use self-signed certs for public distribution
- Skip timestamping

## Troubleshooting

### "No certificates were found that met all the given criteria"
- Certificate not imported to Personal store
- Wrong certificate thumbprint
- Certificate expired

### "SignTool Error: No signing certificate was found"
- signtool not in PATH
- Wrong Windows SDK version
- Missing certificate

### "Signature verification failed"
- Certificate not trusted by Windows
- Timestamp server unreachable during signing
- Corrupted signature

### SmartScreen still shows warnings
- Certificate is new (needs reputation)
- Self-signed certificate (not trusted)
- Certificate from untrusted CA

## Cost-Benefit Analysis

| Approach | Cost | Trust Level | Best For |
|----------|------|-------------|----------|
| Commercial Certificate | $200-600/year | High | Public distribution |
| Self-Signed | Free | Low | Internal/testing only |
| No Signature | Free | None | Dev builds only |

## Next Steps

1. ✅ Review certificate options
2. ✅ Obtain certificate (or create test certificate)
3. ✅ Install Windows SDK
4. ✅ Configure signing script with your certificate
5. ✅ Sign the executable
6. ✅ Test on clean Windows VM
7. ✅ Distribute signed binaries

## Files Created

- `scripts/sign_release.ps1` - Automated signing script
- `scripts/create_test_certificate.ps1` - Create self-signed cert for testing
- `scripts/verify_signature.ps1` - Verify signature script
- `.github/workflows/release.yml` - Example GitHub Actions workflow (optional)
