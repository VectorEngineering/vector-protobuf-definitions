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

// Route handler for /lead-scraper-microservice/api/v1/accounts/list
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "List all accounts",
  description: "Retrieves a list of accounts",
  request: {
    query: z
      .object({
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
        filter: z
          .string()
          .optional()
          .openapi({
            param: {
              name: "filter",
              in: "query",
              required: false,
              description: "",
            },
          }),
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
                schemas.ListAccountsResponse,
                "ListAccountsResponse",
              ),
            })
            .openapi({
              title: "Success Response",
              description: "Accounts retrieved successfully",
            }),
        },
      },
      description: "Retrieves a list of accounts",
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
      pageSize: c.req.valid("query").pageSize,
      pageNumber: c.req.valid("query").pageNumber,
      filter: c.req.valid("query").filter,
      organizationId: c.req.valid("query").organizationId,
      tenantId: c.req.valid("query").tenantId,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1AccountsList(params);
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

export const leadScraperMicroserviceapiv1accountslistRouter = router;
