import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /workspace-service/v1/workspace-sharings
const router = new Hono<{ Bindings: Env }>();

router.put("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response = await client.updateWorkspaceServiceV1WorkspaceSharings(data);
  return c.json(response);
});

export const workspaceServicev1workspaceSharingsRouter = router;
