package postman

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/go-contrib/uuid"
	"github.com/luraproject/lura/v2/config"
)

// HandleCollection returns a simple http.HandleFunc exposing the POSTMAN collection description
func HandleCollection(c Collection) func(http.ResponseWriter, *http.Request) {
	b, err := json.Marshal(c)
	if err != nil {
		return func(rw http.ResponseWriter, r *http.Request) {
			http.Error(rw, err.Error(), http.StatusInternalServerError)
		}
	}
	return func(rw http.ResponseWriter, r *http.Request) {
		rw.Header().Add("Content-Type", "application/json")
		rw.Write(b)
	}
}

// Parse converts the received service config into a simple POSTMAN collection description
func Parse(cfg config.ServiceConfig) Collection {
	schema := "http"
	if cfg.TLS != nil {
		schema = "https"
	}
	c := Collection{
		Info: Info{
			Name:        cfg.Name,
			PostmanID:   uuid.NewV4().String(),
			Description: fmt.Sprintf("Collection for %s API, generated at %s", cfg.Name, time.Now().Format(time.RFC3339)),
			Schema:      "https://schema.getpostman.com/json/collection/v2.1.0/collection.json",
			Version:     "1.0.0",
		},
		Item: []Item{},
		Variables: []Variable{
			{
				ID:          uuid.NewV4().String(),
				Key:         "HOST",
				Type:        "string",
				Value:       fmt.Sprintf("localhost:%d", cfg.Port),
				Description: "API host",
			},
			{
				ID:          uuid.NewV4().String(),
				Key:         "SCHEMA",
				Type:        "string",
				Value:       schema,
				Description: "HTTP/HTTPS schema",
			},
		},
		Auth: &Auth{
			Type: "bearer",
			Bearer: []AuthAttribute{
				{
					Key:   "token",
					Value: "{{bearerToken}}",
					Type:  "string",
				},
			},
		},
	}

	for _, e := range cfg.Endpoints {
		item := Item{
			Name: e.Endpoint,
			Request: Request{
				URL: URL{
					Raw:      "{{SCHEMA}}://{{HOST}}" + e.Endpoint,
					Protocol: "{{SCHEMA}}",
					Host:     []string{"{{HOST}}"},
					Path:     splitPath(e.Endpoint),
				},
				Method:      e.Method,
				Description: fmt.Sprintf("%s endpoint", e.Method),
			},
		}

		if isMethodWithBody(e.Method) {
			item.Request.Body = &Body{
				Mode: "raw",
				Raw:  "{\n    // Add request body here\n}",
				Options: &BodyOptions{
					Raw: &RawOptions{
						Language: "json",
					},
				},
			}
		}

		c.Item = append(c.Item, item)
	}
	return c
}

func isMethodWithBody(method string) bool {
	return method == "POST" || method == "PUT" || method == "PATCH"
}

func splitPath(path string) []string {
	result := []string{}
	
	if path == "" || path == "/" {
		return result
	}

	parts := strings.Split(strings.TrimPrefix(path, "/"), "/")
	for _, p := range parts {
		if p == "" {
			continue
		}

		// Handle complete path parameter segment like {id}
		if strings.HasPrefix(p, "{") && strings.HasSuffix(p, "}") {
			result = append(result, p)
			continue
		}

		// Handle embedded path parameter like users{id} or prefix{param}suffix
		start := strings.Index(p, "{")
		end := strings.Index(p, "}")
		
		if start != -1 && end != -1 && start < end {
			// Add prefix if exists
			if start > 0 {
				result = append(result, p[:start])
			}
			// Add parameter
			result = append(result, p[start:end+1])
			// Add suffix if exists
			if end+1 < len(p) {
				result = append(result, p[end+1:])
			}
		} else {
			result = append(result, p)
		}
	}
	return result
}

type Collection struct {
	Variables []Variable `json:"variables"`
	Info      Info      `json:"info"`
	Item      []Item    `json:"item"`
	Auth      *Auth     `json:"auth,omitempty"`
}

type Variable struct {
	ID          string `json:"id"`
	Key         string `json:"key"`
	Value       string `json:"value"`
	Type        string `json:"type"`
	Description string `json:"description,omitempty"`
}

type Info struct {
	Name        string `json:"name"`
	PostmanID   string `json:"_postman_id"`
	Description string `json:"description,omitempty"`
	Schema      string `json:"schema"`
	Version     string `json:"version,omitempty"`
}

type Item struct {
	Name    string  `json:"name"`
	Request Request `json:"request"`
}

type Request struct {
	URL         URL      `json:"url"`
	Method      string   `json:"method"`
	Header      []Header `json:"header,omitempty"`
	Body        *Body    `json:"body,omitempty"`
	Description string   `json:"description,omitempty"`
}

type URL struct {
	Raw      string    `json:"raw"`
	Protocol string    `json:"protocol"`
	Host     []string  `json:"host"`
	Path     []string  `json:"path"`
}

type Body struct {
	Mode    string       `json:"mode,omitempty"`
	Raw     string       `json:"raw,omitempty"`
	Options *BodyOptions `json:"options,omitempty"`
}

type BodyOptions struct {
	Raw *RawOptions `json:"raw,omitempty"`
}

type RawOptions struct {
	Language string `json:"language"`
}

type Header struct {
	Key         string `json:"key"`
	Value       string `json:"value"`
	Description string `json:"description,omitempty"`
}

type Auth struct {
	Type   string          `json:"type"`
	Bearer []AuthAttribute `json:"bearer,omitempty"`
}

type AuthAttribute struct {
	Key   string `json:"key"`
	Value string `json:"value"`
	Type  string `json:"type"`
}
