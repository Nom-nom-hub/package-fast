# Package Integrity Verification

The integrity module provides cryptographic verification of package contents to ensure they haven't been tampered with or corrupted.

## Overview

This module uses SHA-512 cryptographic hashes to verify the integrity of package files. It can calculate hashes for files and verify that files match expected hash values.

## Usage

### Calculating Package Hashes

```rust
use package_fast_security::integrity::calculate_package_hash;
use std::path::Path;

let file_path = Path::new("/path/to/package.tgz");
let hash = calculate_package_hash(file_path)?;
println!("Package hash: {}", hash);
```

### Verifying Package Integrity

```rust
use package_fast_security::integrity::verify_package_integrity;
use std::path::Path;

let file_path = Path::new("/path/to/package.tgz");
let expected_hash = "expected-sha512-hash";

match verify_package_integrity(file_path, expected_hash) {
    Ok(()) => println!("Package integrity verified"),
    Err(e) => println!("Integrity check failed: {}", e),
}
```

## Error Handling

The module provides specific error types for different failure scenarios:

- `IntegrityError::HashMismatch`: The calculated hash doesn't match the expected hash
- `IntegrityError::IoError`: File I/O errors occurred
- `IntegrityError::InvalidHashFormat`: The provided hash format is invalid

## Security Considerations

- SHA-512 is a strong cryptographic hash function that is resistant to collision attacks
- For even stronger security, consider using digital signatures in addition to hash verification
- Always verify packages before execution or installation
- Store expected hashes securely to prevent tampering

## Performance

Hash calculation is optimized for performance:
- Streaming hash calculation to handle large files efficiently
- Minimal memory usage during hash computation
- Asynchronous operations to avoid blocking

## Integration

This module integrates with:
- Package verification during installation
- Cache validation for downloaded packages
- Security audit trails for integrity checks