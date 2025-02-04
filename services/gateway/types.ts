import { Ai, Fetcher, KVNamespace, R2Bucket } from "@cloudflare/workers-types";

import { AiService } from "./routes/ai/types";
import { Env as HonoEnv } from "hono";

export interface Env extends HonoEnv {
  // Environment variables
  API_BASE_URL: string;
  API_KEY: string;
  // KV namespace for rate limiting
  KV?: KVNamespace;
  AI: AiService;
}

// Export generated types
export * from "./client";
