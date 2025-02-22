import { Ai, Fetcher, KVNamespace, R2Bucket } from "@cloudflare/workers-types";

import { Env as HonoEnv } from "hono";

export interface Env extends HonoEnv {
  // Environment variables
  API_BASE_URL: string;
  API_KEY: string;
  // KV namespace for rate limiting
  KV?: KVNamespace;
  // add ai capabilities
  AI: Ai;
  // add r2 bucket
  R2: R2Bucket;
}

// Export generated types
export * from "./client";
