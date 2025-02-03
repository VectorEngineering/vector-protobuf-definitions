import { Hono } from "hono";
import { cors } from "hono/cors";
import { SampleAPIRouter } from "./route";
import type { Env } from "./types";

const app = new Hono<{
  Bindings: Env;
}>();

// CORS middleware
app.use("*", cors({
  origin: "*",
  allowMethods: ["GET","POST","OPTIONS"],
  allowHeaders: ["Content-Type", "Authorization"],
  exposeHeaders: ["Content-Length", "X-Request-Id"],
  maxAge: 86400,
}));

// API routes
app.route("/api", SampleAPIRouter);

export default app; 