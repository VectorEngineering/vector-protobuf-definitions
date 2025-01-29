package models

func DefaultEndpointExtraConfig(environment string) map[string]interface{} {
	if environment == "production" {
		return map[string]interface{}{
			"qos/ratelimit/router": map[string]interface{}{
				"@comment": "A hundred thousand requests every second",
				"max_rate": 6000000,
				"every":    "1m",
			},
			// TODO: as soon as supabase ships symmetric jwt validation with jwk url please enable the below operation
			// TODO: @ref: https://github.com/supabase/auth-js/issues/873#issuecomment-2092401090
			// "auth/validator": map[string]interface{}{
			// 	"disable_jwk_security": false,
			// 	"operation_debug":      true,
			// 	"cache":                true,
			// 	"cache_duration":       3600,
			// 	"alg":                  "RS256",
			// 	"audience":             []string{"https://production.solomon-ai.dev"},
			// 	"jwk_url":              "https://solomon-ai.us.auth0.com/.well-known/jwks.json",
			// },
		}
	}

	return map[string]interface{}{
		"qos/ratelimit/router": map[string]interface{}{
			"@comment": "six thousand requests every second",
			"max_rate": 60000,
			"every":    "1m",
		},
		// TODO: as soon as supabase ships symmetric jwt validation with jwk url please enable the below operation
		// TODO: @ref: https://github.com/supabase/auth-js/issues/873#issuecomment-2092401090
		// "auth/validator": map[string]interface{}{
		// 	"disable_jwk_security": false,
		// 	"operation_debug":      true,
		// 	"cache":                true,
		// 	"cache_duration":       3600,
		// 	"alg":                  "RS256",
		// 	"audience":             []string{"https://staging.solomon-ai.dev"},
		// 	"jwk_url":              "https://dev-26i1fsf2m1n6hqk1.us.auth0.com/.well-known/jwks.json",
		// },
	}
}

func DefaultBackendExtraConfig() map[string]interface{} {
	return map[string]interface{}{
		"modifier/martian": map[string]interface{}{
			"fifo.Group": map[string]interface{}{
				"scope":           []string{"request", "response"},
				"aggregateErrors": true,
				"modifiers": []map[string]interface{}{
					{
						"header.Append": map[string]interface{}{
							"scope": []string{"request", "response"},
							"name":  "X-PLATFORM",
							"value": "solomon-ai",
						},
					},
				},
			},
		},
		"qos/circuit-breaker": map[string]interface{}{
			"interval":          30,
			"timeout":           10,
			"max_errors":        5,
			"log_status_change": true,
			"name":              "solomon-circuit-breaker",
		},
		"backend/http": map[string]interface{}{
			"return_error_details": "message",
		},
	}
}
