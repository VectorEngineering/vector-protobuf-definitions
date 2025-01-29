# Makefile for the api-gateway service

# Version control and build settings
TAG?=latest
NAME:=api-gateway
DOCKER_REPOSITORY:=solomonai
DOCKER_IMAGE_NAME:=$(DOCKER_REPOSITORY)/$(NAME)
GIT_COMMIT:=$(shell git describe --dirty --always)
VERSION?=0.1.0
CURRENT_TIME:=$(shell date -u "+%Y-%m-%dT%H:%M:%SZ")
EXTRA_RUN_ARGS?=

# Helm docs settings
HELM_DOCS_VERSION ?= 1.11.0
HELM_DOCS := $(shell command -v helm-docs 2> /dev/null)
CHART_DIR ?= charts
CHART_DOCS_CONFIG ?= charts/.helm-docs.yaml
HELM_DOCS_TEMPLATE ?= charts/.helm-docs.gotmpl

# Docker Compose configurations
TEMPORAL_DC=./compose/temporal/docker-compose.yml
CLICKHOUSE_DC=./compose/docker-compose.clickhouse.yaml
NGROK_DC=./compose/docker-compose-ngrok.yaml
DC=docker-compose -f docker-compose.yaml -f $(TEMPORAL_DC) -f $(CLICKHOUSE_DC) -f $(NGROK_DC)

# Build flags
LDFLAGS=-s -w \
	-X github.com/SolomonAIEngineering/backend-monorepo/src/services/backend-api-gateway/pkg/version.REVISION=$(GIT_COMMIT) \
	-X github.com/SolomonAIEngineering/backend-monorepo/src/services/backend-api-gateway/pkg/version.BuildTime=$(CURRENT_TIME)

# Go build settings
GO_BUILD_FLAGS=-trimpath
GOFLAGS=-mod=readonly
SHELL=/bin/bash -o pipefail

# Required tools validation
REQUIRED_TOOLS := git go docker helm kubectl helm-docs
$(foreach tool,$(REQUIRED_TOOLS),\
    $(if $(shell command -v $(tool) 2> /dev/null),$(info Found $(tool)),$(error Please install $(tool))))

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
RED := \033[0;31m
YELLOW := \033[1;33m
NC := \033[0m

validate: lint-gateway-configs validate-manifests

lint-gateway-configs:
	krakend check -tlc krakend.json

validate-manifests:
	./scripts/validate.sh
	./scripts/validate-chart.sh

gen:
	FC_ENABLE=1 \
	FC_SETTINGS="config/settings" \
	FC_PARTIALS="config/partials/prod" \
	FC_TEMPLATES="config/templates" \
	FC_OUT=$(OUTPUT_FILE_NAME) \
	AUTH_DOMAIN=solomon-ai.us.auth0.com \
	NUM_PODS=1 \
	AUTH_AUDIENCE=https://solomon-ai.us.auth0.com/api/v2/ \
	krakend check -t -ddd -c "config/krakend.tmpl"

lint-output:
	krakend check -tlc $(OUTPUT_FILE_NAME)

prettiefy:
	cat $(OUTPUT_FILE_NAME) | jq >> krakend.compiled.json
	mv krakend.compiled.json krakend.json
	rm $(OUTPUT_FILE_NAME)

swagger:
	rm -f api.swagger.json
	cd ./postman-generator && go build ./cmd/krakend-postman/main.go &&  ./main -c ../krakend.json >> ../api.swagger.json

precommit: validate gen lint-output prettiefy swagger lint helm-docs
	@echo "$(GREEN)Pre-commit checks completed successfully$(NC)"

sync-kustomize:
	./scripts/sync-kustomize.sh

helm-docs: ## Generate helm documentation
	$(call task_header,Generating Helm documentation)
	@if ! command -v helm-docs >/dev/null 2>&1; then \
		echo "$(RED)helm-docs is not installed. Installing...$(NC)"; \
		$(MAKE) install-helm-docs; \
	fi
	@echo "$(BLUE)Generating Helm documentation...$(NC)"
	@helm-docs --chart-search-root $(CHART_DIR) \
		$(if $(wildcard $(CHART_DOCS_CONFIG)),--config $(CHART_DOCS_CONFIG)) \
		$(if $(wildcard $(HELM_DOCS_TEMPLATE)),--template-files $(HELM_DOCS_TEMPLATE))
	@echo "$(GREEN)Helm documentation generated successfully$(NC)"

# Docker commands
.PHONY: docker-build docker-push docker-all docker-lint docker-run docker-run-d docker-stop docker-logs docker-shell

docker-lint: ## Lint the Dockerfile
	@echo "$(BLUE)Linting Dockerfile$(NC)"
	docker run --rm -i hadolint/hadolint < Dockerfile
	@echo "$(GREEN)Dockerfile lint complete$(NC)"

docker-build: ## Build the Docker image
	@echo "$(BLUE)Building Docker image $(DOCKER_IMAGE_NAME):$(TAG)$(NC)"
	@if [ -z "$(AUTH_DOMAIN)" ] || [ -z "$(AUTH_AUDIENCE)" ]; then \
		echo "$(RED)Error: AUTH_DOMAIN and AUTH_AUDIENCE must be set$(NC)"; \
		echo "Usage: AUTH_DOMAIN=domain AUTH_AUDIENCE=audience make docker-build"; \
		exit 1; \
	fi
	@echo "$(AUTH_DOMAIN)" > auth_domain.secret
	@echo "$(AUTH_AUDIENCE)" > auth_audience.secret
	docker build -t $(DOCKER_IMAGE_NAME):$(TAG) \
		--build-arg GIT_COMMIT=$(GIT_COMMIT) \
		--build-arg VERSION=$(VERSION) \
		--build-arg BUILD_TIME=$(CURRENT_TIME) \
		--secret id=auth_domain,src=auth_domain.secret \
		--secret id=auth_audience,src=auth_audience.secret \
		.
	@rm -f auth_domain.secret auth_audience.secret
	@echo "$(GREEN)Docker image built successfully$(NC)"

docker-run: ## Run the Docker container in interactive mode
	@echo "$(BLUE)Running Docker container $(DOCKER_IMAGE_NAME):$(TAG)$(NC)"
	docker run --rm -it \
		-p 8000:8080 \
		--name $(NAME) \
		-e AUTH_DOMAIN=$(AUTH_DOMAIN) \
		-e AUTH_AUDIENCE=$(AUTH_AUDIENCE) \
		$(EXTRA_RUN_ARGS) \
		$(DOCKER_IMAGE_NAME):$(TAG)

docker-run-d: ## Run the Docker container in detached mode
	@echo "$(BLUE)Running Docker container in background $(DOCKER_IMAGE_NAME):$(TAG)$(NC)"
	docker run -d \
		-p 8000:8080 \
		--name $(NAME) \
		-e AUTH_DOMAIN=$(AUTH_DOMAIN) \
		-e AUTH_AUDIENCE=$(AUTH_AUDIENCE) \
		$(EXTRA_RUN_ARGS) \
		$(DOCKER_IMAGE_NAME):$(TAG)
	@echo "$(GREEN)Container started. Use 'make docker-logs' to view logs$(NC)"

docker-stop: ## Stop the running Docker container
	@echo "$(BLUE)Stopping Docker container$(NC)"
	docker stop $(NAME) || true
	docker rm $(NAME) 2>/dev/null || true
	@echo "$(GREEN)Container stopped$(NC)"

docker-logs: ## View Docker container logs
	@echo "$(BLUE)Viewing logs for container $(NAME)$(NC)"
	docker logs -f $(NAME)

docker-shell: ## Get a shell inside the running container
	@echo "$(BLUE)Opening shell in container $(NAME)$(NC)"
	docker exec -it $(NAME) /bin/sh

docker-ps: ## List running containers
	@echo "$(BLUE)Listing running containers$(NC)"
	docker ps --filter name=$(NAME)

docker-all: docker-stop docker-build docker-run-d ## Build and run Docker container in background

# Development commands
dev-build: ## Build for local development
	@echo "$(BLUE)Building for local development$(NC)"
	go build $(GO_BUILD_FLAGS) -ldflags "$(LDFLAGS)" -o bin/$(NAME) ./cmd/$(NAME)
	@echo "$(GREEN)Build complete$(NC)"

# Cleanup command
clean: ## Clean up temporary files
	rm -f auth_domain.secret auth_audience.secret
	@echo "$(GREEN)Cleanup complete$(NC)"

# Add help target to show available commands
help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Single command for Docker development
docker-dev: ## Build and run the Docker container with all necessary setup
	@echo "$(BLUE)Starting Docker development environment$(NC)"
	@if [ -z "$(AUTH_DOMAIN)" ] || [ -z "$(AUTH_AUDIENCE)" ]; then \
		echo "$(RED)Error: AUTH_DOMAIN and AUTH_AUDIENCE must be set$(NC)"; \
		echo "Usage: AUTH_DOMAIN=domain AUTH_AUDIENCE=audience make docker-dev"; \
		exit 1; \
	fi
	@echo "$(BLUE)1. Stopping any existing containers...$(NC)"
	@make docker-stop >/dev/null 2>&1 || true
	@echo "$(BLUE)2. Building Docker image...$(NC)"
	@make docker-build
	@echo "$(BLUE)3. Starting container in background...$(NC)"
	@make docker-run-d
	@echo "$(BLUE)4. Following logs (Ctrl+C to exit logs, container will keep running)...$(NC)"
	@sleep 2
	@make docker-logs

# Test settings
TEST_FLAGS ?= -v -race
TEST_TIMEOUT ?= 5m

# Test commands
.PHONY: test test-coverage test-postman

test: ## Run all tests
	@echo "$(BLUE)Running tests...$(NC)"
	go test $(TEST_FLAGS) -timeout $(TEST_TIMEOUT) ./...

test-coverage: ## Run tests with coverage
	@echo "$(BLUE)Running tests with coverage...$(NC)"
	go test $(TEST_FLAGS) -timeout $(TEST_TIMEOUT) -coverprofile=coverage.out ./...
	go tool cover -html=coverage.out -o coverage.html
	@echo "$(GREEN)Coverage report generated in coverage.html$(NC)"

test-postman: ## Run postman generator tests
	@echo "$(BLUE)Running postman generator tests...$(NC)"
	cd postman-generator && go test $(TEST_FLAGS) -timeout $(TEST_TIMEOUT)

install-deps: ## Install test dependencies
	@echo "$(BLUE)Installing test dependencies...$(NC)"
	go get github.com/stretchr/testify/assert
	go mod tidy
