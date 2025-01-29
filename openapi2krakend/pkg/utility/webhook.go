package utility

import (
	"fmt"
	"io/ioutil"

	"gopkg.in/yaml.v2"
)

type WebhookConfig struct {
	Endpoints []Endpoint `yaml:"endpoints"`
}

type Endpoint struct {
	Name           string `yaml:"name"`
	KrakendApiPath string `yaml:"krakendApiPath"`
	ServiceApiPath string `yaml:"serviceApiPath"`
	Method         string `yaml:"method"`
	Host           string `yaml:"host"`
	Encoding       string `yaml:"encoding"`
}

func ReadAndUnmarshalYamlFile(filePath string) (*WebhookConfig, error) {
	if filePath == "" {
		return nil, fmt.Errorf("file path is empty")
	}

	// Read the file
	yamlFile, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Printf("Error reading YAML file: %s\n", err)
		return nil, err
	}

	// Unmarshal the YAML file into a WebhookConfigs struct
	var config WebhookConfig
	err = yaml.Unmarshal(yamlFile, &config)
	if err != nil {
		fmt.Printf("Error parsing YAML file: %s\n", err)
		return nil, err
	}

	return &config, nil
}
