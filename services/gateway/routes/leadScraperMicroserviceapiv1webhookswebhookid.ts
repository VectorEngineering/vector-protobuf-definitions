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

// Route handler for /lead-scraper-microservice/api/v1/webhooks/{webhookId}
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "Get webhook",
  description: "Retrieves details of a specific webhook configuration",
  request: {
    params: z.object({
      webhookId: z.string(),
    }),
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
    }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: z.object({
            data: schemas.GetWebhookResponse,
          }),
        },
      },
      description: "Retrieves details of a specific webhook configuration",
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
      webhookId:
        c.req.param("webhookId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: webhookId",
          });
        })(),
      organizationId: c.req.valid("query").organizationId,
      workspaceId: c.req.valid("query").workspaceId,
      tenantId: c.req.valid("query").tenantId,
      accountId: c.req.valid("query").accountId,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1WebhooksWebhookId(params);
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

const deleteRoute = createRoute({
  method: "delete",
  path: "/",
  tags: [""],
  summary: "Delete webhook",
  description: "Deletes a webhook configuration",
  request: {
    params: z.object({
      webhookId: z.string(),
    }),
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
    }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: schemas.DeleteWebhookResponse,
        },
      },
      description: "Deletes a webhook configuration",
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
      webhookId:
        c.req.param("webhookId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: webhookId",
          });
        })(),
      organizationId: c.req.valid("query").organizationId,
      workspaceId: c.req.valid("query").workspaceId,
      tenantId: c.req.valid("query").tenantId,
      accountId: c.req.valid("query").accountId,
    };
    const response =
      await client.deleteLeadScraperMicroserviceApiV1WebhooksWebhookId(params);
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

export const leadScraperMicroserviceapiv1webhookswebhookidRouter = router;
