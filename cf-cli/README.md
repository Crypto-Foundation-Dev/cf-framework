# cf-cli

Scaffold a new project from the CF Framework template.

## Installation

Requires Rust and Git.

```bash
cargo install --git https://github.com/Crypto-Foundation-Dev/cf-framework cf-cli
```

## Usage

```bash
cf new my-app
cd my-app
# fill in DATABASE_* in .env
cargo run -p cf-api
```

`cf new` clones the latest template, renames the project in the deployment files and docs, creates `.env` from `.env.example`, and runs `git init`.

Project names use lowercase letters, digits, and hyphens, and start with a letter, because the name is also used for Kubernetes resources.
