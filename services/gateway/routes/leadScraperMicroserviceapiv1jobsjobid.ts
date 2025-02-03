import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/jobs/{jobId}
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    jobId:
      c.req.param("jobId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: jobId",
        });
      })(),
    userId: c.req.query("userId") || "",
    orgId: c.req.query("orgId") || "",
    tenantId: c.req.query("tenantId") || "",
  };
  const response =
    await client.getLeadScraperMicroserviceApiV1JobsJobId(params);
  return c.json(response);
});

router.delete("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    jobId:
      c.req.param("jobId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: jobId",
        });
      })(),
    userId: c.req.query("userId") || "",
    orgId: c.req.query("orgId") || "",
    tenantId: c.req.query("tenantId") || "",
  };
  const response =
    await client.deleteLeadScraperMicroserviceApiV1JobsJobId(params);
  return c.json(response);
});

export const leadScraperMicroserviceapiv1jobsjobidRouter = router;
