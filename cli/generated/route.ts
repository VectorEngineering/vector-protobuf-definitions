import { ApiClient } from './client';
import type { Env } from './types';
import { Hono } from "hono";

const router = new Hono<{
  Bindings: Env;
}>();

router.get("/users", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const response = await client.getUsers();
  return c.json(response);
});

router.post("/users", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response = await client.createUsers(data);
  return c.json(response, 201);
});

export const SampleAPIRouter = router; 