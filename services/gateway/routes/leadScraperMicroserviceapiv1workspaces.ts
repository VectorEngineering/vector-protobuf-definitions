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

// Route handler for /lead-scraper-microservice/api/v1/workspaces
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "List workspaces",
  description: "Retrieves a list of workspaces for a given account",
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
      pageNumber: z
        .number()
        .optional()
        .openapi({
          param: {
            name: "pageNumber",
            in: "query",
            required: false,
            description: "",
          },
        }),
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
            data: schemas.ListWorkspacesResponse,
          }),
        },
      },
      description: "Retrieves a list of workspaces for a given account",
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
      pageNumber: c.req.valid("query").pageNumber,
      organizationId: c.req.valid("query").organizationId,
      tenantId: c.req.valid("query").tenantId,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1Workspaces(params);
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

const postRoute = createRoute({
  method: "post",
  path: "/",
  tags: [""],
  summary: "Create a new workspace",
  description: "Creates a new workspace for a given account",
  request: {
    body: {
      content: {
        "application/json": {
          schema: schemas.CreateWorkspaceRequest,
        },
      },
    },
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: schemas.CreateWorkspaceResponse,
        },
      },
      description: "Creates a new workspace for a given account",
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
    const response =
      await client.createLeadScraperMicroserviceApiV1Workspaces(data);
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

export const leadScraperMicroserviceapiv1workspacesRouter = router;
