// Client authentication middleware
import type { ZodiosEndpointDefinition } from "@zodios/core";

interface SecurityScheme {
  ApiKeyAuth?: Record<string, unknown>;
}

export const authMiddleware = {
  name: "auth",
  type: "request",
  beforeRequest: async (
    req: { headers: Record<string, string>; url: string; method: string },
    apiEndpoint?: ZodiosEndpointDefinition & { security?: SecurityScheme[] },
  ) => {
    // Check if this endpoint requires ApiKeyAuth
    const requiresAuth = apiEndpoint?.security?.some(
      (scheme) => "ApiKeyAuth" in scheme,
    );

    if (!requiresAuth) {
      return req;
    }

    const authHeader =
      req.headers["Authorization"] || req.headers["authorization"];
    const apiKey = req.headers["X-API-Key"] || req.headers["x-api-key"];

    // First check for Bearer token authentication
    if (authHeader?.startsWith("Bearer ")) {
      const token = authHeader.substring(7);
      if (!token) {
        throw new Error("Invalid bearer token");
      }
      return req;
    }

    // Fall back to API key authentication
    if (!apiKey) {
      throw new Error(
        "Missing authentication. Please provide either a Bearer token or API key",
      );
    }

    return req;
  },
};
