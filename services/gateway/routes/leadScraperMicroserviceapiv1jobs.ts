import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/jobs
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    authPlatformUserId: c.req.query("authPlatformUserId") || "",
    orgId: c.req.query("orgId") || "",
    tenantId: c.req.query("tenantId") || "",
  };
  const response = await client.getLeadScraperMicroserviceApiV1Jobs(params);
  return c.json(response);
});

router.post("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response = await client.createLeadScraperMicroserviceApiV1Jobs(data);
  return c.json(response, 201);
});

export const leadScraperMicroserviceapiv1jobsRouter = router;
