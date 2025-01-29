# Vector Protocol Buffer Definitions

[![Build Status][build-img]][build-url]
[![Go Report Card][go-report-img]][go-report-url]
[![License][license-img]][license-url]
[![Release][release-img]][release-url]

> Central repository for Vector's API definitions, SDKs, and service specifications

## Overview

This repository serves as the single source of truth for Vector's microservices ecosystem, containing comprehensive Protocol Buffer definitions, generated SDKs, and API specifications. It enables seamless integration and consistent API development across Vector's services.

## Key Components

- **Protocol Buffer Definitions**: Core service interface definitions
- **Generated SDKs**: Official client libraries in multiple languages
- **API Gateway Configs**: KrakenD-based API gateway configurations
- **OpenAPI Specs**: Swagger/OpenAPI 3.0 specifications
- **Service Documentation**: Comprehensive API documentation

## Available SDKs

| Language   | Package Name                        | Version | Documentation |
|------------|-------------------------------------|---------|---------------|
| TypeScript | @playbookmedia/backend-node-sdk     | 1.0.0   | [Docs](sdk/client/client-node-sdk/README.md) |
| Python     | vector-python-client                | 1.0.0   | [Docs](sdk/client/python-client-sdk/README.md) |
| Go         | github.com/vector/vector-go-client  | 1.0.0   | [Docs](sdk/client/go-client-sdk/README.md) |

## Project Structure

```
.
├── api-definitions/     # Protocol Buffer definitions and generated code
│   ├── proto/          # Raw protobuf files
│   └── gen/            # Generated protobuf code
├── documentation/      # Auto-generated API documentation
├── krakend-config/    # KrakenD API Gateway configurations
├── sdk/               # Generated SDKs for various languages
│   ├── client/        # Client SDKs
│   └── server/        # Server implementations
├── services/         # Service-specific configurations
├── swagger/          # OpenAPI/Swagger specifications
└── scripts/          # Utility scripts for development
```

## Prerequisites

- Go 1.x or higher
- Protocol Buffer Compiler (protoc) 3.x
- Node.js 12.x or higher
- OpenAPI Generator CLI
- KrakenD
- Make

## Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/VectorEngineering/vector-protobuf-definitions.git
   cd vector-protobuf-definitions
   ```

2. Install dependencies:
   ```bash
   make setup
   ```

3. Generate all artifacts:
   ```bash
   make gen
   ```

## Development Commands

```bash
# Generate complete SDK suite
make autogen

# Update API documentation
make update-docs

# Generate specific SDK
make update-typescript-client-sdk
make update-python-client-sdk
make update-go-client-sdk

# Generate API gateway config
make generate-krakend-config

# Run tests
make test

# Show all available commands
make help
```

## API Documentation

Our API documentation is available in multiple formats:

- **OpenAPI/Swagger**: `swagger/` directory
- **Markdown**: `documentation/` directory
- **Protobuf**: `api-definitions/proto/` directory

Visit our [documentation site](https://docs.vector.engineering) for the most up-to-date API reference.

## Contributing

We welcome contributions! Please follow our guidelines:

1. Read our [Contributing Guide](CONTRIBUTING.md)
2. Follow our [Code Style Guide](STYLE.md)
3. Understand our [API Versioning](API_VERSIONING.md)
4. Review our [PR Process](PULL_REQUEST_REVIEW.md)

## Support

- 📧 Email: yoanyomba@solomon-ai.co
- 📚 Documentation: [docs.vector.engineering](https://docs.vector.engineering)
- 🐛 Issue Tracker: [GitHub Issues](https://github.com/VectorEngineering/vector-protobuf-definitions/issues)

## License

MIT © [Vector Engineering]

[build-img]: https://github.com/VectorEngineering/vector-protobuf-definitions/actions/workflows/main.yml/badge.svg
[build-url]: https://github.com/VectorEngineering/vector-protobuf-definitions/actions/workflows/main.yml
[go-report-img]: https://goreportcard.com/badge/github.com/VectorEngineering/vector-protobuf-definitions
[go-report-url]: https://goreportcard.com/report/github.com/VectorEngineering/vector-protobuf-definitions
[license-img]: https://img.shields.io/github/license/VectorEngineering/vector-protobuf-definitions
[license-url]: https://github.com/VectorEngineering/vector-protobuf-definitions/blob/main/LICENSE
[release-img]: https://img.shields.io/github/v/release/VectorEngineering/vector-protobuf-definitions
[release-url]: https://github.com/VectorEngineering/vector-protobuf-definitions/releases