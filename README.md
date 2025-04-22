# Secrets API

A secure API for storing and managing encrypted secrets built with Rust ideal for security-critical applications where performance and correctness are essential.

## Overview

This project provides an API for storing encrypted data securely. It leverages modern Rust libraries to create a high-performance, type-safe application with strong security guarantees.

## Features

- **Secure Storage**: All secrets are stored encrypted
- **Builtin Web UI**: A simple web interface for managing secrets
- **RESTful API**: Builtin RESTful API to encrypt/decrypt secrets

### Installation

1. Clone this repository and:
   ```bash
   cd secrets-on-premises
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the server:
   ```bash
   ./target/release/secrets-cli start
   ```

4. Access the web interface:
   Open your browser and navigate to `http://localhost:5150/` to access the web interface.


## Architecture

The project is built with the following technologies:

- **[Loco.rs](https://loco.rs/)**: Modern Rust framework for building applications
- **[SeaORM](https://www.sea-ql.org/SeaORM/)**: Async ORM for Rust
- **[AES256 Encryption](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard)**: Industry-standard encryption for data security


## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/secrets/encrypt/` | POST | Create a new secret |
| `/secrets/decrypt/:uid` | POST | Retrieve a secret by UUID |

## Getting Started

### Prerequisites

- Rust 1.70+
- (Optional) PostgreSQL or other supported database. SQLite is used by default.

## Usage Example

### Creating a Secret

```bash
curl -X POST http://localhost:5150/api/secrets/encrypt \
  -H "Content-Type: application/json" \
  -d '{"text": "your-secret-data", "passphrase": "your-secure-passphrase"}'
```

### Retrieving a Secret

```bash
curl -X POST http://localhost:5150/api/secrets/decrypt/c96b3797-396c-41b2-8266-2cce52effaaf \
  -H "Content-Type: application/json" \
  -d '{"passphrase": "your-secure-passphrase"}'
```

## Development

### Adding a New Remote Repository

```bash
# Add a GitHub remote
git remote add origin https://github.com/yourusername/secrets-on-premises.git

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

This project is licensed under the GNU AFFERO GENERAL PUBLIC LICENSE - see the LICENSE file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [Loco.rs](https://loco.rs/) framework
- Database access via [SeaORM](https://www.sea-ql.org/SeaORM/)
- Web framework [Axum](https://github.com/tokio-rs/axum)
- Rust amazing community :love:
- [Rustaceans](https://www.rust-lang.org/community)
