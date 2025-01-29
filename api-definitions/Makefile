# Define targets for each service
.PHONY: gen fmt lint test precommit all

all: precommit

fmt:
	@echo "Formatting Go code..."
	go fmt ./...
	gofumpt -l -w .
	@echo "Organizing imports with gci..."
	gci write --skip-generated -s standard -s default .
	@echo "Formatting and organizing imports completed."

lint:
	@echo "Linting Go code..."
	golangci-lint run ./...
	@echo "Linting completed."

test:
	@echo "Deleting test databases..."
	./delete_test_db.sh
	@echo "Running tests..."
	set -o pipefail && go test -v ./...
	@echo "Testing completed."

# New gen target to incorporate the build process of different services
gen:
	@echo "Generating code for all services..."
	for service in lead-scraper-service; do \
		(cd api/$$service && make) || exit 1; \
	done
	@echo "Running additional generate script..."
	./generate.sh
	@echo "Generation completed."

precommit: gen fmt lint test
	@echo "Precommit checks completed."

# Additional custom targets if needed
clean:
	@echo "Cleaning up generated files..."
	for service in lead-scraper-service; do \
		(cd api/$$service && make clean) || exit 1; \
	done
	@echo "Clean completed."

build:
	@echo "Building all services..."
	for service in lead-scraper-service; do \
		(cd api/$$service && make build) || exit 1; \
	done
	@echo "Build completed."
