package utility

import (
	"path/filepath"
	"testing"
)

func TestReadAndUnmarshalYamlFile_Success(t *testing.T) {
	// Assuming your Go test runs at the package root, and test_data is a folder in your package
	filePath := filepath.Join("../../test_data", "test_config.yaml")

	config, err := ReadAndUnmarshalYamlFile(filePath)
	if err != nil {
		t.Fatalf("Failed to read or unmarshal YAML: %s", err)
	}

	if len(config.Endpoints) != 2 {
		t.Errorf("Expected 2 endpoints, got %d", len(config.Endpoints))
	}
}

func TestReadAndUnmarshalYamlFile_EmptyPath(t *testing.T) {
	config, err := ReadAndUnmarshalYamlFile("")
	if err == nil {
		t.Fatal("Expected an error for empty file path, got none")
	}

	if config != nil {
		t.Fatal("Expected config to be nil on error, it was not")
	}
}

func TestReadAndUnmarshalYamlFile_NonExistentPath(t *testing.T) {
	filePath := "nonexistent.yaml"

	_, err := ReadAndUnmarshalYamlFile(filePath)
	if err == nil {
		t.Fatal("Expected an error for a non-existent file path, got none")
	}
}

func TestReadAndUnmarshalYamlFile_InvalidYAML(t *testing.T) {
	filePath := filepath.Join("../../test_data", "invalid.yaml")

	_, err := ReadAndUnmarshalYamlFile(filePath)
	if err == nil {
		t.Fatal("Expected an error for invalid YAML, got none")
	}
}

func TestReadAndUnmarshalYamlFile_UnexpectedStructure(t *testing.T) {
	filePath := filepath.Join("../../test_data", "unexpected_structure.yaml")

	cfg, err := ReadAndUnmarshalYamlFile(filePath)
	if err != nil {
		t.Fatalf("Unexpected error: %s", err)
	}

	// Assuming you know the structure and which fields are mandatory or expected to have specific values
	// For example, check if the first endpoint's Name field is empty, indicating a structure mismatch
	if len(cfg.Endpoints) != 0 {
		t.Fatal("Expected specific structure with non-empty 'Name' field, got unexpected structure or empty field")
	}
}
