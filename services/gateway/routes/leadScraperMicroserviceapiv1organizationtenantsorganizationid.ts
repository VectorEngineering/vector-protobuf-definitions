import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/organization/tenants/{organizationId}
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    organizationId:
      c.req.param("organizationId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: organizationId",
        });
      })(),
    pageSize: c.req.query("pageSize")
      ? Number(c.req.query("pageSize"))
      : undefined,
    pageNumber: c.req.query("pageNumber")
      ? Number(c.req.query("pageNumber"))
      : undefined,
  };
  const response =
    await client.getLeadScraperMicroserviceApiV1OrganizationTenantsOrganizationId(
      params,
    );
  return c.json(response);
});

export const leadScraperMicroserviceapiv1organizationtenantsorganizationidRouter =
  router;
