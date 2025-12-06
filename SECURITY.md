# Security Policy

## Supported Versions

Currently supported versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue, please follow these steps:

### How to Report

1. **Do NOT** open a public GitHub issue for security vulnerabilities
2. Send a detailed report to the project maintainers via:
   - GitHub Security Advisories (preferred)
   - Email to the repository owner

### What to Include

Please include as much information as possible:
- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact
- Suggested fix (if you have one)
- Your contact information for follow-up

### Expected Response

- **Acknowledgment**: Within 48 hours of report
- **Initial Assessment**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 1-7 days
  - High: 7-30 days
  - Medium: 30-90 days
  - Low: Best effort

### Security Best Practices

When using DiskOfflaner:
1. **Run with Least Privilege**: Only run as administrator/root when necessary
2. **Verify Downloads**: Check file signatures on Windows releases
3. **Keep Updated**: Use the latest version for security patches
4. **Backup Data**: Always maintain backups before disk operations
5. **Review Operations**: Double-check before taking disks offline

## Known Security Considerations

### Administrative Privileges
- This application requires elevated privileges to perform disk operations
- Always verify the source before running with admin/root access
- Review code or use official releases only

### Windows Code Signing
- Official releases are digitally signed
- Verify signatures before execution to prevent tampering
- See `scripts/verify_signature.ps1` for verification

### Dependency Security
- Dependencies are regularly audited with `cargo audit`
- Critical vulnerabilities are addressed promptly
- See `Cargo.lock` for exact dependency versions

## Secure Development

Contributors should:
- Follow secure coding practices outlined in `CONTRIBUTING.md`
- Run `cargo clippy` to catch potential security issues
- Minimize unsafe code blocks
- Document all platform-specific security implications
- Test with various privilege levels

## Disclosure Policy

- Security vulnerabilities will be disclosed after a fix is available
- Users will be notified via GitHub releases and security advisories
- Credit will be given to researchers who responsibly disclose issues

## Contact

For security concerns, please use GitHub's security advisory feature or contact the repository maintainers directly.

Thank you for helping keep DiskOfflaner and its users safe!
