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

// Route handler for /lead-scraper-microservice/api/v1/leads/{leadId}
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "Get lead details",
  description: "Retrieves detailed information about a specific lead",
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
        workspaceId: z
          .string()
          .optional()
          .openapi({
            param: {
              name: "workspaceId",
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
        accountId: z
          .string()
          .optional()
          .openapi({
            param: {
              name: "accountId",
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
              data: wrapSchema(schemas.GetLeadResponse, "GetLeadResponse"),
            })
            .openapi({
              title: "Success Response",
              description: "Lead retrieved successfully",
            }),
        },
      },
      description: "Retrieves detailed information about a specific lead",
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
      leadId:
        c.req.param("leadId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: leadId",
          });
        })(),
      organizationId: c.req.valid("query").organizationId,
      workspaceId: c.req.valid("query").workspaceId,
      tenantId: c.req.valid("query").tenantId,
      accountId: c.req.valid("query").accountId,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1LeadsLeadId(params);
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

export const leadScraperMicroserviceapiv1leadsleadidRouter = router;
