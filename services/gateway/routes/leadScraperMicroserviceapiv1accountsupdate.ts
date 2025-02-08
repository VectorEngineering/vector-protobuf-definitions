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

// Route handler for /lead-scraper-microservice/api/v1/accounts/update
const router = new Hono<{ Bindings: Env }>();

const putRoute = createRoute({
  method: "put",
  path: "/",
  tags: [""],
  summary: "Update account details",
  description: "Updates specified fields of an existing account",
  request: {
    body: {
      content: {
        "application/json": {
          schema: wrapSchema(
            schemas.UpdateAccountRequest,
            "UpdateAccountRequest",
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
                schemas.UpdateAccountResponse,
                "UpdateAccountResponse",
              ),
            })
            .openapi({
              title: "Success Response",
              description: "Account updated successfully",
            }),
        },
      },
      description: "Updates specified fields of an existing account",
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
    const response =
      await client.updateLeadScraperMicroserviceApiV1AccountsUpdate(data);
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

export const leadScraperMicroserviceapiv1accountsupdateRouter = router;
