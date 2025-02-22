import type { Context, MiddlewareHandler, Next } from "hono";

import type { Env } from "../types";
import { HTTPException } from "hono/http-exception";
import { bearerAuth } from "hono/bearer-auth";

// Define privilege levels
export enum TokenPrivilege {
  READ = "read",
  WRITE = "write",
  ADMIN = "admin",
}

// Define allowed methods for each privilege level
const privilegeMethods = {
  [TokenPrivilege.READ]: ["GET"],
  [TokenPrivilege.WRITE]: ["GET", "POST", "PUT", "PATCH"],
  [TokenPrivilege.ADMIN]: ["GET", "POST", "PUT", "PATCH", "DELETE"],
};

export const auth: MiddlewareHandler<Env> = async (c: Context, next: Next) => {
  const authHeader = c.req.header("Authorization");
  const apiKey = c.req.header("x-api-key");

  // First check for Bearer token authentication
  if (authHeader?.startsWith("Bearer ")) {
    const token = authHeader.substring(7);

    try {
      // Verify the bearer token
      const bearer = bearerAuth({
        token: token, // Pass the token to verify against
      });

      await bearer(c, next);

      // Verify if the token matches API_KEY
      if (token !== c.env.API_KEY) {
        throw new HTTPException(401, { message: "Invalid bearer token" });
      }

      // Check method permissions based on token privilege
      const privilege = TokenPrivilege.READ; // Default to READ privilege
      // You can extend this to extract privilege from token or lookup from database

      const method = c.req.method;
      const allowedMethods = privilegeMethods[privilege] || [];

      if (!allowedMethods.includes(method)) {
        throw new HTTPException(403, {
          message: `Insufficient privileges. This token has ${privilege} privileges which don't allow ${method} requests.`,
        });
      }

      return;
    } catch (e) {
      throw new HTTPException(401, { message: "Invalid bearer token" });
    }
  }

  // Fall back to API key authentication
  if (!apiKey) {
    throw new HTTPException(401, {
      message:
        "Missing authentication. Please provide either a Bearer token or API key",
    });
  }

  // Validate API key
  if (apiKey !== c.env.API_KEY) {
    throw new HTTPException(403, { message: "Invalid API key" });
  }

  // API keys have full privileges by default
  await next();
};
