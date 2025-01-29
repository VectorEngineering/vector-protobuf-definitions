package main

import (
	"encoding/json"
	"flag"
	"os"
	"path/filepath"

	"github.com/okhuz/openapi2krakend/pkg/converter"
	"github.com/okhuz/openapi2krakend/pkg/utility"
	"go.uber.org/zap"
)

func main() {
	logger := zap.L()
	swaggerDirectory := flag.String("directory", "./swagger", "Directory of the swagger files")
	outputFilePath := flag.String("output", "output/krakend.json", "Output file path")
	environment := flag.String("environment", "development", "Environment")
	webhookConfigFilePath := flag.String("webhook-config-path", "", "Webhook configuration file path")

	flag.Parse()

	encoding := utility.GetEnv("ENCODING", "json")
	globalTimeout := utility.GetEnv("GLOBAL_TIMEOUT", "15s")

	webhookConfg, err := utility.ReadAndUnmarshalYamlFile(*webhookConfigFilePath)
	if err != nil {
		logger.Error("Error reading webhook configuration file", zap.Error(err))
		panic(err)
	}

	logger.Info("Converting swagger files to krakend configuration", zap.Any("cfg", webhookConfg))

	configuration := converter.Convert(*swaggerDirectory, encoding, globalTimeout, *environment, webhookConfg)

	file, _ := json.MarshalIndent(configuration, "", " ")
	outputDir := filepath.Dir(*outputFilePath)
	_ = os.MkdirAll(outputDir, 0777)
	_ = os.WriteFile(*outputFilePath, file, 0644)
}
