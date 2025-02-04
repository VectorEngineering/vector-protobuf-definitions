import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    workspaceId:
      c.req.param("workspaceId") ||
      (() => {
        throw new HTTPException(400, {
          message: "Missing required path parameter: workspaceId",
        });
      })(),
    pageSize: c.req.query("pageSize")
      ? Number(c.req.query("pageSize"))
      : undefined,
    pageNumber: c.req.query("pageNumber")
      ? Number(c.req.query("pageNumber"))
      : undefined,
    filter: c.req.query("filter"),
  };
  const response =
    await client.getLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflows(
      params,
    );
  return c.json(response);
});

router.post("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response =
    await client.createLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflows(
      data,
    );
  return c.json(response, 201);
});

export const leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsRouter =
  router;
