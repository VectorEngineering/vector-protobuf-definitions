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

// Wrap imported schemas with OpenAPI metadata
const wrapSchema = (schema: any, title: string) => {
  return z
    .lazy(() => schema)
    .openapi({
      type: "object",
      title: title,
    });
};

// Route handler for /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/list
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "List tenant API keys",
  description: "Retrieves all API keys for a specific tenant",
  request: {
    query: z
      .object({
        organizationId: z
          .string()
          .optional()
          .openapi({
            param: {
              name: "organizationId",
              in: "query",
              required: false,
              description: "",
            },
          }),
        tenantId: z
          .string()
          .optional()
          .openapi({
            param: {
              name: "tenantId",
              in: "query",
              required: false,
              description: "",
            },
          }),
        pageSize: z
          .number()
          .optional()
          .openapi({
            param: {
              name: "pageSize",
              in: "query",
              required: false,
              description: "",
            },
          }),
        pageNumber: z
          .number()
          .optional()
          .openapi({
            param: {
              name: "pageNumber",
              in: "query",
              required: false,
              description: "",
            },
          }),
        status: z
          .string()
          .optional()
          .openapi({
            param: {
              name: "status",
              in: "query",
              required: false,
              description: "",
            },
          }),
      })
      .openapi({
        title: "Query Parameters",
        description: "Query parameters for the request",
      }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: z
            .object({
              data: wrapSchema(
                schemas.ListTenantAPIKeysResponse,
                "ListTenantAPIKeysResponse",
              ),
            })
            .openapi({
              title: "Success Response",
              description: "Tenant API keys retrieved successfully",
            }),
        },
      },
      description: "Retrieves all API keys for a specific tenant",
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

router.openapi(getRoute, async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  try {
    const params = {
      organizationId: c.req.valid("query").organizationId,
      tenantId: c.req.valid("query").tenantId,
      pageSize: c.req.valid("query").pageSize,
      pageNumber: c.req.valid("query").pageNumber,
      status: c.req.valid("query").status,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1OrganizationsTenantsApiKeysList(
        params,
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

export const leadScraperMicroserviceapiv1organizationstenantsapiKeyslistRouter =
  router;
