import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/workspaces
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    accountId: c.req.query("accountId"),
    pageSize: c.req.query("pageSize")
      ? Number(c.req.query("pageSize"))
      : undefined,
    pageNumber: c.req.query("pageNumber")
      ? Number(c.req.query("pageNumber"))
      : undefined,
    organizationId: c.req.query("organizationId"),
    tenantId: c.req.query("tenantId"),
  };
  const response =
    await client.getLeadScraperMicroserviceApiV1Workspaces(params);
  return c.json(response);
});

router.post("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response =
    await client.createLeadScraperMicroserviceApiV1Workspaces(data);
  return c.json(response, 201);
});

export const leadScraperMicroserviceapiv1workspacesRouter = router;
