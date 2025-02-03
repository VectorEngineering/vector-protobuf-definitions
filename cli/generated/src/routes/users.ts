import { Hono } from "hono";
import { ApiClient } from '../client';
import type { Env } from '../types';

// Route handler for /users
const router = new Hono<{
  Bindings: Env;
}>();

router.get("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const response = await client.getUsers();
  return c.json(response);
});

router.post("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response = await client.createUsers(data);
  return c.json(response, 201);
});



export const usersRouter = router; 