#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color
BLUE='\033[0;34m'

# Store the chart path
CHART_PATH=${1:-"./charts/api-gateway"}
CHART_NAME=$(basename $(pwd))
TEMP_VALUES="/tmp/test-values.yaml"
TEST_NAMESPACE="test-${CHART_NAME}"

# Error counter
ERRORS=0
WARNINGS=0

echo -e "${BLUE}Starting Helm Chart Validation for: ${CHART_NAME}${NC}\n"

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to print section headers
print_section() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

# Function to check for required tools
check_prerequisites() {
    print_section "Checking Prerequisites"
    local missing_tools=0

    local required_tools=("helm" "kubectl" "yq" "yamllint")

    for tool in "${required_tools[@]}"; do
        if ! command_exists "$tool"; then
            echo -e "${RED}✗ $tool is not installed${NC}"
            missing_tools=$((missing_tools + 1))
        else
            echo -e "${GREEN}✓ $tool is installed${NC}"
        fi
    done

    if [ $missing_tools -gt 0 ]; then
        echo -e "${RED}Please install missing tools before proceeding${NC}"
        exit 1
    fi
}

# Function to validate chart syntax
validate_chart_syntax() {
    print_section "Validating Chart Syntax"

    if helm lint "$CHART_PATH" --strict; then
        echo -e "${GREEN}✓ Helm lint passed${NC}"
    else
        echo -e "${RED}✗ Helm lint failed${NC}"
        ERRORS=$((ERRORS + 1))
    fi
}

# Function to validate yaml syntax
validate_yaml_syntax() {
    print_section "Validating YAML Syntax"

    local yaml_files=$(find "$CHART_PATH" -name "*.yaml" -o -name "*.yml")
    local yaml_errors=0

    for file in $yaml_files; do
        if yamllint -d relaxed "$file" > /dev/null 2>&1; then
            echo -e "${GREEN}✓ $file - YAML syntax valid${NC}"
        else
            echo -e "${RED}✗ $file - YAML syntax errors found${NC}"
            yaml_errors=$((yaml_errors + 1))
        fi
    done

    ERRORS=$((ERRORS + yaml_errors))
}

# Function to validate templates
validate_templates() {
    print_section "Validating Template Rendering"
    local template_errors=0
    local template_warnings=0
    local temp_output="/tmp/template-output.yaml"

    echo -e "${BLUE}Performing comprehensive template validation...${NC}"

    # Step 1: Basic template rendering
    if helm template "$CHART_PATH" > "$temp_output" 2>/dev/null; then
        echo -e "${GREEN}✓ Templates render successfully${NC}"
    else
        echo -e "${RED}✗ Template rendering failed${NC}"
        template_errors=$((template_errors + 1))
    fi

    # Step 2: Find all template files
    local templates=$(find "$CHART_PATH/templates" -type f -name "*.yaml" -o -name "*.tpl")

    # Step 3: Detailed template analysis
    for template in $templates; do
        local template_name=$(basename "$template")
        echo -e "\n${BLUE}Analyzing template: $template_name${NC}"

        # Check for required labels
        if ! grep -l "app.kubernetes.io/name" "$template" > /dev/null; then
            echo -e "${YELLOW}⚠ Missing recommended label: app.kubernetes.io/name${NC}"
            template_warnings=$((template_warnings + 1))
        fi

        # Check for potentially dangerous functions
        if grep -l "tpl " "$template" > /dev/null; then
            echo -e "${YELLOW}⚠ Uses tpl function - verify implementation${NC}"
            template_warnings=$((template_warnings + 1))
        fi

        # Check for hardcoded values
        if grep -E "namespace: [\"']?[a-zA-Z0-9-]+[\"']?" "$template" > /dev/null; then
            echo -e "${YELLOW}⚠ Contains hardcoded namespace${NC}"
            template_warnings=$((template_warnings + 1))
        fi

        # Check for proper indentation
        if grep -E "^[[:space:]]{1,3}[a-zA-Z]" "$template" > /dev/null; then
            echo -e "${YELLOW}⚠ Possible indentation issues detected${NC}"
            template_warnings=$((template_warnings + 1))
        fi

        # Check for commented-out code
        if grep -E "^[[:space:]]*#[[:space:]]*[a-zA-Z]" "$template" > /dev/null; then
            echo -e "${YELLOW}⚠ Contains commented code - review if necessary${NC}"
            template_warnings=$((template_warnings + 1))
        fi

        # Check for template best practices
        local template_content=$(<"$template")

        # Check for proper quote usage
        if echo "$template_content" | grep -l 'value: {{ ' > /dev/null; then
            echo -e "${YELLOW}⚠ Values should be quoted: value: \"{{ ... }}\"${NC}"
            template_warnings=$((template_warnings + 1))
        fi

        # Check for proper helper usage
        if grep -l "define \".*\"" "$template" > /dev/null && [[ "$template" != *"_helpers.tpl" ]]; then
            echo -e "${YELLOW}⚠ Template defines should be in _helpers.tpl${NC}"
            template_warnings=$((template_warnings + 1))
        fi

        # Check for resource naming conventions
        if grep -E 'name: "[^{]' "$template" > /dev/null; then
            echo -e "${YELLOW}⚠ Resource names should use template functions${NC}"
            template_warnings=$((template_warnings + 1))
        fi

        # Validate conditional statements
        if grep -l "if and" "$template" > /dev/null; then
            echo -e "${BLUE}ℹ Contains conditional logic - verifying syntax${NC}"
            if ! grep -l "end" "$template" > /dev/null; then
                echo -e "${RED}✗ Missing 'end' statement for conditional${NC}"
                template_errors=$((template_errors + 1))
            fi
        fi

        # Check for required annotations
        local required_annotations=(
            "app.kubernetes.io/managed-by"
            "helm.sh/chart"
            "app.kubernetes.io/version"
        )

        for annotation in "${required_annotations[@]}"; do
            if ! grep -l "$annotation" "$template" > /dev/null; then
                echo -e "${YELLOW}⚠ Missing recommended annotation: $annotation${NC}"
                template_warnings=$((template_warnings + 1))
            fi
        done

        # Validate value references
        if grep -E "{{ \.Values\.[a-zA-Z0-9._-]+ }}" "$template" > /dev/null; then
            # Check if the referenced values exist in values.yaml
            local value_refs=$(grep -oE "\.Values\.[a-zA-Z0-9._-]+" "$template")
            for ref in $value_refs; do
                if ! yq e "$ref" "$CHART_PATH/values.yaml" > /dev/null 2>&1; then
                    echo -e "${YELLOW}⚠ Referenced value not found in values.yaml: $ref${NC}"
                    template_warnings=$((template_warnings + 1))
                fi
            done
        fi
    done

    # Step 4: Check for template naming conventions
    for template in $templates; do
        local template_name=$(basename "$template")
        case $template_name in
            _*)
                # Helper templates are fine
                ;;
            *.yaml)
                # Check if filename matches resource type
                local resource_type=$(grep -l "kind:" "$template" | head -n 1)
                if [ ! -z "$resource_type" ]; then
                    if ! echo "$template_name" | grep -iE "^(deployment|service|ingress|configmap|secret|hpa|pdb|serviceaccount)" > /dev/null; then
                        echo -e "${YELLOW}⚠ Template filename might not match resource type: $template_name${NC}"
                        template_warnings=$((template_warnings + 1))
                    fi
                fi
                ;;
        esac
    done

    # Step 5: Cleanup and summary
    rm -f "$temp_output"

    # Update global counters
    ERRORS=$((ERRORS + template_errors))
    WARNINGS=$((WARNINGS + template_warnings))

    # Final status report
    if [ $template_errors -gt 0 ]; then
        echo -e "\n${RED}✗ Template validation failed with $template_errors errors and $template_warnings warnings${NC}"
    else
        echo -e "\n${GREEN}✓ Template validation completed with $template_warnings warnings${NC}"
    fi
}

# Function to validate values schema
validate_values_schema() {
    print_section "Validating Values Schema"

    if [ -f "$CHART_PATH/values.schema.json" ]; then
        if helm lint "$CHART_PATH" --strict --values "$CHART_PATH/values.yaml" --values "$CHART_PATH/values.schema.json"; then
            echo -e "${GREEN}✓ Values schema validation passed${NC}"
        else
            echo -e "${RED}✗ Values schema validation failed${NC}"
            ERRORS=$((ERRORS + 1))
        fi
    else
        echo -e "${YELLOW}⚠ No values.schema.json found - consider adding schema validation${NC}"
        WARNINGS=$((WARNINGS + 1))
    fi
}

# Function to validate Kubernetes resources
validate_k8s_resources() {
    print_section "Validating Kubernetes Resources"
    local validation_errors=0
    local temp_rendered="/tmp/rendered-templates.yaml"

    echo -e "${BLUE}Performing comprehensive Kubernetes resource validation...${NC}"

    # Create a temporary values file for testing
    cp "$CHART_PATH/values.yaml" "$TEMP_VALUES"

    # Step 1: Render templates to a file for inspection
    if ! helm template "$CHART_PATH" --values "$TEMP_VALUES" > "$temp_rendered" 2>/dev/null; then
        echo -e "${RED}✗ Failed to render Helm templates${NC}"
        validation_errors=$((validation_errors + 1))
        rm -f "$TEMP_VALUES" "$temp_rendered"
        return 1
    fi

    # Step 2: Split and validate individual resources
    echo "---" >> "$temp_rendered"  # Ensure last document has separator
    csplit --quiet --prefix="/tmp/resource-" --suffix-format="%03d.yaml" "$temp_rendered" "/^---$/" "{*}" 2>/dev/null

    # Step 3: Validate each resource
    for resource in /tmp/resource-*.yaml; do
        if [ -s "$resource" ]; then  # Check if file is non-empty
            # Get resource type and name
            local kind=$(yq e '.kind' "$resource" 2>/dev/null)
            local name=$(yq e '.metadata.name' "$resource" 2>/dev/null)

            if [ ! -z "$kind" ] && [ ! -z "$name" ]; then
                echo -e "${BLUE}Validating $kind: $name${NC}"

                # Perform kubectl validation
                if ! kubectl apply --dry-run=client -f "$resource" > /dev/null 2>&1; then
                    echo -e "${RED}✗ Invalid $kind manifest: $name${NC}"
                    validation_errors=$((validation_errors + 1))
                    continue
                fi

                # Resource-specific validations
                case $kind in
                    "Deployment"|"StatefulSet")
                        # Check for resource limits
                        if ! yq e '.spec.template.spec.containers[].resources' "$resource" | grep -q "limits"; then
                            echo -e "${YELLOW}⚠ No resource limits defined for $kind: $name${NC}"
                            WARNINGS=$((WARNINGS + 1))
                        fi
                        # Check for security context
                        if ! yq e '.spec.template.spec.securityContext' "$resource" | grep -q ".*"; then
                            echo -e "${YELLOW}⚠ No security context defined for $kind: $name${NC}"
                            WARNINGS=$((WARNINGS + 1))
                        fi
                        # Check for liveness/readiness probes
                        if ! yq e '.spec.template.spec.containers[].livenessProbe' "$resource" | grep -q ".*"; then
                            echo -e "${YELLOW}⚠ No liveness probe defined for $kind: $name${NC}"
                            WARNINGS=$((WARNINGS + 1))
                        fi
                        ;;
                    "Service")
                        # Validate service type
                        local svc_type=$(yq e '.spec.type' "$resource")
                        if [ "$svc_type" = "LoadBalancer" ]; then
                            echo -e "${YELLOW}⚠ Service $name uses LoadBalancer type - ensure this is intended${NC}"
                            WARNINGS=$((WARNINGS + 1))
                        fi
                        ;;
                    "Ingress")
                        # Check for TLS configuration
                        if ! yq e '.spec.tls' "$resource" | grep -q ".*"; then
                            echo -e "${YELLOW}⚠ No TLS configuration found for Ingress: $name${NC}"
                            WARNINGS=$((WARNINGS + 1))
                        fi
                        ;;
                    "Secret")
                        # Check if secrets are base64 encoded
                        if ! yq e '.data' "$resource" | grep -q ".*"; then
                            echo -e "${YELLOW}⚠ Secret $name may not be properly encoded${NC}"
                            WARNINGS=$((WARNINGS + 1))
                        fi
                        ;;
                    "ConfigMap")
                        # Validate ConfigMap data presence
                        if ! yq e '.data' "$resource" | grep -q ".*"; then
                            echo -e "${YELLOW}⚠ Empty ConfigMap found: $name${NC}"
                            WARNINGS=$((WARNINGS + 1))
                        fi
                        ;;
                esac

                echo -e "${GREEN}✓ $kind $name validation passed${NC}"
            fi
        fi
    done

    # Step 4: Check for common deployment patterns
    if ! grep -q "strategy:" "$temp_rendered"; then
        echo -e "${YELLOW}⚠ No deployment strategy found in templates${NC}"
        WARNINGS=$((WARNINGS + 1))
    fi

    # Step 5: Cleanup
    rm -f "$TEMP_VALUES" "$temp_rendered" /tmp/resource-*.yaml

    # Final validation status
    if [ $validation_errors -gt 0 ]; then
        echo -e "${RED}✗ Kubernetes resource validation failed with $validation_errors errors${NC}"
        ERRORS=$((ERRORS + validation_errors))
        return 1
    else
        echo -e "${GREEN}✓ Kubernetes resource validation passed${NC}"
        return 0
    fi
}

# Function to check for security best practices
check_security_practices() {
    print_section "Checking Security Best Practices"

    local templates=$(find "$CHART_PATH/templates" -type f -name "*.yaml")

    # Check for security contexts
    for template in $templates; do
        if grep -l "kind: Deployment\|kind: StatefulSet" "$template" > /dev/null; then
            if ! grep -l "securityContext:" "$template" > /dev/null; then
                echo -e "${YELLOW}⚠ $template - No securityContext defined${NC}"
                WARNINGS=$((WARNINGS + 1))
            fi
        fi
    done

    # Check for resource limits
    if ! grep -r "resources:" "$CHART_PATH/templates" > /dev/null; then
        echo -e "${YELLOW}⚠ No resource limits found in templates${NC}"
        WARNINGS=$((WARNINGS + 1))
    fi
}

# Function to validate HPA configuration
validate_hpa_config() {
    print_section "Validating HPA Configuration"

    if [ -f "$CHART_PATH/templates/hpa.yaml" ]; then
        # Check for required HPA fields
        local hpa_template="$CHART_PATH/templates/hpa.yaml"

        if ! grep -l "minReplicas:" "$hpa_template" > /dev/null; then
            echo -e "${YELLOW}⚠ HPA missing minReplicas${NC}"
            WARNINGS=$((WARNINGS + 1))
        fi

        if ! grep -l "metrics:" "$hpa_template" > /dev/null; then
            echo -e "${YELLOW}⚠ HPA missing metrics configuration${NC}"
            WARNINGS=$((WARNINGS + 1))
        fi

        # Validate HPA template rendering
        if helm template "$CHART_PATH" -s templates/hpa.yaml > /dev/null; then
            echo -e "${GREEN}✓ HPA template renders successfully${NC}"
        else
            echo -e "${RED}✗ HPA template rendering failed${NC}"
            ERRORS=$((ERRORS + 1))
        fi
    else
        echo -e "${YELLOW}⚠ No HPA template found${NC}"
        WARNINGS=$((WARNINGS + 1))
    fi
}

# Main execution
main() {
    check_prerequisites
    validate_chart_syntax
    validate_yaml_syntax
    validate_templates
    validate_values_schema
    validate_k8s_resources
    check_security_practices
    validate_hpa_config

    # Print summary
    print_section "Validation Summary"
    echo -e "Total Errors: ${RED}${ERRORS}${NC}"
    echo -e "Total Warnings: ${YELLOW}${WARNINGS}${NC}"

    # Exit with error if there are errors
    if [ $ERRORS -gt 0 ]; then
        echo -e "\n${RED}Validation failed with $ERRORS errors${NC}"
        exit 1
    else
        echo -e "\n${GREEN}Validation completed successfully${NC}"
        if [ $WARNINGS -gt 0 ]; then
            echo -e "${YELLOW}Consider addressing $WARNINGS warnings${NC}"
        fi
        exit 0
    fi
}

# Run main function
main
