import { ApiClient } from '../client';
import type { Env } from '../types';
import { HTTPException } from 'hono/http-exception';
import { Hono } from "hono";

// Route handler for /lead-scraper-microservice/api/v1/workspaces/workflow
const router = new Hono<{ Bindings: Env }>();



router.put("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response = await client.updateLeadScraperMicroserviceApiV1WorkspacesWorkflow(data);
  return c.json(response);
});


export const leadScraperMicroserviceapiv1workspacesworkflowRouter = router; 