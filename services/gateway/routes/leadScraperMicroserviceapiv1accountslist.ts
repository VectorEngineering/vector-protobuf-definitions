import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/accounts/list
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    pageSize: c.req.query("pageSize")
      ? Number(c.req.query("pageSize"))
      : undefined,
    pageNumber: c.req.query("pageNumber")
      ? Number(c.req.query("pageNumber"))
      : undefined,
    filter: c.req.query("filter"),
    organizationId: c.req.query("organizationId"),
    tenantId: c.req.query("tenantId"),
  };
  const response =
    await client.getLeadScraperMicroserviceApiV1AccountsList(params);
  return c.json(response);
});

export const leadScraperMicroserviceapiv1accountslistRouter = router;
