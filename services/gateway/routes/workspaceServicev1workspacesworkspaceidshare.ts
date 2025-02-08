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

// Route handler for /workspace-service/v1/workspaces/{workspaceId}/share
const router = new Hono<{ Bindings: Env }>();

const postRoute = createRoute({
  method: "post",
  path: "/",
  tags: [""],
  summary: "Share workspace",
  description: "",
  request: {
    body: {
      content: {
        "application/json": {
          schema: wrapSchema(schemas.ShareWorkspaceBody, "ShareWorkspaceBody"),
        },
      },
    },
  },
  responses: {
    201: {
      content: {
        "application/json": {
          schema: z
            .object({
              data: z.any().openapi({ type: "object" }),
            })
            .openapi({
              title: "Success Response",
              description: "",
            }),
        },
      },
      description: "",
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
    };
    const response =
      await client.createWorkspaceServiceV1WorkspacesWorkspaceIdShare(
        data,
        params,
      );
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

export const workspaceServicev1workspacesworkspaceidshareRouter = router;
