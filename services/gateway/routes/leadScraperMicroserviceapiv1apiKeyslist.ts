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

// Route handler for /lead-scraper-microservice/api/v1/api-keys/list
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "List API keys",
  description: "Retrieves a list of API keys with optional filtering",
  request: {
    query: z.object({
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
      search: z
        .string()
        .optional()
        .openapi({
          param: {
            name: "search",
            in: "query",
            required: false,
            description: "",
          },
        }),
    }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: z.object({
            data: schemas.ListAPIKeysResponse,
          }),
        },
      },
      description: "Retrieves a list of API keys with optional filtering",
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
      accountId: c.req.valid("query").accountId,
      workspaceId: c.req.valid("query").workspaceId,
      pageSize: c.req.valid("query").pageSize,
      pageNumber: c.req.valid("query").pageNumber,
      status: c.req.valid("query").status,
      search: c.req.valid("query").search,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1ApiKeysList(params);
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

export const leadScraperMicroserviceapiv1apiKeyslistRouter = router;
