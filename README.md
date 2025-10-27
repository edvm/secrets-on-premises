# Secrets.rs

This project provides an API and a Web UI for storing encrypted data securely. It leverages modern Rust libraries to create a high-performance, type-safe application with strong security guarantees.

Try it online at [https://secrets.edvm.network](https://secrets.edvm.network)

## Features

- **Secure Storage**: All secrets are stored encrypted
- **RESTful API**: Builtin RESTful API to encrypt/decrypt secrets
- **Builtin Web UI**: A simple web interface for managing secrets

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

### Container Deployment

Alternatively, you can build and run the application using Podman (or Docker):

1. Build the container image:
   ```bash
   podman build -t secrets-on-premises -f Containerfile .
   ```

2. Run the container:
   ```bash
   podman run -d \
     --name secrets \
     -p 5150:5150 \
     -v secrets-data:/data \
     -e LOCO_ENV=production \
     secrets-on-premises
   ```

3. Access the web interface:
   Open your browser and navigate to `http://localhost:5150/`

4. View logs:
   ```bash
   podman logs -f secrets
   ```

5. Stop the container:
   ```bash
   podman stop secrets
   podman rm secrets
   ```

**Note**: The container uses a volume (`secrets-data`) to persist the SQLite database. To use a different database, mount your config file and set the appropriate environment variables.

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/secrets/encrypt/` | POST | Create a new secret |
| `/secrets/decrypt/:uid` | POST | Retrieve a secret by UUID |

