# Default target
.PHONY: all
all: format test

# Variables
IMAGE_NAME = scraping-service
IMAGE_TAG = latest

# Install dependencies
.PHONY: install
install:
	pnpm install

# Run tests
.PHONY: test
test:
	pnpm test

# Format code
.PHONY: format
format:
	pnpm run format

# Clean up cache files
.PHONY: clean
clean:
	rm -rf node_modules
	rm -rf dist
	rm -rf coverage
	rm -rf .cache

# Check everything before committing
.PHONY: precommit
precommit: format test build docker-build

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all          : Run format, lint, and test"
	@echo "  install      : Install project dependencies"
	@echo "  test         : Run tests"
	@echo "  lint         : Run linting checks"
	@echo "  format       : Format code"
	@echo "  clean        : Remove cache and build files"
	@echo "  pre-commit   : Run format, lint, test, build and docker-build"
	@echo "  help         : Show this help message"
	@echo "Local Development:"
	@echo "  dev          : Run the API locally with Docker Compose"
	@echo "  dev-down     : Stop the local development environment"
	@echo "  swagger      : Open the Swagger UI in your browser"
	@echo "  dev-logs     : View the container logs"
	@echo "  dev-clean    : Clean up all local development resources"

# Development commands
lint:
	pnpm run lint

build:
	pnpm run build

# Docker commands
docker-build:
	docker build -t $(IMAGE_NAME):$(IMAGE_TAG) .

docker-run:
	docker run -p 3003:3003 $(IMAGE_NAME):$(IMAGE_TAG)

docker-push:
	docker push $(IMAGE_NAME):$(IMAGE_TAG)

##@ Helm

# Check for required binaries
HELM := $(shell command -v helm 2> /dev/null)

check-helm:
ifndef HELM
	$(error "helm is not installed. Please install it first: https://helm.sh/docs/intro/install/")
endif

.PHONY: helm-lint
helm-lint: check-helm ## Lint the Helm chart
	helm lint charts/scraping-service

.PHONY: helm-template
helm-template: check-helm ## Template the Helm chart to validate the rendered manifests
	helm template charts/scraping-service

.PHONY: helm-test
helm-test: check-helm helm-lint helm-template ## Run all Helm tests (lint and template)
	@echo "✅ Helm chart tests passed"

##@ Local Development

.PHONY: dev
dev: build-clean ## Run the API locally with Docker Compose
	docker-compose up --build

.PHONY: dev-down
dev-down: ## Stop the local development environment
	docker-compose down

.PHONY: swagger
swagger: ## Open the Swagger UI in the default browser
	@echo "Opening Swagger UI..."
ifeq ($(shell uname),Darwin)
	open http://localhost:3003/api-docs
else ifeq ($(shell uname),Linux)
	xdg-open http://localhost:3003/api-docs
else
	@echo "Please open http://localhost:3003/api-docs in your browser"
endif

.PHONY: dev-logs
dev-logs: ## View the logs of the running containers
	docker-compose logs -f

.PHONY: dev-clean
dev-clean: dev-down ## Clean up all local development resources
	docker-compose rm -f
	docker volume prune -f

.PHONY: build-clean
build-clean: ## Clean and rebuild the application
	rm -rf dist
	pnpm run build 