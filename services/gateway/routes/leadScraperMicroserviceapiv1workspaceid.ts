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

// Route handler for /lead-scraper-microservice/api/v1/workspace/{id}
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "Get workspace details",
  description: "Retrieves details of a specific workspace",
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
              data: wrapSchema(
                schemas.GetWorkspaceResponse,
                "GetWorkspaceResponse",
              ),
            })
            .openapi({
              title: "Success Response",
              description: "Workspace retrieved successfully",
            }),
        },
      },
      description: "Retrieves details of a specific workspace",
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
      accountId: c.req.valid("query").accountId,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1WorkspaceId(params);
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
  summary: "Delete a workspace",
  description: "Deletes a specific workspace",
  request: {},
  responses: {
    200: {
      content: {
        "application/json": {
          schema: z
            .object({
              success: z.boolean(),
            })
            .openapi({
              title: "Success Response",
              description: "Workspace deleted successfully",
            }),
        },
      },
      description: "Deletes a specific workspace",
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
    };
    const response =
      await client.deleteLeadScraperMicroserviceApiV1WorkspaceId(params);
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

export const leadScraperMicroserviceapiv1workspaceidRouter = router;
