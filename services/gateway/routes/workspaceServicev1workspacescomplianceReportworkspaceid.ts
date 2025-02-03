import { ApiClient } from '../client';
import type { Env } from '../types';
import { HTTPException } from 'hono/http-exception';
import { Hono } from "hono";

// Route handler for /workspace-service/v1/workspaces/compliance-report/{workspaceId}
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    workspaceId: c.req.param("workspaceId") || (() => { throw new HTTPException(400, { message: "Missing required path parameter: workspaceId" }) })(),
    complianceType: c.req.query("complianceType"),
  };
  const response = await client.getWorkspaceServiceV1WorkspacesComplianceReportWorkspaceId(params);
  return c.json(response);
});




export const workspaceServicev1workspacescomplianceReportworkspaceidRouter = router; 