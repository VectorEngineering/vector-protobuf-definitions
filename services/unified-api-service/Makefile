# Makefile for Vector Unified API

# Variables
NODE_ENV ?= development
PNPM = pnpm
NEST = $(PNPM) nest
DOCKER = docker
DOCKER_COMPOSE = docker compose
IMAGE_NAME = vector-unified-api
IMAGE_TAG ?= latest

# Colors for terminal output
CYAN = \033[0;36m
GREEN = \033[0;32m
RESET = \033[0m

.PHONY: help install build start dev test lint format clean ci validate-connectors prebuild-oauth docker-build docker-push docker-run docker-stop docker-clean docker-compose-up docker-compose-down helm-lint helm-template helm-validate helm-test helm-sanitize

help: ## Display this help message
	@echo "$(CYAN)Vector Unified API Makefile Commands:$(RESET)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(CYAN)%-30s$(RESET) %s\n", $$1, $$2}'

install: ## Install all dependencies
	@echo "$(GREEN)Installing dependencies...$(RESET)"
	$(PNPM) install

build: ## Build the application
	@echo "$(GREEN)Building the application...$(RESET)"
	./scripts/pull-database.sh && $(NEST) build

start: ## Start the application in production mode
	@echo "$(GREEN)Starting the application in production mode...$(RESET)"
	$(PNPM) start:prod

dev: ## Start the application in development mode
	@echo "$(GREEN)Starting the application in development mode...$(RESET)"
	$(PNPM) start:dev

debug: ## Start the application in debug mode
	@echo "$(GREEN)Starting the application in debug mode...$(RESET)"
	$(PNPM) start:debug

test: ## Run tests
	@echo "$(GREEN)Running tests...$(RESET)"
	$(PNPM) test

test-watch: ## Run tests in watch mode
	@echo "$(GREEN)Running tests in watch mode...$(RESET)"
	$(PNPM) test:watch

test-coverage: ## Run tests with coverage
	@echo "$(GREEN)Running tests with coverage...$(RESET)"
	$(PNPM) test:cov

test-e2e: ## Run end-to-end tests
	@echo "$(GREEN)Running end-to-end tests...$(RESET)"
	$(PNPM) test:e2e

lint: ## Run linter
	@echo "$(GREEN)Running linter...$(RESET)"
	$(PNPM) lint

format: ## Format code
	@echo "$(GREEN)Formatting code...$(RESET)"
	$(PNPM) format

clean: ## Clean build artifacts
	@echo "$(GREEN)Cleaning build artifacts...$(RESET)"
	rm -rf dist
	rm -rf node_modules

ci: ## Run CI tasks
	@echo "$(GREEN)Running CI tasks...$(RESET)"
	$(PNPM) ci

validate-connectors: ## Validate connectors
	@echo "$(GREEN)Validating connectors...$(RESET)"
	$(PNPM) validate-connectors

prebuild-oauth: ## Prebuild OAuth connector
	@echo "$(GREEN)Prebuilding OAuth connector...$(RESET)"
	$(PNPM) prebuild-oauth-connector

# Docker commands
docker-build: ## Build Docker image
	@echo "$(GREEN)Building Docker image...$(RESET)"
	$(DOCKER) build -t $(IMAGE_NAME):$(IMAGE_TAG) .

docker-build-dev: ## Build development Docker image
	@echo "$(GREEN)Building development Docker image...$(RESET)"
	$(DOCKER) build -t $(IMAGE_NAME)-dev:$(IMAGE_TAG) -f Dockerfile.dev .

docker-build-lint: ## Build linter Docker image
	@echo "$(GREEN)Building linter Docker image...$(RESET)"
	$(DOCKER) build -t package_linter -f Dockerfile.pnpm-lint .

docker-build-builder: ## Build builder Docker image
	@echo "$(GREEN)Building builder Docker image...$(RESET)"
	$(DOCKER) build -t package_builder -f Dockerfile.pnpm-build .

docker-build-installer: ## Build installer Docker image
	@echo "$(GREEN)Building installer Docker image...$(RESET)"
	$(DOCKER) build -t package_installer -f Dockerfile.pnpm-installer .

docker-build-validator: ## Build connector validator Docker image
	@echo "$(GREEN)Building connector validator Docker image...$(RESET)"
	$(DOCKER) build -t validate_connectors -f Dockerfile.validate-connectors .

docker-build-all: docker-build docker-build-dev docker-build-lint docker-build-builder docker-build-installer docker-build-validator ## Build all Docker images

docker-push: ## Push Docker image to registry
	@echo "$(GREEN)Pushing Docker image to registry...$(RESET)"
	$(DOCKER) push $(IMAGE_NAME):$(IMAGE_TAG)

docker-run: ## Run Docker container
	@echo "$(GREEN)Running Docker container...$(RESET)"
	$(DOCKER) run -d -p 3000:3000 --name $(IMAGE_NAME) $(IMAGE_NAME):$(IMAGE_TAG)

docker-run-dev: ## Run development Docker container
	@echo "$(GREEN)Running development Docker container...$(RESET)"
	$(DOCKER) run -v $(PWD):/app/ -p 3000:3000 $(IMAGE_NAME)-dev:$(IMAGE_TAG)

docker-run-lint: ## Run linter Docker container
	@echo "$(GREEN)Running linter Docker container...$(RESET)"
	$(DOCKER) run -v $(PWD):/app/ package_linter

docker-run-build: ## Run builder Docker container
	@echo "$(GREEN)Running builder Docker container...$(RESET)"
	$(DOCKER) run -v $(PWD):/app/ package_builder

docker-run-installer: ## Run installer Docker container (requires PACKAGE_NAME env var)
	@echo "$(GREEN)Running installer Docker container...$(RESET)"
	$(DOCKER) run -v $(PWD):/app/ -e PACKAGE_NAME=$(PACKAGE_NAME) package_installer

docker-run-validator: ## Run connector validator Docker container (requires VERTICAL and OBJECT_TYPE env vars)
	@echo "$(GREEN)Running connector validator Docker container...$(RESET)"
	$(DOCKER) run -v $(PWD):/app/ -e VERTICAL=$(VERTICAL) -e OBJECT_TYPE=$(OBJECT_TYPE) validate_connectors

docker-stop: ## Stop Docker container
	@echo "$(GREEN)Stopping Docker container...$(RESET)"
	$(DOCKER) stop $(IMAGE_NAME) || true
	$(DOCKER) rm $(IMAGE_NAME) || true

docker-clean: ## Remove Docker image
	@echo "$(GREEN)Removing Docker image...$(RESET)"
	$(DOCKER) rmi $(IMAGE_NAME):$(IMAGE_TAG) || true

docker-compose-up: ## Start all services with Docker Compose
	@echo "$(GREEN)Starting services with Docker Compose...$(RESET)"
	$(DOCKER_COMPOSE) up -d

docker-compose-down: ## Stop all services with Docker Compose
	@echo "$(GREEN)Stopping services with Docker Compose...$(RESET)"
	$(DOCKER_COMPOSE) down

# Development workflow commands
format-check: ## Check code formatting
	@echo "$(GREEN)Checking code formatting...$(RESET)"
	$(PNPM) prettier --check "**/*.{ts,js,json,yml,yaml,md}"

format-fix: ## Fix code formatting
	@echo "$(GREEN)Fixing code formatting...$(RESET)"
	$(PNPM) prettier --write "**/*.{ts,js,json,yml,yaml,md}"

lint-check: ## Check code linting
	@echo "$(GREEN)Checking code linting...$(RESET)"
	$(PNPM) eslint "src/**/*.{js,ts}" "test/**/*.{js,ts}" --max-warnings=0

lint-fix: ## Fix code linting issues
	@echo "$(GREEN)Fixing code linting issues...$(RESET)"
	$(PNPM) eslint "src/**/*.{js,ts}" "test/**/*.{js,ts}" --fix

test-unit: ## Run unit tests
	@echo "$(GREEN)Running unit tests...$(RESET)"
	$(PNPM) test

test-e2e: ## Run end-to-end tests
	@echo "$(GREEN)Running end-to-end tests...$(RESET)"
	$(PNPM) test:e2e

test-coverage: ## Run tests with coverage
	@echo "$(GREEN)Running tests with coverage...$(RESET)"
	$(PNPM) test:cov

test-all: test-unit test-e2e test-coverage ## Run all tests

build-clean: ## Clean build artifacts
	@echo "$(GREEN)Cleaning build artifacts...$(RESET)"
	rm -rf dist

build-deps: ## Install dependencies
	@echo "$(GREEN)Installing dependencies...$(RESET)"
	$(PNPM) install

build-prisma: ## Generate Prisma client
	@echo "$(GREEN)Generating Prisma client...$(RESET)"
	$(PNPM) prisma generate

build-app: ## Build the application
	@echo "$(GREEN)Building the application...$(RESET)"
	$(PNPM) build

build: build-clean build-deps build-prisma build-app ## Full build process

# Combined workflows
check: format-check lint-check test-unit ## Run all checks (format, lint, test)
fix: format-fix lint-fix ## Fix formatting and linting issues
verify: check build ## Full verification (check and build)

# All-in-one command
all: fix verify test-all docker-build-all ## Run everything (fix, verify, test-all, docker-build-all)
	@echo "$(GREEN)All tasks completed successfully!$(RESET)"

precommit: format-fix format-check build test-unit ## Run all checks (format, lint, test)
	@echo "$(GREEN)Precommit tasks completed successfully!$(RESET)"

# Helm Chart Sanitization Commands
HELM_CHART_DIR := charts/unified-api-service

helm-lint:
	@echo "🔍 Linting Helm chart..."
	helm lint $(HELM_CHART_DIR)

helm-template:
	@echo "📄 Validating Helm template rendering..."
	helm template $(HELM_CHART_DIR) --debug

helm-validate:
	@echo "✅ Validating Helm chart..."
	helm lint $(HELM_CHART_DIR)
	helm template $(HELM_CHART_DIR) | kubectl apply --dry-run=client -f -

helm-test:
	@echo "🧪 Testing Helm chart..."
	helm install --dry-run --debug test-release $(HELM_CHART_DIR)

helm-sanitize: helm-lint helm-template helm-validate helm-test
	@echo "✨ Helm chart sanitization complete"

# Add this to your existing check/test commands if you have any
check: helm-sanitize
test: helm-sanitize

# Default target
.DEFAULT_GOAL := help 