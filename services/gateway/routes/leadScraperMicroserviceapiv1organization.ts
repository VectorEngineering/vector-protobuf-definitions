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

// Route handler for /lead-scraper-microservice/api/v1/organization
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "List all organizations",
  description: "Retrieves a list of all organizations in a tenant",
  request: {
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
    }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: z.object({
            data: schemas.ListOrganizationsResponse,
          }),
        },
      },
      description: "Retrieves a list of all organizations in a tenant",
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
      pageSize: c.req.valid("query").pageSize,
      pageNumber: c.req.valid("query").pageNumber,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1Organization(params);
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
  summary: "Create a new organization",
  description: "Creates a new organization within a tenant",
  request: {
    body: {
      content: {
        "application/json": {
          schema: schemas.CreateOrganizationRequest,
        },
      },
    },
  },
  responses: {
    201: {
      content: {
        "application/json": {
          schema: schemas.CreateOrganizationResponse,
        },
      },
      description: "Creates a new organization within a tenant",
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
      await client.createLeadScraperMicroserviceApiV1Organization(data);
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
  summary: "Update organization details",
  description: "Updates the configuration of a specific organization",
  request: {
    body: {
      content: {
        "application/json": {
          schema: schemas.UpdateOrganizationRequest,
        },
      },
    },
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: schemas.UpdateOrganizationResponse,
        },
      },
      description: "Updates the configuration of a specific organization",
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
      await client.updateLeadScraperMicroserviceApiV1Organization(data);
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

export const leadScraperMicroserviceapiv1organizationRouter = router;
