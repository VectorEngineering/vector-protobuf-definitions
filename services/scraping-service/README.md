# Web Scraping Service

A robust web scraping microservice built with Playwright, Express, and TypeScript. This service provides a REST API endpoint for scraping web pages with configurable options.

## Features

- 🚀 Fast and reliable web scraping
- 🔄 Auto-reload in development
- 📄 Swagger documentation
- 🔒 Proxy support
- 🚫 Ad blocking
- 🖼️ Optional media blocking
- 🔧 Configurable timeouts and wait times
- 🛠️ Comprehensive development tooling
- ⚓ Helm chart support for Kubernetes deployment

## Prerequisites

- Docker
- Docker Compose
- Make (optional, for convenience commands)
- Node.js with PNPM (for local development)
- Helm (for Kubernetes deployment)

## Quick Start

1. Clone the repository:
```bash
git clone <repository-url>
cd scraping-service
```

2. Create a `.env` file:
```env
PORT=3003
BLOCK_MEDIA=false
PROXY_SERVER=
PROXY_USERNAME=
PROXY_PASSWORD=
```

3. Start the service:
```bash
make dev
```

The service will be available at `http://localhost:3003`

## API Usage

### Scrape Endpoint

```http
POST /scrape
Content-Type: application/json

{
    "url": "https://example.com",
    "timeout": 30000,
    "wait_after_load": 0,
    "headers": {},
    "check_selector": ""
}
```

#### Parameters

- `url` (required): The URL to scrape
- `timeout` (optional): Maximum time to wait for page load (in ms, default: 30000)
- `wait_after_load` (optional): Additional wait time after page load (in ms, default: 0)
- `headers` (optional): Custom headers to send with request
- `check_selector` (optional): CSS selector to wait for before returning content

#### Example Request

```bash
curl -X POST http://localhost:3003/scrape \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://github.com",
    "timeout": 30000
  }'
```

## Development

### Available Make Commands

#### Core Commands
- `make install` - Install project dependencies
- `make build` - Build the application
- `make test` - Run tests
- `make format` - Format code
- `make lint` - Run linting checks
- `make clean` - Remove cache and build files
- `make precommit` - Run format, lint, test, build and docker-build

#### Docker Commands
- `make docker-build` - Build Docker image
- `make docker-run` - Run Docker container
- `make docker-push` - Push Docker image to registry

#### Local Development
- `make dev` - Run the API locally with Docker Compose and auto-reload
- `make dev-down` - Stop the local development environment
- `make swagger` - Open Swagger UI in browser
- `make dev-logs` - View container logs
- `make dev-clean` - Clean up all local development resources
- `make build-clean` - Clean and rebuild the application

#### Helm Commands
- `make helm-lint` - Lint the Helm chart
- `make helm-template` - Template the Helm chart
- `make helm-test` - Run all Helm tests

### Running in Development Mode

The service includes auto-reload capability using nodemon:

```bash
make dev
```

### API Documentation

Swagger UI is available at:
```
http://localhost:3003/api-docs
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| PORT | Server port | 3003 |
| BLOCK_MEDIA | Block media resources | false |
| PROXY_SERVER | Proxy server URL | |
| PROXY_USERNAME | Proxy username | |
| PROXY_PASSWORD | Proxy password | |

## Docker Configuration

The service uses a multi-stage build process and runs as a non-root user for security. The container includes:

- Playwright with Chromium
- Node.js
- PNPM package manager
- Nodemon for development

## Helm Deployment

The service includes a Helm chart for Kubernetes deployment. To use it:

1. Ensure Helm is installed
2. Validate the chart:
```bash
make helm-test
```
3. Deploy using Helm:
```bash
helm install scraping-service charts/scraping-service
```

## Contributing

1. Fork the repository
2. Create your feature branch
3. Run `make precommit` to ensure everything is working
4. Submit a pull request

## License

[Add your license information here]