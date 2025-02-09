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

// Route handler for /lead-scraper-microservice/api/v1/jobs/{jobId}
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "Get a specific job",
  description: "This endpoint retrieves a specific Google Maps scraping job",
  request: {
    params: z.object({
      jobId: z.string(),
    }),
    query: z.object({
      userId: z.string().openapi({
        param: {
          name: "userId",
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
            data: schemas.GetScrapingJobResponse,
          }),
        },
      },
      description:
        "This endpoint retrieves a specific Google Maps scraping job",
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
      jobId:
        c.req.param("jobId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: jobId",
          });
        })(),
      userId: c.req.valid("query").userId,
      orgId: c.req.valid("query").orgId,
      tenantId: c.req.valid("query").tenantId,
    };
    const response =
      await client.getLeadScraperMicroserviceApiV1JobsJobId(params);
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

const deleteRoute = createRoute({
  method: "delete",
  path: "/",
  tags: [""],
  summary: "Delete a specific job",
  description: "This endpoint deletes a specific Google Maps scraping job",
  request: {
    params: z.object({
      jobId: z.string(),
    }),
    query: z.object({
      userId: z.string().openapi({
        param: {
          name: "userId",
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
          schema: schemas.DeleteScrapingJobResponse,
        },
      },
      description: "This endpoint deletes a specific Google Maps scraping job",
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
      jobId:
        c.req.param("jobId") ||
        (() => {
          throw new HTTPException(400, {
            message: "Missing required path parameter: jobId",
          });
        })(),
      userId: c.req.valid("query").userId,
      orgId: c.req.valid("query").orgId,
      tenantId: c.req.valid("query").tenantId,
    };
    const response =
      await client.deleteLeadScraperMicroserviceApiV1JobsJobId(params);
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

export const leadScraperMicroserviceapiv1jobsjobidRouter = router;
