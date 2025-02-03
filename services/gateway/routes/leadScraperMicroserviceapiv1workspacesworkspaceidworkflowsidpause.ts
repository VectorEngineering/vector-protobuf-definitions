import { ApiClient } from '../client';
import type { Env } from '../types';
import { HTTPException } from 'hono/http-exception';
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/pause
const router = new Hono<{ Bindings: Env }>();


router.post("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response = await client.createLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflowsIdPause(data);
  return c.json(response, 201);
});



export const leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidpauseRouter = router; 