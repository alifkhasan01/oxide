# Oxide

> A modern Rust CLI for generating, developing, and managing backend projects.

Oxide is a Rust-based CLI designed to simplify backend development by providing a unified workflow for creating projects, selecting technology stacks, generating resources, managing development services, and running common development tasks.

## Features

- 🚀 **Quick Project Creation** - Create a new Rust backend project with sensible defaults
- 🎯 **Interactive Wizard** - Choose your framework, database, auth, and cache
- 🔧 **Feature Management** - Add features to existing projects
- 🛠️ **Resource Generation** - Generate CRUD operations and project structure
- 🐳 **Service Management** - Manage development services (PostgreSQL, Redis)
- ✅ **Built-in Checks** - Run cargo check, fmt, and clippy
- 📚 **Shell Completion** - Auto-complete for Bash, Zsh, Fish
- 🔍 **Project Diagnostics** - Check project health with doctor command
- 🔄 **Easy Upgrades** - Upgrade projects to latest version

## Installation

```bash
# Install from source
git clone https://github.com/nuswa/oxide.git
cd oxide
cargo install --path .
```

## Quick Start

```bash
# Create a new project
oxide create my-api

# Navigate to project
cd my-api

# Start development
oxide dev
```

## Commands

### Create

Create a new backend project:

```bash
oxide create my-api
```

With options:

```bash
oxide create my-api \
  --framework axum \
  --database postgres \
  --auth jwt \
  --cache redis \
  --non-interactive
```

### Development

Start the development server:

```bash
oxide dev
```

### Build

Build the project:

```bash
oxide build
oxide build --release
```

### Test

Run tests:

```bash
oxide test
oxide test --name "test_name"
```

### Check

Run all checks:

```bash
oxide check
oxide check --skip-clippy
```

### Generate

Generate a resource:

```bash
oxide generate resource user
oxide g resource user --crud
```

### Add

Add a feature to an existing project:

```bash
oxide add postgres
oxide add mysql
oxide add sqlite
oxide add redis
oxide add auth
oxide add cors
oxide add validation
oxide add openapi
oxide add websocket
oxide add ratelimit
```

### Services

Manage development services:

```bash
oxide services up
oxide services down
oxide services status
```

### Completion

Generate shell completion scripts:

```bash
oxide completion bash
oxide completion zsh
oxide completion fish
oxide completion powershell
oxide completion elvish
```

### Doctor

Check environment and project health:

```bash
oxide doctor
oxide doctor --project
```

### Upgrade

Upgrade project to latest Oxide version:

```bash
oxide upgrade
oxide upgrade --check
```

## Configuration

Oxide generates a `nexus.toml` file in your project root:

```toml
[project]
name = "my-api"
version = "0.1.0"
framework = "axum"
edition = "2021"

[database]
provider = "postgres"

[auth]
provider = "jwt"

[cache]
provider = "redis"

[tool.oxide]
version = "0.1.0"

[api]
cors = true
validation = true
openapi = true
websocket = true
ratelimit = true
```

## Default Stack

Oxide uses the following default stack:

- **Framework**: Axum
- **Runtime**: Tokio
- **Serialization**: Serde
- **Logging**: Tracing
- **Configuration**: dotenvy

Optional:
- **Database**: SQLx (PostgreSQL, MySQL, SQLite)
- **Cache**: Redis
- **Authentication**: JWT + Argon2
- **API Features**: CORS, Validation, OpenAPI, WebSocket, Rate Limiting

## Philosophy

> **Users choose capabilities. Oxide chooses implementation details.**

You don't need to understand the entire Rust ecosystem before creating a backend. Just tell Oxide what you need, and it handles the rest.

## License

MIT
