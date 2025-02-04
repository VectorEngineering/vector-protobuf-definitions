import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}/{tenantId}
const router = new Hono<{ Bindings: Env }>();

router.delete("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    organizationId:
      c.req.param("organizationId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: organizationId",
        });
      })(),
    tenantId:
      c.req.param("tenantId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: tenantId",
        });
      })(),
  };
  const response =
    await client.deleteLeadScraperMicroserviceApiV1OrganizationTenantsOrganizationIdTenantId(
      params,
    );
  return c.json(response);
});

export const leadScraperMicroserviceapiv1organizationtenantsorganizationidtenantidRouter =
  router;
