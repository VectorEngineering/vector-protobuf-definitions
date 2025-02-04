import type { Context, Env, MiddlewareHandler, Next } from "hono";

import { KVNamespace } from "@cloudflare/workers-types";
import { bearerAuth } from "hono/bearer-auth";
import { cache } from "hono/cache";
import { cors } from "hono/cors";
import { env } from "hono/adapter";
import { logger as honoLogger } from "hono/logger";
import { logger } from "../utils/logger";
import { secureHeaders } from "hono/secure-headers";
import { z } from "zod";

const PUBLIC_PATHS = ["/", "/openapi", "/health"];

const authMiddleware: MiddlewareHandler<Env> = async (
  c: Context,
  next: Next,
) => {
  if (PUBLIC_PATHS.includes(c.req.path)) {
    return next();
  }

  const { API_SECRET_KEY } = env(c);
  const bearer = bearerAuth({ token: API_SECRET_KEY });

  return bearer(c, next);
};

const rateLimitMiddleware: MiddlewareHandler<{
  Bindings: {
    AUTH_STORE: KVNamespace;
  };
}> = async (c, next) => {
  const ip = c.req.header("CF-Connecting-IP");
  if (!ip) return next();

  const key = `ratelimit:${ip}`;
  const count = await c.env.AUTH_STORE.get(key);
  const currentCount = count ? parseInt(count) : 0;

  if (currentCount > 100) {
    return c.json({ error: "Rate limit exceeded" }, 429);
  }

  await c.env.AUTH_STORE.put(key, (currentCount + 1).toString(), {
    expirationTtl: 60,
  });
  return next();
};

const corsMiddleware = cors({
  origin: "*",
  allowMethods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
  allowHeaders: ["Content-Type", "Authorization", "X-API-Key"],
  maxAge: 86400,
});

const validateRequest = (schema: z.ZodSchema): MiddlewareHandler => {
  return async (c, next) => {
    try {
      const body = await c.req.json();
      const validated = await schema.parseAsync(body);
      c.set("validated", validated);
      return next();
    } catch (error) {
      if (error instanceof z.ZodError) {
        return c.json(
          {
            error: "Validation Error",
            details: error.errors,
          },
          400,
        );
      }
      throw error;
    }
  };
};

const cacheMiddleware: MiddlewareHandler<Env> = async (
  c: Context,
  next: Next,
) => {
  if (process.env.NODE_ENV === "development") {
    return next();
  }

  return cache({
    cacheName: "LeadScrapingServiceAPI",
    cacheControl: "max-age=3600",
  })(c, next);
};

const securityMiddleware = secureHeaders();

const loggingMiddleware: MiddlewareHandler<Env> = async (
  c: Context,
  next: Next,
) => {
  c.set("logger", logger);
  return honoLogger((message, rest) => {
    logger.info(c, message, {}, rest);
  })(c, next);
};

const versionMiddleware: MiddlewareHandler = async (c, next) => {
  const version = c.req.header("API-Version") || "v1";
  c.set("version", version);
  return next();
};

const tracingMiddleware: MiddlewareHandler = async (c, next) => {
  const requestId = c.req.header("X-Request-ID") || crypto.randomUUID();
  const traceId = c.req.header("X-Trace-ID") || crypto.randomUUID();

  c.set("requestId", requestId);
  c.set("traceId", traceId);
  c.header("X-Request-ID", requestId);
  c.header("X-Trace-ID", traceId);

  return next();
};

const metricsMiddleware: MiddlewareHandler = async (c, next) => {
  const start = performance.now();

  try {
    await next();
  } finally {
    const duration = performance.now() - start;
    const path = new URL(c.req.url).pathname;

    // You can send these metrics to your analytics service
    c.executionCtx.waitUntil(
      Promise.all([
        c.env.KV?.put(`metrics:latency:${path}`, duration.toString()),
        c.env.KV?.put(`metrics:hits:${path}`, "1", { expirationTtl: 86400 }),
      ]),
    );
  }
};

const errorHandlerMiddleware: MiddlewareHandler = async (c, next) => {
  try {
    await next();
  } catch (error: unknown) {
    logger.error(c, "Unhandled error", { error });

    if (error instanceof z.ZodError) {
      return c.json(
        {
          error: "Validation Error",
          details: error.errors,
        },
        400,
      );
    }

    return c.json(
      {
        error: "Internal Server Error",
        requestId: c.get("requestId"),
      },
      500,
    );
  }
};

const quotaMiddleware: MiddlewareHandler = async (c, next) => {
  const apiKey = c.req.header("X-API-Key");
  if (!apiKey) return next();

  const usage = await c.env.KV?.get(`quota:${apiKey}`);
  const limit = 1000; // Configure based on plan

  if (usage && parseInt(usage) > limit) {
    return c.json({ error: "Quota exceeded" }, 429);
  }

  await next();
  await c.env.KV?.put(`quota:${apiKey}`, "1", {
    expirationTtl: 86400,
    metadata: { timestamp: new Date().toISOString() },
  });
};

export {
  authMiddleware,
  cacheMiddleware,
  corsMiddleware,
  loggingMiddleware,
  rateLimitMiddleware,
  securityMiddleware,
  validateRequest,
  versionMiddleware,
  tracingMiddleware,
  metricsMiddleware,
  errorHandlerMiddleware,
  quotaMiddleware,
};
