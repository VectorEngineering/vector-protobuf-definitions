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

// Route handler for /workspace-service/v1/workspaces/list
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "List workspaces",
  description: "",
  request: {
    query: z.object({
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
      pageToken: z
        .string()
        .optional()
        .openapi({
          param: {
            name: "pageToken",
            in: "query",
            required: false,
            description: "",
          },
        }),
      filter: z
        .string()
        .optional()
        .openapi({
          param: {
            name: "filter",
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
            data: schemas.ListWorkspacesResponse1,
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

router.openapi(getRoute, async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  try {
    const params = {
      accountId: c.req.valid("query").accountId,
      pageSize: c.req.valid("query").pageSize,
      pageToken: c.req.valid("query").pageToken,
      filter: c.req.valid("query").filter,
    };
    const response = await client.getWorkspaceServiceV1WorkspacesList(params);
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

export const workspaceServicev1workspaceslistRouter = router;
