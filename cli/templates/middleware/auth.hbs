import type { MiddlewareHandler } from "hono";
import { HTTPException } from "hono/http-exception";
import type { Env } from "../types";

export const auth: MiddlewareHandler<{ Bindings: Env }> = async (c, next) => {
  const apiKey = c.req.header("x-api-key");

  if (!apiKey) {
    throw new HTTPException(401, { message: "Missing API key" });
  }

  // You can add additional validation here
  // For example, check if the API key is valid against your database/KV store
  if (apiKey !== c.env.API_KEY) {
    throw new HTTPException(403, { message: "Invalid API key" });
  }

  await next();
}; 