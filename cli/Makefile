.PHONY: run build generate test clean

# Default target
all: clean generate

# Clean generated files
clean:
	rm -rf generated

# Build the CLI
build:
	pnpm install
	pnpm run build

# Generate API client from OpenAPI spec
generate:
	pnpm run cli generate -i ../swagger/backend-api.yaml -o ./generated

# Generate Cloudflare Worker
worker:
	pnpm run cli generate \
		-i ../swagger/backend-api.yaml \
		-o ../services/public-api-gateway/src/generated \
		-t ./templates

# Run tests
test:
	pnpm run test

# Development mode
dev:
	pnpm run dev

run:
	rm -rf generated && pnpm run cli generate -i openapi.yaml -o ./generated && cd generated && pnpm run format && pnpm run build && cd ..
