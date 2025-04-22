# Secrets API

A secure API for storing and managing encrypted secrets built with Rust ideal for security-critical applications where performance and correctness are essential.

## Overview

This project provides an API for storing encrypted data securely. It leverages modern Rust libraries to create a high-performance, type-safe application with strong security guarantees.

## Features

- **Secure Storage**: All secrets are stored encrypted
- **UUID-Based Access**: Secrets are accessed via UUIDs
- **Serializable Models**: API responses are cleanly structured using serialization
- **RESTful API**: Simple and intuitive RESTful endpoints (encrypt/decrypt)

## Architecture

The project is built with the following technologies:

- **[Loco.rs](https://loco.rs/)**: Modern Rust framework for building applications
- **[SeaORM](https://www.sea-ql.org/SeaORM/)**: Async ORM for Rust
- **[AES256 Encryption](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard)**: Industry-standard encryption for data security

### Project Structure

```
secrets/
├── src/
│   ├── models/
│   │   └── secrets.rs       # Data models and serialization
│   ├── cryptography/
│   │   └── encryption.rs    # Encryption/decryption logic
│   ├── routes.rs            # API endpoints
│   └── main.rs              # Application entry point
└── ...
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/secrets/decrypt/:uid` | POST | Retrieve a secret by UUID |
| `/secrets/encrypt` | POST | Create a new secret |

## Getting Started

### Prerequisites

- Rust 1.70+
- (Optional) PostgreSQL or other supported database

### Installation

1. Clone this repository and:
   ```bash
   cd secrets
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the server:
   ```bash
   ```

## Usage Example

### Creating a Secret

```bash
curl -X POST http://localhost:5150/secrets \
  -H "Content-Type: application/json" \
  -d '{"data": "your-secret-data", "passphrase": "your-secure-passphrase"}'
```

### Retrieving a Secret

```bash
curl http://localhost:5150/secrets/123e4567-e89b-12d3-a456-426614174000
```

## Development

### Adding a New Remote Repository

```bash
# Add a GitHub remote
git remote add origin https://github.com/yourusername/secrets.git

# Verify remotes
git remote -v

# Push to remote
git push -u origin main
```

### Running Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [Loco.rs](https://loco.rs/) framework
- Database access via [SeaORM](https://www.sea-ql.org/SeaORM/)
- Web framework [Axum](https://github.com/tokio-rs/axum)
