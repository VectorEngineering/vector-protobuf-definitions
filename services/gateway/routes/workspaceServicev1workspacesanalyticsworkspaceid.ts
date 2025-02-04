import { ApiClient } from "../client";
import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";

// Route handler for /workspace-service/v1/workspaces/analytics/{workspaceId}
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
    startTime: c.req.query("startTime"),
    endTime: c.req.query("endTime"),
  };
  const response =
    await client.getWorkspaceServiceV1WorkspacesAnalyticsWorkspaceId(params);
  return c.json(response);
});

export const workspaceServicev1workspacesanalyticsworkspaceidRouter = router;
