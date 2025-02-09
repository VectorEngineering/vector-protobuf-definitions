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

// Route handler for /lead-scraper-microservice/api/v1/jobs
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "Get all jobs",
  description: "This endpoint retrieves all Google Maps scraping jobs",
  request: {
    query: z.object({
      authPlatformUserId: z.string().openapi({
        param: {
          name: "authPlatformUserId",
          in: "query",
          required: true,
          description: "",
        },
      }),
      orgId: z.string().openapi({
        param: {
          name: "orgId",
          in: "query",
          required: true,
          description: "",
        },
      }),
      tenantId: z.string().openapi({
        param: {
          name: "tenantId",
          in: "query",
          required: true,
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
            data: schemas.ListScrapingJobsResponse,
          }),
        },
      },
      description: "This endpoint retrieves all Google Maps scraping jobs",
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
      authPlatformUserId: c.req.valid("query").authPlatformUserId,
      orgId: c.req.valid("query").orgId,
      tenantId: c.req.valid("query").tenantId,
    };
    const response = await client.getLeadScraperMicroserviceApiV1Jobs(params);
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
  summary: "Create a new job scraping task",
  description: "This endpoint creates a new Google Maps scraping job",
  request: {
    body: {
      content: {
        "application/json": {
          schema: schemas.CreateScrapingJobRequest,
        },
      },
    },
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: schemas.CreateScrapingJobResponse,
        },
      },
      description: "This endpoint creates a new Google Maps scraping job",
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
    const response = await client.createLeadScraperMicroserviceApiV1Jobs(data);
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

export const leadScraperMicroserviceapiv1jobsRouter = router;
