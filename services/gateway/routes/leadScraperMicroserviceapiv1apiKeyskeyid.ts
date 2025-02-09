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

// Route handler for /lead-scraper-microservice/api/v1/api-keys/{keyId}
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "Get API key details",
  description: "Retrieves details of a specific API key",
  request: {
    params: z.object({
      keyId: z.string(),
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
    }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: z.object({
            data: schemas.GetAPIKeyResponse,
          }),
        },
      },
      description: "Retrieves details of a specific API key",
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
      keyId:
        c.req.param("keyId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: keyId",
          });
        })(),
      organizationId: c.req.valid("query").organizationId,
      tenantId: c.req.valid("query").tenantId,
      accountId: c.req.valid("query").accountId,
      workspaceId: c.req.valid("query").workspaceId,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1ApiKeysKeyId(params);
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
  summary: "Delete API key",
  description: "Permanently deletes an API key",
  request: {
    params: z.object({
      keyId: z.string(),
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
          schema: schemas.DeleteAPIKeyResponse,
        },
      },
      description: "Permanently deletes an API key",
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
      keyId:
        c.req.param("keyId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: keyId",
          });
        })(),
      organizationId: c.req.valid("query").organizationId,
      workspaceId: c.req.valid("query").workspaceId,
      tenantId: c.req.valid("query").tenantId,
      accountId: c.req.valid("query").accountId,
    };
    const response =
      await client.deleteLeadScraperMicroserviceApiV1ApiKeysKeyId(params);
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

export const leadScraperMicroserviceapiv1apiKeyskeyidRouter = router;
