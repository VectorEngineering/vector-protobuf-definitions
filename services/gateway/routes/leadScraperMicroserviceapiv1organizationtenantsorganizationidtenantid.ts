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

// Route handler for /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId}
const router = new Hono<{ Bindings: Env }>();

const deleteRoute = createRoute({
  method: "delete",
  path: "/",
  tags: [""],
  summary: "Delete a tenant",
  description: "Permanently deletes a tenant and all associated resources",
  request: {
    params: z.object({
      organizationId: z.string(),
      tenantId: z.string(),
    }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: schemas.DeleteTenantResponse,
        },
      },
      description: "Permanently deletes a tenant and all associated resources",
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

router.openapi(deleteRoute, async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  try {
    const params = {
      organizationId:
        c.req.param("organizationId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: organizationId",
          });
        })(),
      tenantId:
        c.req.param("tenantId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: tenantId",
          });
        })(),
    };
    const response =
      await client.deleteLeadScraperMicroserviceApiV1OrganizationTenantsOrganizationIdTenantId(
        params,
      );
    return c.json({ success: true }, 200);
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

export const leadScraperMicroserviceapiv1organizationtenantsorganizationidtenantidRouter =
  router;
