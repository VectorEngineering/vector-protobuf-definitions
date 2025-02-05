import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/leads
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    organizationId: c.req.query("organizationId"),
    workspaceId: c.req.query("workspaceId"),
    tenantId: c.req.query("tenantId"),
    accountId: c.req.query("accountId"),
    pageSize: c.req.query("pageSize")
      ? Number(c.req.query("pageSize"))
      : undefined,
    pageNumber: c.req.query("pageNumber")
      ? Number(c.req.query("pageNumber"))
      : undefined,
  };
  const response = await client.getLeadScraperMicroserviceApiV1Leads(params);
  return c.json(response);
});

export const leadScraperMicroserviceapiv1leadsRouter = router;
