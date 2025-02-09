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

// Route handler for /lead-scraper-microservice/api/v1/accounts/{id}
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "Get account details",
  description: "Retrieves details of a specific account",
  request: {
    params: z.object({
      id: z.string(),
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
    }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: z.object({
            data: schemas.GetAccountResponse,
          }),
        },
      },
      description: "Retrieves details of a specific account",
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
      id:
        c.req.param("id") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: id",
          });
        })(),
      organizationId: c.req.valid("query").organizationId,
      tenantId: c.req.valid("query").tenantId,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1AccountsId(params);
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
  summary: "Delete account",
  description: "Permanently deletes an account and associated resources",
  request: {
    params: z.object({
      id: z.string(),
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
    }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: schemas.DeleteAccountResponse,
        },
      },
      description: "Permanently deletes an account and associated resources",
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
      id:
        c.req.param("id") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: id",
          });
        })(),
      organizationId: c.req.valid("query").organizationId,
      tenantId: c.req.valid("query").tenantId,
    };
    const response =
      await client.deleteLeadScraperMicroserviceApiV1AccountsId(params);
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

export const leadScraperMicroserviceapiv1accountsidRouter = router;
