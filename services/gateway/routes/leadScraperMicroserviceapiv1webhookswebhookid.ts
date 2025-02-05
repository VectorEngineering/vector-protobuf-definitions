import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/webhooks/{webhookId}
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    webhookId:
      c.req.param("webhookId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: webhookId",
        });
      })(),
    organizationId: c.req.query("organizationId"),
    workspaceId: c.req.query("workspaceId"),
    tenantId: c.req.query("tenantId"),
    accountId: c.req.query("accountId"),
  };
  const response =
    await client.getLeadScraperMicroserviceApiV1WebhooksWebhookId(params);
  return c.json(response);
});

router.delete("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    webhookId:
      c.req.param("webhookId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: webhookId",
        });
      })(),
    organizationId: c.req.query("organizationId"),
    workspaceId: c.req.query("workspaceId"),
    tenantId: c.req.query("tenantId"),
    accountId: c.req.query("accountId"),
  };
  const response =
    await client.deleteLeadScraperMicroserviceApiV1WebhooksWebhookId(params);
  return c.json(response);
});

export const leadScraperMicroserviceapiv1webhookswebhookidRouter = router;
