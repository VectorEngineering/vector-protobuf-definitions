package converter

import (
	"fmt"
	"io/fs"
	"io/ioutil"
	"strings"

	"github.com/okhuz/openapi2krakend/pkg/extensions"
	"github.com/okhuz/openapi2krakend/pkg/models"
	"github.com/okhuz/openapi2krakend/pkg/utility"
)

func Convert(swaggerDirectory string, encoding string, globalTimeout, environment string, webhookCfg *utility.WebhookConfig) models.Configuration {
	var swaggerFiles []fs.FileInfo
	if files, err := ioutil.ReadDir(swaggerDirectory); err == nil {
		swaggerFiles = filterFiles(files)
	}

	// numberOfFiles := len(swaggerFiles)
	configuration := models.NewConfiguration(encoding, globalTimeout)

	for _, file := range swaggerFiles {

		openApiDefinition := loadFromFile(fmt.Sprintf("%s/%s", swaggerDirectory, file.Name()))

		host := openApiDefinition.Servers[0].URL
		path := sanitizeTitle(openApiDefinition.Info.Title)
		pathPrefix := utility.GetEnv("PATH_PREFIX", "")
		apiTimeout := globalTimeout

		if extensionValue := getExtension(openApiDefinition.Extensions, extensions.TimeOut); extensionValue != "" {
			apiTimeout = extensionValue
		}

		for pathUrl, pathItem := range openApiDefinition.Paths {
			for methodName, methodObject := range pathItem.Operations() {

				krakendEndpointUrl := fmt.Sprintf("%s", pathUrl)
				// if numberOfFiles > 1 {
				// 	krakendEndpointUrl = fmt.Sprintf("/%s%s", path, pathUrl)
				// }
				if pathPrefix != "" {
					krakendEndpointUrl = fmt.Sprintf("/%s%s", pathPrefix, pathUrl)
				}

				methodTimeout := apiTimeout
				if extensionValue := getExtension(methodObject.Extensions, extensions.TimeOut); extensionValue != "" {
					methodTimeout = extensionValue
				}

				krakendEndpoint := models.NewEndpointWithDefaults(host, krakendEndpointUrl, pathUrl, methodName, encoding, methodTimeout, environment)
				if methodObject.Security != nil {
					lengthOfSecurity := len(*methodObject.Security)
					if lengthOfSecurity > 0 {
						krakendEndpoint.InsertHeadersToPass("Authorization")
					}
				}
				for _, parameterObject := range methodObject.Parameters {
					parameter := parameterObject.Value
					if parameter.In == "query" {
						if parameter.Explode != nil && *parameter.Explode == true && parameter.Schema.Ref != "" {
							explodedParams := getComponentFromReferenceAddress(*openApiDefinition, parameter.Schema.Ref)
							if explodedParams.Type == "object" {
								insertQueryParams(&explodedParams, openApiDefinition, &krakendEndpoint)
							} else if explodedParams.Type == "Array" {
								krakendEndpoint.InsertQuerystringParams(parameter.Name)
							} else if explodedParams.Type == "string" && explodedParams.Enum != nil {
								krakendEndpoint.InsertQuerystringParams(parameter.Name)
							}
						} else {
							krakendEndpoint.InsertQuerystringParams(parameter.Name)
						}
					} else if parameter.In == "header" {
						krakendEndpoint.InsertHeadersToPass(parameter.Name)
					}
				}

				configuration.InsertEndpoint(krakendEndpoint)
			}
		}

		additionalPaths := utility.GetEnv("ADDITIONAL_PATHS", "")
		if additionalPaths != "" {
			additionalPathArray := strings.Split(additionalPaths, ",")
			for _, v := range additionalPathArray {
				additionalEndpoint := models.NewEndpointWithDefaults(host, fmt.Sprintf("/%s%s", path, v), v, "get", encoding, apiTimeout, environment)
				additionalEndpoint.InsertHeadersToPass("Authorization")
				configuration.InsertEndpoint(additionalEndpoint)
			}
		}
	}

	// Add webhook endpoints
	if webhookCfg != nil {
		for _, endpoint := range webhookCfg.Endpoints {
			krakendEndpoint := models.NewProxyEndpoint(endpoint.Host, endpoint.KrakendApiPath, endpoint.ServiceApiPath, globalTimeout)
			fmt.Println("Adding webhook endpoint", krakendEndpoint)
			// make sure the content type is empty
			krakendEndpoint.InputHeaders = []string{"*"}
			configuration.InsertEndpoint(krakendEndpoint)
		}
	}

	return configuration
}
