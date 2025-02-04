import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /workspace-service/v1/workspaces/{workspaceId}/share
const router = new Hono<{ Bindings: Env }>();

router.post("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response =
    await client.createWorkspaceServiceV1WorkspacesWorkspaceIdShare(data);
  return c.json(response, 201);
});

export const workspaceServicev1workspacesworkspaceidshareRouter = router;
