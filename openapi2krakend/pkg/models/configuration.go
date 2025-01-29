package models

import (
	"strings"

	"github.com/okhuz/openapi2krakend/pkg/utility"
)

type Backend struct {
	UrlPattern          string                 `json:"url_pattern"`
	Encoding            string                 `json:"encoding"`
	Method              string                 `json:"method,omitempty"`
	Host                []string               `json:"host"`
	DisableHostSanitize bool                   `json:"disable_host_sanitize"`
	ExtraConfig         map[string]interface{} `json:"extra_config,omitempty"` // Added line

}

func NewBackend(host string, endpoint string, method string, outputEncoding string) Backend {
	return Backend{
		UrlPattern:          endpoint,
		Encoding:            outputEncoding,
		Method:              strings.ToUpper(method),
		Host:                []string{host},
		DisableHostSanitize: false,
	}
}

func NewProxyBackend(host string, endpoint string) Backend {
	return Backend{
		UrlPattern:          endpoint,
		Encoding:            "no-op",
		Host:                []string{host},
		DisableHostSanitize: false,
	}
}

func NewBackendWithDefaults(host string, endpoint string, method string, outputEncoding string) Backend {
	return Backend{
		UrlPattern:          endpoint,
		Encoding:            outputEncoding,
		Method:              strings.ToUpper(method),
		Host:                []string{host},
		DisableHostSanitize: false,
		ExtraConfig:         DefaultBackendExtraConfig(),
	}
}

func (b *Backend) AddExtraConfig(key string, value interface{}) {
	if b.ExtraConfig == nil {
		b.ExtraConfig = make(map[string]interface{})
	}
	b.ExtraConfig[key] = value
}

type Endpoint struct {
	Endpoint          string                 `json:"endpoint"`
	Method            string                 `json:"method,omitempty"`
	OutputEncoding    string                 `json:"output_encoding"`
	Timeout           string                 `json:"timeout"`
	InputQueryStrings []string               `json:"input_query_strings"`
	Backend           []Backend              `json:"backend"`
	InputHeaders      []string               `json:"input_headers"`
	ExtraConfig       map[string]interface{} `json:"extra_config,omitempty"`
}

func NewEndpoint(host string, endpoint string, backendEndpoint string, method string, outputEncoding string, timeout string) Endpoint {
	backend := NewBackend(host, backendEndpoint, method, outputEncoding)
	return Endpoint{
		Endpoint:          endpoint,
		Method:            strings.ToUpper(method),
		OutputEncoding:    outputEncoding,
		Timeout:           timeout,
		InputQueryStrings: []string{},
		Backend:           []Backend{backend},
		InputHeaders:      []string{"Content-Type"},
	}
}

func NewProxyEndpoint(host string, endpoint string, backendEndpoint string, timeout string) Endpoint {
	backend := NewProxyBackend(host, backendEndpoint)
	return Endpoint{
		Endpoint:          endpoint,
		OutputEncoding:    "no-op",
		Timeout:           timeout,
		InputQueryStrings: []string{},
		Backend:           []Backend{backend},
		InputHeaders:      []string{"Content-Type"},
	}
}

func NewEndpointWithDefaults(host string, endpoint string, backendEndpoint string, method string, outputEncoding string, timeout, environment string) Endpoint {
	backend := NewBackendWithDefaults(host, backendEndpoint, method, outputEncoding)
	return Endpoint{
		Endpoint:          endpoint,
		Method:            strings.ToUpper(method),
		OutputEncoding:    outputEncoding,
		Timeout:           timeout,
		InputQueryStrings: []string{},
		Backend:           []Backend{backend},
		InputHeaders:      []string{"Content-Type"},
		ExtraConfig:       DefaultEndpointExtraConfig(environment),
	}
}

func (e *Endpoint) AddExtraConfig(key string, value interface{}) {
	if e.ExtraConfig == nil {
		e.ExtraConfig = make(map[string]interface{})
	}
	e.ExtraConfig[key] = value
}

func (e *Endpoint) InsertQuerystringParams(param string) {
	e.InputQueryStrings = append(e.InputQueryStrings, param)
}

func (e *Endpoint) InsertHeadersToPass(header string) {
	e.InputHeaders = append(e.InputHeaders, header)
}

type Configuration struct {
	Schema         string                 `json:"$schema"`
	Version        string                 `json:"version"`
	Timeout        string                 `json:"timeout"`
	CacheTtl       string                 `json:"cache_ttl"`
	OutputEncoding string                 `json:"output_encoding"`
	Name           string                 `json:"name"`
	Endpoints      []Endpoint             `json:"endpoints"`
	ExtraConfig    map[string]interface{} `json:"extra_config,omitempty"`
}

func NewConfiguration(outputEncoding string, timeout string) Configuration {
	var extraConfig = make(map[string]interface{}, 15)

	if utility.GetEnv("ENABLE_CORS", "false") == "true" {
		extraConfig["security/cors"] = NewCors()
	}
	if utility.GetEnv("ENABLE_LOGGING", "false") == "true" {
		extraConfig["telemetry/logging"] = NewLogging()
	}

	var loggerSkipPaths = utility.GetEnv("LOGGER_SKIP_PATHS", "")

	if loggerSkipPaths != "" {
		extraConfig["router"] = Router{
			LoggerSkipPaths: strings.Split(loggerSkipPaths, ","),
		}
	}

	return Configuration{
		Schema:         "https://www.krakend.io/schema/v3.json",
		Version:        "3",
		Timeout:        timeout,
		CacheTtl:       "300s",
		OutputEncoding: outputEncoding,
		Name:           "Vector Backend API",
		Endpoints:      []Endpoint{},
		ExtraConfig:    extraConfig,
	}
}

func (c *Configuration) InsertEndpoint(endpoint Endpoint) {
	c.Endpoints = append(c.Endpoints, endpoint)
}
