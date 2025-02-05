import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/api-keys/{keyId}
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    keyId:
      c.req.param("keyId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: keyId",
        });
      })(),
    organizationId: c.req.query("organizationId"),
    tenantId: c.req.query("tenantId"),
    accountId: c.req.query("accountId"),
    workspaceId: c.req.query("workspaceId"),
  };
  const response =
    await client.getLeadScraperMicroserviceApiV1ApiKeysKeyId(params);
  return c.json(response);
});

router.delete("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    keyId:
      c.req.param("keyId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: keyId",
        });
      })(),
    organizationId: c.req.query("organizationId"),
    workspaceId: c.req.query("workspaceId"),
    tenantId: c.req.query("tenantId"),
    accountId: c.req.query("accountId"),
  };
  const response =
    await client.deleteLeadScraperMicroserviceApiV1ApiKeysKeyId(params);
  return c.json(response);
});

export const leadScraperMicroserviceapiv1apiKeyskeyidRouter = router;
