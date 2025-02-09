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

// Route handler for /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "List workflows",
  description: "Retrieves a list of workflows for a given workspace",
  request: {
    params: z.object({
      workspaceId: z.string(),
    }),
    query: z.object({
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
    }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: z.object({
            data: schemas.ListWorkflowsResponse,
          }),
        },
      },
      description: "Retrieves a list of workflows for a given workspace",
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
      pageSize: c.req.valid("query").pageSize,
      pageNumber: c.req.valid("query").pageNumber,
      filter: c.req.valid("query").filter,
      organizationId: c.req.valid("query").organizationId,
      tenantId: c.req.valid("query").tenantId,
      accountId: c.req.valid("query").accountId,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflows(
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

const postRoute = createRoute({
  method: "post",
  path: "/",
  tags: [""],
  summary: "Create a new workflow",
  description: "Creates a new workflow for a specific workspace",
  request: {
    params: z.object({
      workspaceId: z.string(),
    }),
    body: {
      content: {
        "application/json": {
          schema: schemas.CreateWorkflowBody,
        },
      },
    },
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: schemas.CreateWorkflowResponse,
        },
      },
      description: "Creates a new workflow for a specific workspace",
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
      await client.createLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflows(
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

export const leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsRouter =
  router;
