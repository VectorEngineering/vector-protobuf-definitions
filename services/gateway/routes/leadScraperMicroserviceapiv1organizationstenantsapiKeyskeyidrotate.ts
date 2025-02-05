import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/organizations/tenants/api-keys/{keyId}/rotate
const router = new Hono<{ Bindings: Env }>();

router.post("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const params = {
    keyId:
      c.req.param("keyId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: keyId",
        });
      })(),
  };
  const response =
    await client.createLeadScraperMicroserviceApiV1OrganizationsTenantsApiKeysKeyIdRotate(
      data,
      params,
    );
  return c.json(response, 201);
});

export const leadScraperMicroserviceapiv1organizationstenantsapiKeyskeyidrotateRouter =
  router;
