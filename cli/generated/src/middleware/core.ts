import type { Context, Next } from "hono";

import { bearerAuth } from "hono/bearer-auth";
import { cache } from "hono/cache";
import { env } from "hono/adapter";
import { logger as honoLogger } from "hono/logger";
import { logger } from "../utils/logger";
import { secureHeaders } from "hono/secure-headers";

const PUBLIC_PATHS = ["/", "/openapi", "/health"];

const authMiddleware = (c: Context, next: Next) => {
  if (PUBLIC_PATHS.includes(c.req.path)) {
    return next();
  }

  const { API_SECRET_KEY } = env(c);
  const bearer = bearerAuth({ token: API_SECRET_KEY });

  return bearer(c, next);
};

const cacheMiddleware = (c: Context, next: Next) => {
  if (process.env.NODE_ENV === "development") {
    return next();
  }

  return cache({
    cacheName: "LeadScrapingServiceAPI",
    cacheControl: "max-age=3600",
  })(c, next);
};

const securityMiddleware = secureHeaders();

const loggingMiddleware = (c: Context, next: Next) => {
  c.set("logger", logger);
  return honoLogger((message, rest) => {
    logger.info(c, message, {}, rest);
  })(c, next);
};

export {
  authMiddleware,
  cacheMiddleware,
  securityMiddleware,
  loggingMiddleware,
}; 