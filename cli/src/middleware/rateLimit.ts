import type { Context, MiddlewareHandler, Next } from 'hono';
import { HTTPException } from 'hono/http-exception';
import type { Env } from '../types';

const DEFAULT_LIMIT = 100;
const WINDOW_SIZE = 60 * 60; // 1 hour in seconds

export const rateLimit: MiddlewareHandler<{ Bindings: Env }> = async (
  c: Context,
  next: Next
) => {
  // Simple in-memory rate limiting
  // Replace with your storage solution (Redis, DB, etc.)
  const endpoint = new URL(c.req.url).pathname;
  const key = `rate_limit:${endpoint}`;

  const currentCount = parseInt((await c.env.KV?.get(key)) || '0');

  if (currentCount >= DEFAULT_LIMIT) {
    throw new HTTPException(429, { message: 'Rate limit exceeded' });
  }

  await c.env.KV?.put(key, (currentCount + 1).toString(), {
    expirationTtl: WINDOW_SIZE,
  });

  c.header('X-RateLimit-Limit', DEFAULT_LIMIT.toString());
  c.header(
    'X-RateLimit-Remaining',
    (DEFAULT_LIMIT - currentCount - 1).toString()
  );

  await next();
};
