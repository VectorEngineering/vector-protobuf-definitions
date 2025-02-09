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

// Route handler for /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/analytics
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "Get workspace analytics",
  description: "Retrieves analytics data for a specific workspace",
  request: {
    params: z.object({
      workspaceId: z.string(),
    }),
    query: z.object({
      startTime: z
        .string()
        .optional()
        .openapi({
          param: {
            name: "startTime",
            in: "query",
            required: false,
            description: "",
          },
        }),
      endTime: z
        .string()
        .optional()
        .openapi({
          param: {
            name: "endTime",
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
            data: schemas.GetWorkspaceAnalyticsResponse,
          }),
        },
      },
      description: "Retrieves analytics data for a specific workspace",
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
      workspaceId:
        c.req.param("workspaceId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: workspaceId",
          });
        })(),
      startTime: c.req.valid("query").startTime,
      endTime: c.req.valid("query").endTime,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdAnalytics(
        params,
      );
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

export const leadScraperMicroserviceapiv1workspacesworkspaceidanalyticsRouter =
  router;
