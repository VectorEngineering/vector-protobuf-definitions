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

// Route handler for /lead-scraper-microservice/api/v1/jobs/{jobId}/download
const router = new Hono<{ Bindings: Env }>();

const getRoute = createRoute({
  method: "get",
  path: "/",
  tags: [""],
  summary: "Download job results as CSV",
  description:
    "This endpoint downloads the results of a Google Maps scraping job as CSV",
  request: {
    query: z
      .object({
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
      })
      .openapi({
        title: "Query Parameters",
        description: "Query parameters for the request",
      }),
  },
  responses: {
    200: {
      content: {
        "application/json": {
          schema: z
            .object({
              data: wrapSchema(
                schemas.DownloadScrapingResultsResponse,
                "DownloadScrapingResultsResponse",
              ),
            })
            .openapi({
              title: "Success Response",
              description: "Successful response",
            }),
        },
      },
      description:
        "This endpoint downloads the results of a Google Maps scraping job as CSV",
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
      await client.getLeadScraperMicroserviceApiV1JobsJobIdDownload(params);
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

export const leadScraperMicroserviceapiv1jobsjobiddownloadRouter = router;
