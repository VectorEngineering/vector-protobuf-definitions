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

// Route handler for /workspace-service/v1/accounts
const router = new Hono<{ Bindings: Env }>();

const postRoute = createRoute({
  method: "post",
  path: "/",
  tags: [""],
  summary: "Create a new account",
  description: "Creates a new user account with initial workspace",
  request: {
    body: {
      content: {
        "application/json": {
          schema: wrapSchema(
            schemas.CreateAccountRequest,
            "CreateAccountRequest",
          ),
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
              data: wrapSchema(
                schemas.CreateAccountResponse,
                "CreateAccountResponse",
              ),
            })
            .openapi({
              title: "Success Response",
              description: "Account created successfully",
            }),
        },
      },
      description: "Creates a new user account with initial workspace",
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
    const response = await client.createWorkspaceServiceV1Accounts(data);
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
  summary: "Update account details",
  description: "",
  request: {
    body: {
      content: {
        "application/json": {
          schema: wrapSchema(
            schemas.UpdateAccountRequest1,
            "UpdateAccountRequest1",
          ),
        },
      },
    },
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: z
            .object({
              data: wrapSchema(
                schemas.UpdateAccountResponse1,
                "UpdateAccountResponse1",
              ),
            })
            .openapi({
              title: "Success Response",
              description: "Account updated successfully",
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

router.openapi(putRoute, async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  try {
    const data = await c.req.json();
    const response = await client.updateWorkspaceServiceV1Accounts(data);
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

export const workspaceServicev1accountsRouter = router;
