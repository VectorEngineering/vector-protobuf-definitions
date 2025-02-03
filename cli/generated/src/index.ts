import { Hono } from "hono";
import { cors } from "hono/cors";
import { LeadScrapingServiceAPIRouter } from "./routes";
import type { Env } from "./types";
import type { MiddlewareHandler } from "hono";

// Import user-defined middleware
import * as middleware from "./middleware";

const app = new Hono<{
  Bindings: Env;
}>();

// CORS middleware
app.use("*", cors({
  origin: "*",
  allowMethods: ["POST","PUT","GET","DELETE","GET","POST","GET","DELETE","GET","OPTIONS"],
  allowHeaders: ["Content-Type", "Authorization"],
  exposeHeaders: ["Content-Length", "X-Request-Id"],
  maxAge: 86400,
}));

// Apply user-defined middleware
Object.values(middleware).forEach(mw => {
  if (typeof mw === 'function') {
    app.use("*", mw as MiddlewareHandler<{ Bindings: Env }>);
  }
});

// API routes
app.route("/api", LeadScrapingServiceAPIRouter);

export default app; 