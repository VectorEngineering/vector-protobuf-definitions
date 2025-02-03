import { ApiClient } from '../client';
import type { Env } from '../types';
import { HTTPException } from 'hono/http-exception';
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/workspaces/{workspaceId}/workflows/{id}/trigger
const router = new Hono<{ Bindings: Env }>();


router.post("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response = await client.createLeadScraperMicroserviceApiV1WorkspacesWorkspaceIdWorkflowsIdTrigger(data);
  return c.json(response, 201);
});



export const leadScraperMicroserviceapiv1workspacesworkspaceidworkflowsidtriggerRouter = router; 