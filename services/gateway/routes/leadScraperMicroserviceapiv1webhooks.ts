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

// Route handler for /lead-scraper-microservice/api/v1/webhooks
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "List webhooks",
  description: "Lists all webhook configurations with pagination",
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
            data: schemas.ListWebhooksResponse,
          }),
        },
      },
      description: "Lists all webhook configurations with pagination",
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
      workspaceId: c.req.valid("query").workspaceId,
      tenantId: c.req.valid("query").tenantId,
      accountId: c.req.valid("query").accountId,
      pageSize: c.req.valid("query").pageSize,
      pageNumber: c.req.valid("query").pageNumber,
      status: c.req.valid("query").status,
      search: c.req.valid("query").search,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1Webhooks(params);
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

const postRoute = createRoute({
  method: "post",
  path: "/",
  tags: [""],
  summary: "Create webhook",
  description: "Creates a new webhook configuration for event notifications",
  request: {
    body: {
      content: {
        "application/json": {
          schema: schemas.CreateWebhookRequest,
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
        "Creates a new webhook configuration for event notifications",
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
      await client.createLeadScraperMicroserviceApiV1Webhooks(data);
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
  summary: "Update webhook",
  description: "Updates an existing webhook configuration",
  request: {
    body: {
      content: {
        "application/json": {
          schema: schemas.UpdateWebhookRequest,
        },
      },
    },
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: schemas.UpdateWebhookResponse,
        },
      },
      description: "Updates an existing webhook configuration",
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
      await client.updateLeadScraperMicroserviceApiV1Webhooks(data);
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

export const leadScraperMicroserviceapiv1webhooksRouter = router;
