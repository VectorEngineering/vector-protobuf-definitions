#!/bin/bash

# Environment variables
SWAGGER_LEAD_SCRAPER_SERVICE_CODEGEN_PATH="./api-definitions/pkg/generated/lead_scraper_service/v1/apidocs.swagger.json"
SWAGGER_WORKSPACE_SERVICE_CODEGEN_PATH="./api-definitions/pkg/generated/workspace_service/v1/apidocs.swagger.json"

VERSION=3
FILE_PROD="./krakend-config/final-krakend.prod.json"
FILE_STAGING="./krakend-config/final-krakend.staging.json"
export JAVA_OPTS="-DmaxYamlCodePoints=99999999 -Dsnakeyaml.maxaliases=2147483647"

# ANSI color codes
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Read version from config file
read_versions() {
    TS_VERSION=$(grep 'typescript:' version.yaml | awk '{print $2}')
    PYTHON_VERSION=$(grep 'python:' version.yaml | awk '{print $2}')
    GO_VERSION=$(grep 'go:' version.yaml | awk '{print $2}')
    DOCS_VERSION=$(grep 'docs:' version.yaml | awk '{print $2}')
    ORG_NAME=$(grep 'org:' version.yaml | awk '{print $2}')
}

# Help function
show_help() {
    echo -e "${CYAN}Usage:${NC}"
    echo -e "  ./build.sh ${GREEN}<command>${NC}"
    echo ""
    echo -e "${CYAN}Available commands:${NC}"
    echo -e "  ${GREEN}test${NC}                        Run all SDK tests"
    echo -e "  ${GREEN}gen-linkerd${NC}                    Generate Linkerd service profiles"
    echo -e "  ${GREEN}gen${NC}                           Generate API definitions"
    echo -e "  ${GREEN}copy-swagger${NC}                  Copy Swagger files"
    echo -e "  ${GREEN}convert-swagger${NC}               Convert Swagger to OpenAPI v3"
    echo -e "  ${GREEN}update-typescript-client${NC}      Generate TypeScript client SDKs"
    echo -e "  ${GREEN}update-typescript-server${NC}      Generate server SDKs"
    echo -e "  ${GREEN}update-docs${NC}                   Generate documentation"
    echo -e "  ${GREEN}generate-krakend${NC}             Generate KrakenD config"
    echo -e "  ${GREEN}prettify-krakend${NC}             Format KrakenD config"
    echo -e "  ${GREEN}merge-configs${NC}                 Merge KrakenD configurations"
    echo -e "  ${GREEN}validate${NC}                      Validate KrakenD config"
    echo -e "  ${GREEN}copy-configs${NC}                  Copy configs to gateway"
    echo -e "  ${GREEN}gen-workspace-api${NC}            Generate Workspace Service API"
    echo -e "  ${GREEN}autogen${NC}                      Run all generation steps"
}

gen_linkerd() {
    echo "Generating Linkerd service profiles..."
    linkerd profile --namespace lead-scraper-service --proto ./api-definitions/api/lead-scraper-service/protobuf/lead_scraper_service/v1/account_service.proto backend-lead-scraper-service > linkerd-definitions/lead-scraper-service.yaml
    linkerd profile --namespace workspace-service --proto ./api-definitions/api/workspace-service/protobuf/workspace_service/v1/account_service.proto backend-workspace-service > linkerd-definitions/workspace-service.yaml
}

gen() {
    echo "Generating API definitions..."
    (cd api-definitions && make gen && cd ..)
}

copy_swagger() {
    echo "Copying Swagger files..."
    cp -rf "$SWAGGER_LEAD_SERVICE_CODEGEN_PATH" ./swagger/lead-scraper-service.json
    cp -rf "$SWAGGER_WORKSPACE_SERVICE_CODEGEN_PATH" ./swagger/workspace-service.json
}

convert_swagger() {
    echo "Converting Swagger to OpenAPI v3..."
    local services=("lead-scraper-service" "workspace-service")
    
    for service in "${services[@]}"; do
        swagger2openapi --yaml --outfile "./swagger/${service}-backend-api.yaml" "./swagger/${service}.json"
        swagger2openapi --outfile "./swagger/${service}-backend-api.json" "./swagger/${service}.json"
    done

    npx openapi-merge-cli --config ./swagger/merge-config.json
    swagger2openapi --outfile ./swagger/backend-api.json ./swagger/backend-api.yaml
}

update_typescript_client() {
    read_versions
    update_typescript_server
    echo "Generating TypeScript client SDKs..."
    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g typescript-fetch -o ./sdk/client/client-typescript-sdk/src/typescript-fetch \
        --additional-properties= \
            typescriptThreePlus=true, \
            useSingleRequestParameter=true, \
            withInterfaces=true, \
            useObjectParameters=true, \
            supportsES6=true, \
            modelPropertyNaming=original, \
            paramNaming=original, \
            withNodeImports=true, \
            runtimeAssertions=true, \
            disallowAdditionalPropertiesIfNotPresent=true, \
            precompileModel=true, \
            packageName=@${ORG_NAME}/client, \
            npmVersion=${TS_VERSION}, \
            npmAuthor="Playbook Media Team"

    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g typescript-node \
        --additional-properties= \
            legacyDiscriminatorBehavior=false, \
            nullSafeAdditionalProps=true, \
            disallowAdditionalPropertiesIfNotPresent=true, \
            modelPropertyNaming=original, \
            paramNaming=original, \
            prependFormOrBodyParameters=true, \
            withSeparateModelsAndApi=true, \
            apiPackage=api, \
            modelPackage=model, \
            npmName=@${ORG_NAME}/node-client, \
            npmVersion=${TS_VERSION}, \
            useSingleRequestParameter=true, \
            useObjectParameters=true, \
            serviceSuffix=Service, \
            serviceResponseSuffix=Response, \
            generateModelTests=true, \
            generateParameterTests=true, \
            enumUnknownDefaultCase=true, \
            hooks=onEventHook,preprocessOpenAPI \
        -o ./sdk/client/client-node-sdk/src/node-sdk

    echo "Generating Python client SDK..."
    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g python -o ./sdk/client/python-client-sdk \
        --additional-properties=projectName=playbookmedia-backend-client-sdk,packageName=playbookmedia_backend_client_sdk,packageVersion=${PYTHON_VERSION},packageUrl=${REPO}/sdk-python-client,licenseName=MIT,library=asyncio,generateSourceCodeOnly=true,hideGenerationTimestamp=true,strictSpecBehavior=true,validateSpec=true

    echo "Generating Go client SDK..."
    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g go -o ./sdk/client/go-client-sdk \
        --additional-properties= \
            packageName=client, \
            packageVersion=${GO_VERSION}, \
            structPrefix=Playbook, \
            withGoCodegenComment=true, \
            hideGenerationTimestamp=true, \
            isGoSubmodule=true, \
            enumClassPrefix=true, \
            generateInterfaces=true, \
            disallowAdditionalPropertiesIfNotPresent=true
}

update_typescript_server() {
    echo "Generating server SDKs..."
    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g graphql-nodejs-express-server -o ./sdk/server/node

    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g spring -o ./sdk/server/java

    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g rust-server -o ./sdk/server/rust

    echo "Generating Python server SDK..."
    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g python-flask -o ./sdk/server/python \
        --additional-properties=packageName=playbookmedia_backend_server_sdk,packageVersion=${PYTHON_VERSION},controllerPackage=controllers,defaultController=default_controller,supportPython2=false,hideGenerationTimestamp=true,validateSpec=true,strictSpecValidation=true,generateSourceCodeOnly=true,serverPort=8080

    echo "Generating Go server SDK..."
    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g go-server -o ./sdk/server/go \
        --additional-properties= \
            packageName=server, \
            packageVersion=${GO_VERSION}, \
            serverPort=8080, \
            withGoCodegenComment=true, \
            hideGenerationTimestamp=true, \
            apiPath=api, \
            modelPath=model, \
            strictValidate=true, \
            structPrefix=Playbook
}

update_docs() {
    read_versions
    echo "Generating documentation..."
    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g openapi-yaml -o ./documentation/autogen/openapi-yaml \
        --additional-properties= \
            outputName=playbookmedia-api, \
            useIntroduction=true, \
            useMethodTags=true, \
            sortParamsByRequiredFlag=true, \
            ensureUniqueParams=true, \
            enablePostProcessFile=true, \
            docExtension=md, \
            infoEmail=api@playbookmedia.com, \
            infoDescription="Playbook Media API Documentation", \
            infoLicenseName=MIT, \
            infoLicenseUrl=https://opensource.org/licenses/MIT, \
            tagsOrder=Authentication,Scraping,Account

    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g markdown -o ./documentation/autogen/markdown \
        --additional-properties= \
            generateAliasAsModel=true, \
            enablePostProcessFile=true, \
            generateExamples=true, \
            generateDescription=true, \
            useIntroduction=true, \
            useMethodTags=true, \
            codeFormatting=PRETTY, \
            docsStructure=GROUPS, \
            introductionContent=docs/intro.md, \
            separateModels=true, \
            separateApis=true

    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g postman-collection -o ./documentation/autogen/postman-collection \
        --additional-properties= \
            postmanName="Playbook Media API", \
            postmanSchema="https://schema.getpostman.com/json/collection/v2.1.0/collection.json", \
            postmanUid=${ORG_NAME}-api, \
            postmanVersion=1.0.0, \
            postmanVariables=HOST=http://localhost:8080, \
            securityDefinitionFormat=API_KEY, \
            generateAuthHelpers=true, \
            generateExampleRequests=true, \
            generateExampleResponses=true

    echo "Generating HTML documentation..."
    openapi-generator-cli generate \
        -i ./swagger/backend-api.yaml \
        -g html2 -o ./documentation/autogen/html \
        --additional-properties= \
            name=PlaybookMedia, \
            packageName=PlaybookMediaDocs, \
            hideGenerationTimestamp=true, \
            theme=light, \
            useNavigation=true, \
            includeApiList=true, \
            includeModelList=true, \
            logo=docs/assets/logo.png, \
            primaryColor="#2F3C4F", \
            secondaryColor="#00B4AB"
}

generate_krakend() {
    echo "Generating KrakenD configuration..."
    mkdir -p temp-swagger
    cp ./swagger/*-backend-api.json ./temp-swagger/
    cp ./swagger/lead-scraper-service-rest-api.json ./temp-swagger/

    go run ./openapi2krakend/pkg/main.go -directory ./temp-swagger \
        -output ./krakend-config/krakend.prod.json -environment production -webhook-config-path ./webhooks.yaml
    go run ./openapi2krakend/pkg/main.go -directory ./temp-swagger \
        -output ./krakend-config/krakend.staging.json -environment staging -webhook-config-path ./webhooks.yaml

    rm -rf temp-swagger
}

prettify_krakend() {
    echo "Formatting KrakenD configuration..."
    cat ./krakend-config/krakend.json | jq > ./krakend-config/krakend.compiled.json
    mv ./krakend-config/krakend.compiled.json ./krakend-config/krakend.json
}

merge_configs() {
    echo "Merging KrakenD configurations..."
    jq -s '.[0] * .[1]' ./krakend-config/gateway-configurations.json ./krakend-config/krakend.prod.json > "$FILE_PROD"
    sed -i '' "s/\"version\": \"3\"/\"version\": $VERSION/" "$FILE_PROD"
    
    jq -s '.[0] * .[1]' ./krakend-config/gateway-configurations.json ./krakend-config/krakend.staging.json > "$FILE_STAGING"
    sed -i '' "s/\"version\": \"3\"/\"version\": $VERSION/" "$FILE_STAGING"
}

validate() {
    echo "Validating KrakenD configuration..."
    krakend check --config "$FILE_PROD" -ddd -t --lint
    krakend check -tlc "$FILE_PROD"
    krakend check --config "$FILE_STAGING" -ddd -t --lint
    krakend check -tlc "$FILE_STAGING"
}

copy_configs() {
    echo "Copying configurations to API gateway..."
    cp "$FILE_PROD" ./services/backend-api-gateway/krakend.prod.json
    cp "$FILE_STAGING" ./services/backend-api-gateway/krakend.staging.json
}

autogen() {
    gen
    copy_swagger
    convert_swagger
    update_typescript_client
    update_docs
    generate_krakend
    prettify_krakend
    merge_configs
    validate
    copy_configs
    gen_linkerd
}

# Test all SDKs
run_sdk_tests() {
    echo -e "${CYAN}Running SDK tests...${NC}"
    local exit_code=0
    
    # TypeScript tests
    echo -e "\n${GREEN}Testing TypeScript SDKs${NC}"
    (cd sdk/client/client-typescript-sdk && npm install && npm run test) || exit_code=$?
    (cd sdk/client/client-node-sdk && npm install && npm run test) || exit_code=$?

    # Python tests
    echo -e "\n${GREEN}Testing Python SDKs${NC}"
    (cd sdk/client/python-client-sdk && python3 -m pytest tests/) || exit_code=$?
    (cd sdk/server/python && python3 -m pytest tests/) || exit_code=$?
}

# Main script execution
case "$1" in
    "gen-linkerd")
        gen_linkerd
        ;;
    "gen")
        gen
        ;;
    "copy-swagger")
        copy_swagger
        ;;
    "convert-swagger")
        convert_swagger
        ;;
    "update-typescript-client")
        update_typescript_client
        ;;
    "update-typescript-server")
        update_typescript_server
        ;;
    "update-docs")
        update_docs
        ;;
    "generate-krakend")
        generate_krakend
        ;;
    "prettify-krakend")
        prettify_krakend
        ;;
    "merge-configs")
        merge_configs
        ;;
    "validate")
        validate
        ;;
    "copy-configs")
        copy_configs
        ;;
    "gen-workspace-api")
        gen_workspace_api
        ;;
    "autogen")
        autogen
        ;;
    "test")
        run_sdk_tests
        ;;
    "help"|"-h"|"--help"|"")
        show_help
        ;;
    *)
        echo -e "${YELLOW}Unknown command: $1${NC}"
        echo "Run './build.sh help' for usage information"
        exit 1
        ;;
esac

exit 0