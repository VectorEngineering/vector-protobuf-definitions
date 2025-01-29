SWAGGER_WORKSPACE_SERVICE_CODEGEN_PATH:=./api-definitions/pkg/generated/lead_scraper_service/v1/apidocs.swagger.json

VERSION = 3
FILE_PROD = ./krakend-config/final-krakend.prod.json
FILE_STAGING = ./krakend-config/final-krakend.staging.json
export JAVA_OPTS=-DmaxYamlCodePoints=99999999 -Dsnakeyaml.maxaliases=2147483647

CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RESET := \033[0m

.PHONY: help
help: ## Show this help message
	@echo '${CYAN}Usage:${RESET}'
	@echo '  make ${GREEN}<target>${RESET}'
	@echo ''
	@echo '${CYAN}Targets:${RESET}'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  ${GREEN}%-30s${RESET} %s\n", $$1, $$2}'

gen: ## Generate API definitions
	cd api-definitions && make gen && cd ../

copy-swagger: ## Copy Swagger files to swagger directory
	cp -rf $(SWAGGER_WORKSPACE_SERVICE_CODEGEN_PATH) ./swagger/lead-scraper-service.json

convert-swagger-to-openapiv3: ## Convert Swagger files to OpenAPI v3 format
	swagger2openapi --yaml --outfile ./swagger/lead-scraper-service-backend-api.yaml ./swagger/lead-scraper-service.json

	swagger2openapi --outfile ./swagger/lead-scraper-service-backend-api.json ./swagger/lead-scraper-service.json

	npx openapi-merge-cli --config ./swagger/merge-config.json
	swagger2openapi --outfile ./swagger/backend-api.json ./swagger/backend-api.yaml

update-typescript-client-sdk: update-typescript-server-sdk ## Generate TypeScript client SDKs
	JAVA_OPTS="$(JAVA_OPTS)" openapi-generator-cli generate \
    	-i ./swagger/backend-api.yaml \
    	-g typescript-fetch -o ../sdk/client/client-typescript-sdk/src/typescript-fetch
	JAVA_OPTS="$(JAVA_OPTS)" openapi-generator-cli generate \
    	-i ./swagger/backend-api.yaml \
    	-g typescript-node \
      --additional-properties=legacyDiscriminatorBehavior=false,nullSafeAdditionalProps=true,disallowAdditionalPropertiesIfNotPresent=true \
      -o ../sdk/client/client-node-sdk/src/node-sdk

update-typescript-server-sdk: ## Generate server SDKs for Node.js, Java, and Rust
	JAVA_OPTS="$(JAVA_OPTS)" openapi-generator-cli generate \
    	-i ./swagger/backend-api.yaml \
    	-g graphql-nodejs-express-server -o ../sdk/server/node
	JAVA_OPTS="$(JAVA_OPTS)" openapi-generator-cli generate \
    	-i ./swagger/backend-api.yaml \
    	-g spring -o ../sdk/server/java
	JAVA_OPTS="$(JAVA_OPTS)" openapi-generator-cli generate \
    	-i ./swagger/backend-api.yaml \
    	-g rust-server -o ../sdk/server/rust

update-docs: ## Generate documentation in various formats
	JAVA_OPTS="$(JAVA_OPTS)" openapi-generator-cli generate \
    -i ./swagger/backend-api.yaml \
    -g openapi-yaml -o ./documentation/autogen/openapi-yaml

	JAVA_OPTS="$(JAVA_OPTS)" openapi-generator-cli generate \
		-i ./swagger/backend-api.yaml \
		-g markdown -o ./documentation/autogen/markdown

	JAVA_OPTS="$(JAVA_OPTS)" openapi-generator-cli generate \
		-i ./swagger/backend-api.yaml \
		-g postman-collection -o ./documentation/autogen/postman-collection

generate-krakend-config: ## Generate KrakenD configuration files
	mkdir temp-swagger && cp ./swagger/lead-scraper-service-backend-api.json ./temp-swagger/lead-scraper-service-backend-api.json
	go run ./openapi2krakend/pkg/main.go -directory ./temp-swagger -output ./krakend-config/krakend.prod.json -environment production -webhook-config-path ./webhooks.yaml
	go run ./openapi2krakend/pkg/main.go -directory ./temp-swagger -output ./krakend-config/krakend.staging.json -environment staging -webhook-config-path ./webhooks.yaml
	rm -rf temp-swagger

prettify-krakend: ## Format KrakenD configuration files
	cat ./krakend-config/krakend.json | jq >> ./krakend-config/krakend.compiled.json
	mv ./krakend-config/krakend.compiled.json ./krakend-config/krakend.json

merge-configs: ## Merge KrakenD configurations with gateway configurations
	jq -s '.[0] * .[1]' ./krakend-config/gateway-configurations.json ./krakend-config/krakend.prod.json > ./krakend-config/final-krakend.prod.json
	@sed -i '' 's/"version": "3"/"version": $(VERSION)/' $(FILE_PROD)
	jq -s '.[0] * .[1]' ./krakend-config/gateway-configurations.json ./krakend-config/krakend.staging.json > ./krakend-config/final-krakend.staging.json
	@sed -i '' 's/"version": "3"/"version": $(VERSION)/' $(FILE_STAGING)

validate: ## Validate KrakenD configuration files
	krakend check --config ./krakend-config/final-krakend.prod.json -ddd  -t --lint
	krakend check -tlc ./krakend-config/final-krakend.prod.json
	krakend check --config ./krakend-config/final-krakend.staging.json -ddd  -t --lint
	krakend check -tlc ./krakend-config/final-krakend.staging.json

copy-configs-to-gateway: ## Copy final configurations to API gateway
	cp ./krakend-config/final-krakend.prod.json ../services/backend-api-gateway/krakend.prod.json
	cp ./krakend-config/final-krakend.staging.json ../services/backend-api-gateway/krakend.staging.json

autogen: gen copy-swagger convert-swagger-to-openapiv3 update-typescript-client-sdk update-docs generate-krakend-config prettify-krakend merge-configs validate copy-configs-to-gateway ## Run all generation steps

generate: gen copy-swagger convert-swagger-to-openapiv3 update-typescript-client-sdk update-docs generate-krakend-config prettify-krakend merge-configs validate copy-configs-to-gateway ## Alias for autogen
