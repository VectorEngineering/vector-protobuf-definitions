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

// Route handler for /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger
const router = new Hono<{ Bindings: Env }>();

const postRoute = createRoute({
  method: "post",
  path: "/",
  tags: [""],
  summary: "Trigger workflow execution",
  description: "Triggers the execution of a specific workflow",
  request: {
    params: z.object({
      workspaceId: z.string(),
      id: z.string(),
    }),
    body: {
      content: {
        "application/json": {
          schema: schemas.TriggerWorkflowBody,
        },
      },
    },
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: schemas.TriggerWorkflowResponse,
        },
      },
      description: "Triggers the execution of a specific workflow",
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
      workspaceId:
        c.req.param("workspaceId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: workspaceId",
          });
        })(),
      id:
        c.req.param("id") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: id",
          });
        })(),
    };
    const response =
      await client.createLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflowsIdTrigger(
        data,
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

export const leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidtriggerRouter =
  router;
