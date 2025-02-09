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

// Route handler for /workspace-service/v1/workspace-sharings/{sharingId}
const router = new Hono<{ Bindings: Env }>();

const deleteRoute = createRoute({
  method: "delete",
  path: "/",
  tags: [""],
  summary: "Remove workspace sharing",
  description: "",
  request: {
    params: z.object({
      sharingId: z.string(),
    }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: schemas.RemoveWorkspaceSharingResponse,
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

router.openapi(deleteRoute, async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  try {
    const params = {
      sharingId:
        c.req.param("sharingId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: sharingId",
          });
        })(),
    };
    const response =
      await client.deleteWorkspaceServiceV1WorkspaceSharingsSharingId(params);
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

export const workspaceServicev1workspaceSharingssharingidRouter = router;
