import {
  D1Database,
  KVNamespace,
  R2Bucket,
  Fetcher,
  Ai,
} from "@cloudflare/workers-types";
import { TokenPrivilege } from "./middleware/auth";

interface Env {
  // Environment variables
  ENVIRONMENT: string;
  API_KEY: string;
  SECRET: string;

  // Service bindings
  USERS_DATABASE: D1Database;
  AUTH_STORE: KVNamespace;
  KV?: KVNamespace;
  // Add any other environment variables your worker needs
  ENRICH_KV: KVNamespace;
  STORAGE: R2Bucket;
  TELLER_CERT: Fetcher;
  AI: Ai;
}

type Variables = {
  privilege?: TokenPrivilege;
};

type Bindings = Env;
