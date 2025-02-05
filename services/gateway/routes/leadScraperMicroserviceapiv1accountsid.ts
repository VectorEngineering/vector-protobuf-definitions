import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/accounts/{id}
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    id:
      c.req.param("id") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: id",
        });
      })(),
    organizationId: c.req.query("organizationId"),
    tenantId: c.req.query("tenantId"),
  };
  const response =
    await client.getLeadScraperMicroserviceApiV1AccountsId(params);
  return c.json(response);
});

router.delete("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    id:
      c.req.param("id") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: id",
        });
      })(),
    organizationId: c.req.query("organizationId"),
    tenantId: c.req.query("tenantId"),
  };
  const response =
    await client.deleteLeadScraperMicroserviceApiV1AccountsId(params);
  return c.json(response);
});

export const leadScraperMicroserviceapiv1accountsidRouter = router;
