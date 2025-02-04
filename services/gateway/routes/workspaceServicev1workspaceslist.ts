import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /workspace-service/v1/workspaces/list
const router = new Hono<{ Bindings: Env }>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const params = {
    accountId: c.req.query("accountId"),
    pageSize: c.req.query("pageSize")
      ? Number(c.req.query("pageSize"))
      : undefined,
    pageToken: c.req.query("pageToken"),
    filter: c.req.query("filter"),
  };
  const response = await client.getWorkspaceServiceV1WorkspacesList(params);
  return c.json(response);
});

export const workspaceServicev1workspaceslistRouter = router;
