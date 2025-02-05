// Import user-defined middleware
import * as middleware from "./middleware";

import type { Env } from "./types";
import { LeadScrapingServiceAPIRouter } from "./routes";
import type { MiddlewareHandler } from "hono";
import { OpenAPIHono } from "@hono/zod-openapi";
import { compress } from "hono/compress";
import { cors } from "hono/cors";
import { csrf } from "hono/csrf";
import { prettyJSON } from "hono/pretty-json";
import { secureHeaders } from "hono/secure-headers";
import { swaggerUI } from "@hono/swagger-ui";
import { timing } from "hono/timing";

const app = new OpenAPIHono<Env>({
  defaultHook: (result, c) => {
    if (!result.success) {
      return c.json({ success: false, errors: result.error.errors }, 422);
    }
  },
});

// CORS middleware
app.use(
  "*",
  cors({
    origin: "*",
    allowMethods: [
      "POST",
      "PUT",
      "GET",
      "DELETE",
      "GET",
      "POST",
      "GET",
      "DELETE",
      "GET",
      "OPTIONS",
    ],
    allowHeaders: ["Content-Type", "Authorization"],
    exposeHeaders: ["Content-Length", "X-Request-Id"],
    maxAge: 86400,
  }),
);

// Timing middleware
app.use("*", timing());

// Secure headers middleware
app.use("*", secureHeaders());

// Pretty JSON middleware
app.use("*", prettyJSON());

// CSRF middleware
app.use("*", csrf());

// Compress middleware
app.use("*", compress());

// Apply user-defined middleware
Object.values(middleware).forEach((mw) => {
  if (typeof mw === "function") {
    app.use("*", mw as MiddlewareHandler<Env, "*">);
  }
});

// Swagger UI
app.get(
  "/",
  swaggerUI({
    url: "/openapi",
  }),
);

// OpenAPI documentation
app.doc("/openapi", {
  openapi: "3.1.0",
  info: {
    version: "1.0",
    title: "Lead Scraping Service API",
  },
});

// Register security scheme
app.openAPIRegistry.registerComponent("securitySchemes", "Bearer", {
  type: "http",
  scheme: "bearer",
});

// API routes
app.route("/api", LeadScrapingServiceAPIRouter);

// Error handling
app.onError((err, c) => {
  console.error(`${err}`);
  const error = err instanceof Error ? err.message : "Internal Server Error";
  return c.json({ error }, 500);
});

// 404 handler
app.notFound((c) => {
  return c.json({ error: "Not Found" }, 404);
});

export type AppType = typeof app;
export default app;
