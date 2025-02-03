import { ApiClient } from '../client';
import type { Env } from '../types';
import { HTTPException } from 'hono/http-exception';
import { Hono } from "hono";

// Route handler for /workspace-service/v1/workspace-sharings/{sharingId}
const router = new Hono<{ Bindings: Env }>();




router.delete("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    sharingId: c.req.param("sharingId") || (() => { throw new HTTPException(400, { message: "Missing required path parameter: sharingId" }) })(),
  };
  const response = await client.deleteWorkspaceServiceV1WorkspaceSharingsSharingId(params);
  return c.json(response);
});

export const workspaceServicev1workspaceSharingssharingidRouter = router; 