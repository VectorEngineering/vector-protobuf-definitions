# Vector Protocol Buffer Definitions

This repository contains Protocol Buffer definitions, generated code, and API specifications for Vector's microservices ecosystem, with a primary focus on the Lead Scraper Service and related components.

## Overview

The repository serves as a central location for:
- Protocol Buffer (protobuf) definitions
- OpenAPI/Swagger specifications
- Generated SDKs for multiple languages
- API Gateway configurations
- Service documentation

## Project Structure

```
.
├── api-definitions/      # Protocol Buffer definitions and generated code
├── documentation/       # Auto-generated API documentation
├── krakend-config/     # KrakenD API Gateway configurations
├── sdk/                # Generated SDKs for various languages
├── services/          # Service-specific configurations
├── swagger/           # OpenAPI/Swagger specifications
└── scripts/           # Utility scripts for development
```

## Features

- Multi-language SDK generation (TypeScript, Node.js, Java, Rust)
- OpenAPI v3 specification generation
- Automated documentation generation
- API Gateway configuration management
- Comprehensive protobuf definitions

## Development

### Prerequisites

- Go 1.x
- Protocol Buffer Compiler (protoc)
- Node.js and npm (for TypeScript/Node.js SDK generation)
- OpenAPI Generator CLI
- KrakenD

### Available Commands

```bash
# Generate all API definitions and related files
make gen

# Generate complete SDK suite and documentation
make autogen

# Generate API documentation
make update-docs

# Generate TypeScript client SDK
make update-typescript-client-sdk

# Generate KrakenD configurations
make generate-krakend-config
```

For a complete list of available commands, run:
```bash
make help
```

## Documentation

- API documentation is automatically generated in multiple formats:
  - OpenAPI YAML
  - Markdown
  - Postman Collection

## Contributing

Please refer to the following documents for contribution guidelines:
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [STYLE.md](STYLE.md) - Code style guide
- [API_VERSIONING.md](API_VERSIONING.md) - API versioning guidelines
- [PULL_REQUEST_REVIEW.md](PULL_REQUEST_REVIEW.md) - PR review process

## License

[Add appropriate license information]