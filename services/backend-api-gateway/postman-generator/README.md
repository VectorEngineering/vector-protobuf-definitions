# Postman Generator

A robust Go package that automatically converts KrakenD API Gateway configurations into comprehensive Postman Collections. This tool streamlines the process of testing and documenting API endpoints by generating structured, ready-to-use Postman collections.

## Features

### Core Functionality
- Seamless conversion of KrakenD endpoints to Postman collection format
- Intelligent handling of path parameters with various complexity levels
- Automatic request body generation for POST, PUT, and PATCH methods
- Built-in Bearer token authentication integration
- Dynamic environment variable management
- Comprehensive TLS configuration support

### Advanced Capabilities
- Custom header management for each endpoint
- Response example generation
- Request parameter validation
- Batch conversion support
- Nested endpoint hierarchy preservation
- Environment-specific configurations

## Installation

### Prerequisites
- Go 1.16 or higher
- KrakenD configuration file
- Postman (for collection usage)

### Install via Go
```bash
go get github.com/Vector/vector-lead-management/services/backend-api-gateway/postman-generator
```

### Manual Installation
```bash
git clone https://github.com/Vector/vector-lead-management/services/backend-api-gateway/postman-generator
cd postman-generator
go build
```

## Usage

### Package Integration

```go
import (
    "github.com/luraproject/lura/v2/config"
    "github.com/Vector/vector-lead-management/services/backend-api-gateway/postman-generator"
)

func main() {
    // Create a KrakenD service configuration
    cfg := config.ServiceConfig{
        Port: 8080,
        Name: "my-api",
        Endpoints: []*config.EndpointConfig{
            {
                Endpoint: "/api/v1/users/{id}",
                Method:   "GET",
                Headers:  map[string]string{"Content-Type": "application/json"},
            },
            {
                Endpoint: "/api/v1/users",
                Method:   "POST",
                Headers:  map[string]string{"Content-Type": "application/json"},
            },
        },
    }

    // Generate Postman collection with additional options
    collection := postman.Parse(cfg, postman.Options{
        IncludeExamples: true,
        ValidateRequests: true,
        GroupByPrefix: true,
    })

    // Export the collection to a file
    if err := collection.Export("api_collection.json"); err != nil {
        log.Fatal(err)
    }
}
```

### Server Integration

```go
// Create an HTTP handler for collection generation
http.HandleFunc("/postman", postman.HandleCollection(collection))

// Start the server with custom options
server := &http.Server{
    Addr:         ":8080",
    ReadTimeout:  10 * time.Second,
    WriteTimeout: 10 * time.Second,
}
log.Fatal(server.ListenAndServe())
```

## Features in Detail

### Path Parameter Handling

The generator provides sophisticated handling of path parameters:

```plaintext
Basic Parameters:
- /users/{id}
- /products/{productId}

Nested Parameters:
- /users/{userId}/profile
- /organizations/{orgId}/teams/{teamId}

Complex Patterns:
- /api/v1/users/{userId}/posts/{postId}/comments/{commentId}
- /services/{serviceId}/versions/{versionId}/configurations
```

### Request Body Templates

Automatic request body generation for different HTTP methods:

```json
// POST /api/v1/users
{
    "username": "string",
    "email": "string",
    "profile": {
        "firstName": "string",
        "lastName": "string",
        "age": 0
    }
}

// PUT /api/v1/users/{id}
{
    "email": "string",
    "profile": {
        "age": 0
    }
}
```

### Authentication Configuration

Built-in support for various authentication methods:

```json
{
    "auth": {
        "type": "bearer",
        "bearer": {
            "token": "{{bearerToken}}"
        }
    }
}
```

### Environment Variables

Comprehensive environment variable management:

```json
{
    "variables": {
        "HOST": {
            "default": "localhost:8080",
            "description": "API host address"
        },
        "SCHEMA": {
            "default": "https",
            "description": "HTTP/HTTPS schema"
        },
        "API_VERSION": {
            "default": "v1",
            "description": "API version"
        }
    }
}
```

## Testing

### Running Tests

```bash
# Run all tests
go test -v ./...

# Run specific test suite
go test -v ./internal/converter/...

# Run tests with race detection
go test -race ./...
```

### Makefile Commands

```bash
# Run all postman generator tests
make test-postman

# Generate coverage report
make test-coverage

# Run integration tests
make test-integration

# Run all tests and generate reports
make test-all
```

## Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Go best practices and idioms
- Maintain test coverage above 80%
- Update documentation for new features
- Add examples for new functionality
- Follow semantic versioning

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- GitHub Issues: For bug reports and feature requests
- Documentation: Comprehensive guides and API reference
- Community: Join our Discord channel for discussions