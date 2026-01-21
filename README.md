# Shamir

Encrypted secret store with Shamir Secret Sharing support.

## Features

- **Encrypted Storage**: AES-256-GCM encryption, secrets only decrypted in memory
- **BIP39 Keys**: 256-bit keys encoded as 24-word mnemonics
- **Shamir Secret Sharing**: Split keys into shares for distributed recovery
- **TUI Interface**: Terminal-based UI for managing secrets

## Usage

```bash
cargo run
```

### Workflow

1. **Init Store**: Generate new 256-bit key and create empty store
2. **Load Store**: Decrypt existing store with key/mnemonic
3. **Edit Store**: Add/modify secrets (in-memory only)
4. **Save Store**: Encrypt and write to disk

Keys never touch disk in plaintext. Store file remains encrypted at rest.
