import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { z } from "@hono/zod-openapi";
import { OpenAPIHono as Hono, createRoute } from "@hono/zod-openapi";
import { env } from "hono/adapter";
import { schemas } from "../client";

// Error response schema
const ErrorResponseSchema = z
  .object({
    error: z.string(),
    code: z.string().optional(),
    details: z.any().optional(),
  })
  .openapi({
    title: "Error Response",
    description: "Standard error response object",
  });

// Route handler for /lead-scraper-microservice/api/v1/organizations/tenants/api-keys
const router = new Hono<{ Bindings: Env }>();

const postRoute = createRoute({
  method: "post",
  path: "/",
  tags: [""],
  summary: "Create a new tenant API key",
  description: "Creates a new API key for a tenant with specified permissions",
  request: {
    body: {
      content: {
        "application/json": {
          schema: schemas.CreateTenantAPIKeyRequest,
        },
      },
    },
  },
  responses: {
    201: {
      content: {
        "application/json": {
          schema: z.object({}).openapi({
            type: "object",
            title: "EmptyResponse",
            description: "Empty response object",
          }),
        },
      },
      description:
        "Creates a new API key for a tenant with specified permissions",
    },
    400: {
      content: {
        "application/json": {
          schema: ErrorResponseSchema,
        },
      },
      description: "Returns an error",
    },
  },
});

router.openapi(postRoute, async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  try {
    const data = await c.req.json();
    const response =
      await client.createLeadScraperMicroserviceApiV1OrganizationsTenantsApiKeys(
        data,
      );
    return c.json({ data: response }, 201);
  } catch (error) {
    if (error instanceof HTTPException) {
      throw error;
    }
    return c.json(
      {
        error: error.message || "Internal Server Error",
        code: "INTERNAL_ERROR",
      },
      400,
    );
  }
});

const putRoute = createRoute({
  method: "put",
  path: "/",
  tags: [""],
  summary: "Update tenant API key",
  description: "Updates an existing tenant API key&#x27;s configuration",
  request: {
    body: {
      content: {
        "application/json": {
          schema: schemas.UpdateTenantAPIKeyRequest,
        },
      },
    },
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: schemas.UpdateTenantAPIKeyResponse,
        },
      },
      description: "Updates an existing tenant API key&#x27;s configuration",
    },
    400: {
      content: {
        "application/json": {
          schema: ErrorResponseSchema,
        },
      },
      description: "Returns an error",
    },
  },
});

router.openapi(putRoute, async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  try {
    const data = await c.req.json();
    const response =
      await client.updateLeadScraperMicroserviceApiV1OrganizationsTenantsApiKeys(
        data,
      );
    return c.json({ data: response }, 200);
  } catch (error) {
    if (error instanceof HTTPException) {
      throw error;
    }
    return c.json(
      {
        error: error.message || "Internal Server Error",
        code: "INTERNAL_ERROR",
      },
      400,
    );
  }
});

export const leadScraperMicroserviceapiv1organizationstenantsapiKeysRouter =
  router;
