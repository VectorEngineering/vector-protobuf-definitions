# Vector API CLI

A command-line tool for generating TypeScript API clients and Cloudflare Workers from OpenAPI specifications.

## Installation

```bash
# Install dependencies
pnpm install

# Build the CLI
pnpm run build
```

## Usage

### Generate API Client and Worker

You can run the CLI in several ways:

1. Using pnpm (recommended during development):
```bash
pnpm run cli generate \
  -i path/to/openapi.yaml \
  -o ./generated \
  -t ./templates
```

2. Using the global command (after installation):
```bash
vector generate \
  -i path/to/openapi.yaml \
  -o ./generated \
  -t ./templates
```

### Options

- `-i, --input <path>`: Path to OpenAPI specification file (required)
- `-o, --output <path>`: Output directory for generated code (required)
- `-t, --template <path>`: Path to custom templates directory (optional, defaults to built-in templates)

### Example with Sample OpenAPI

1. Create a sample OpenAPI specification file `openapi.yaml`:
```yaml
openapi: 3.0.0
info:
  title: Sample API
  version: 1.0.0
  description: A sample API for testing the Vector CLI
servers:
  - url: https://api.example.com/v1
paths:
  /users:
    get:
      summary: List users
      responses:
        '200':
          description: List of users
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/User'
    post:
      summary: Create user
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/CreateUser'
      responses:
        '201':
          description: User created
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
components:
  schemas:
    User:
      type: object
      properties:
        id:
          type: string
          format: uuid
        name:
          type: string
        email:
          type: string
          format: email
        createdAt:
          type: string
          format: date-time
      required:
        - id
        - name
        - email
        - createdAt
    CreateUser:
      type: object
      properties:
        name:
          type: string
        email:
          type: string
          format: email
      required:
        - name
        - email
```

2. Run the CLI:
```bash
# Create the output directory
mkdir -p generated

# Generate the API client
pnpm run cli generate -i openapi.yaml -o ./generated
```

This will generate a TypeScript API client with:
- Zod schemas for request/response validation
- Type-safe API client methods
- Cloudflare Worker integration

The generated code will be placed in the `generated` directory.

## Running the CLI

After building the project, there are several ways to run the CLI:

### 1. Global Installation (Recommended)

```bash
# Make the CLI executable
chmod +x dist/index.js

# Link the package globally
pnpm link --global

# Now you can use the vector command from anywhere
vector --help
```

If the `vector` command is not found after linking globally, you may need to add the global pnpm bin directory to your PATH. Add this line to your `~/.zshrc`:

```bash
export PATH="$PATH:$(pnpm -g bin)"
```

Then reload your shell:
```bash
source ~/.zshrc
```

### 2. Direct Execution

You can run the CLI directly from the dist directory:

```bash
./dist/index.js
```

### 3. Using pnpm

You can also run the CLI using pnpm:

```bash
pnpm vector
```

## Project Structure

```
.
├── src/
│   ├── index.ts           # CLI entry point
│   └── commands/
│       └── generate.ts    # Generate command implementation
├── templates/
│   ├── client.hbs        # API client template
│   └── worker.hbs        # Cloudflare Worker template
└── package.json
```

## Development

1. Clone the repository
2. Install dependencies: `pnpm install`
3. Build the project: `pnpm run build`
4. Run the CLI: `./dist/index.js`

## Custom Templates

You can create custom templates using Handlebars syntax. The following variables are available in templates:

- `zodSchemas`: Generated Zod schemas
- `apiClient`: Generated API client code
- `openApiSpec`: The parsed OpenAPI specification

## License

MIT 

## Troubleshooting

### SyntaxError: Invalid or unexpected token (Shebang issue)

If you encounter this error:
```
SyntaxError: Invalid or unexpected token
#!/usr/bin/env node
^
```

Try these solutions:

1. Rebuild the project with the correct ESM settings:
```bash
pnpm run build
```

2. If the error persists, run the CLI using node directly:
```bash
node dist/index.js
```
