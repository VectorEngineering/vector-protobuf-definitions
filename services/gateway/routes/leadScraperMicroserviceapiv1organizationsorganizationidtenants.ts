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

// Route handler for /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants
const router = new Hono<{ Bindings: Env }>();

const postRoute = createRoute({
  method: "post",
  path: "/",
  tags: [""],
  summary: "Create a new tenant",
  description: "Creates a new tenant in the system",
  request: {
    params: z.object({
      organizationId: z.string(),
    }),
    body: {
      content: {
        "application/json": {
          schema: schemas.CreateTenantBody,
        },
      },
    },
  },
  responses: {
    201: {
      content: {
        "application/json": {
          schema: schemas.CreateTenantResponse,
        },
      },
      description: "Creates a new tenant in the system",
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
    const params = {
      organizationId:
        c.req.param("organizationId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: organizationId",
          });
        })(),
    };
    const response =
      await client.createLeadScraperMicroserviceApiV1OrganizationsOrganizationIdTenants(
        data,
        params,
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

export const leadScraperMicroserviceapiv1organizationsorganizationidtenantsRouter =
  router;
