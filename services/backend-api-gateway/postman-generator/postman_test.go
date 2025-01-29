package postman

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/luraproject/lura/v2/config"
	"github.com/stretchr/testify/assert"
)

func TestParse(t *testing.T) {
	tests := []struct {
		name     string
		config   config.ServiceConfig
		validate func(*testing.T, Collection)
	}{
		{
			name: "basic endpoints",
			config: config.ServiceConfig{
				Port: 8080,
				Name: "test-api",
				Endpoints: []*config.EndpointConfig{
					{
						Endpoint: "/foo",
						Method:   "GET",
					},
					{
						Endpoint: "/bar",
						Method:   "POST",
					},
				},
			},
			validate: func(t *testing.T, c Collection) {
				assert.Equal(t, "test-api", c.Info.Name)
				assert.Equal(t, 2, len(c.Item))
				assert.Equal(t, "/foo", c.Item[0].Name)
				assert.Equal(t, "GET", c.Item[0].Request.Method)
				assert.Equal(t, "/bar", c.Item[1].Name)
				assert.Equal(t, "POST", c.Item[1].Request.Method)
				assert.NotNil(t, c.Item[1].Request.Body)
			},
		},
		{
			name: "with TLS",
			config: config.ServiceConfig{
				Port: 8443,
				Name: "secure-api",
				TLS:  &config.TLS{},
			},
			validate: func(t *testing.T, c Collection) {
				assert.Equal(t, "secure-api", c.Info.Name)
				assert.Equal(t, "https", c.Variables[1].Value)
			},
		},
		{
			name: "complex path",
			config: config.ServiceConfig{
				Port: 8080,
				Name: "path-api",
				Endpoints: []*config.EndpointConfig{
					{
						Endpoint: "/api/v1/users/{id}",
						Method:   "GET",
					},
				},
			},
			validate: func(t *testing.T, c Collection) {
				assert.Equal(t, 4, len(c.Item[0].Request.URL.Path))
				assert.Equal(t, "api", c.Item[0].Request.URL.Path[0])
				assert.Equal(t, "v1", c.Item[0].Request.URL.Path[1])
				assert.Equal(t, "users", c.Item[0].Request.URL.Path[2])
				assert.Equal(t, "{id}", c.Item[0].Request.URL.Path[3])
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			collection := Parse(tt.config)
			tt.validate(t, collection)
		})
	}
}

func TestHandleCollection(t *testing.T) {
	cfg := config.ServiceConfig{
		Port: 8080,
		Name: "test-api",
		Endpoints: []*config.EndpointConfig{
			{
				Endpoint: "/test",
				Method:   "GET",
			},
		},
	}

	ts := httptest.NewServer(http.HandlerFunc(HandleCollection(Parse(cfg))))
	defer ts.Close()

	resp, err := http.Get(ts.URL)
	assert.NoError(t, err)
	defer resp.Body.Close()

	assert.Equal(t, http.StatusOK, resp.StatusCode)
	assert.Equal(t, "application/json", resp.Header.Get("Content-Type"))

	var collection Collection
	err = json.NewDecoder(resp.Body).Decode(&collection)
	assert.NoError(t, err)
	assert.Equal(t, "test-api", collection.Info.Name)
}

func ExampleParse() {
	c := Parse(config.ServiceConfig{
		Port: 8080,
		Name: "sample",
		Endpoints: []*config.EndpointConfig{
			{
				Endpoint: "/foo",
				Method:   "GET",
			},
			{
				Endpoint: "/bar",
				Method:   "POST",
			},
		},
	})
	fmt.Println(c.Info.Name)
	fmt.Println(c.Info.Schema)
	fmt.Println(len(c.Item))
	fmt.Printf("%+v\n", c.Item[0].Name)
	fmt.Printf("%+v\n", c.Item[1].Name)
	// output:
	// sample
	// https://schema.getpostman.com/json/collection/v2.1.0/collection.json
	// 2
	// /foo
	// /bar
}

func TestIsMethodWithBody(t *testing.T) {
	tests := []struct {
		method string
		want   bool
	}{
		{"GET", false},
		{"POST", true},
		{"PUT", true},
		{"PATCH", true},
		{"DELETE", false},
		{"HEAD", false},
	}

	for _, tt := range tests {
		t.Run(tt.method, func(t *testing.T) {
			got := isMethodWithBody(tt.method)
			assert.Equal(t, tt.want, got)
		})
	}
}

func TestSplitPath(t *testing.T) {
	tests := []struct {
		path string
		want []string
	}{
		{"/simple", []string{"simple"}},
		{"/api/v1/users", []string{"api", "v1", "users"}},
		{"/api/v1/users/{id}", []string{"api", "v1", "users", "{id}"}},
		{"//double//slash", []string{"double", "slash"}},
		{"/", []string{}},
		{"", []string{}},
		{"/users/{id}/profile", []string{"users", "{id}", "profile"}},
		{"/users{id}", []string{"users", "{id}"}},
	}

	for _, tt := range tests {
		t.Run(tt.path, func(t *testing.T) {
			got := splitPath(tt.path)
			assert.Equal(t, tt.want, got)
		})
	}
}
