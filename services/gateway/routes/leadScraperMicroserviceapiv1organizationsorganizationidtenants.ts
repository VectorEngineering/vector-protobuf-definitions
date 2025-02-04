import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/organizations/{organizationId}/tenants
const router = new Hono<{ Bindings: Env }>();

router.post("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const params = {
    organizationId:
      c.req.param("organizationId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: organizationId",
        });
      })(),
  };
  const response =
    await client.createLeadScraperMicroserviceApiV1OrganizationsOrganizationIdTenants(
      data,
      params,
    );
  return c.json(response, 201);
});

export const leadScraperMicroserviceapiv1organizationsorganizationidtenantsRouter =
  router;
