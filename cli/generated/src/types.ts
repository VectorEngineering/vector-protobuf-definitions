export interface Env {
  // Environment variables
  API_BASE_URL: string;
  API_KEY: string;
  // KV namespace for rate limiting
  KV?: KVNamespace;
  // Add any other environment variables your worker needs
}

// Export generated types
export * from './client'; 