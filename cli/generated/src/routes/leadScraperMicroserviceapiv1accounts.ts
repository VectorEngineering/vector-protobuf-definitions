import { Hono } from "hono";
import { ApiClient } from '../client';
import type { Env } from '../types';
import { HTTPException } from 'hono/http-exception';

// Route handler for /lead-scraper-microservice/api/v1/accounts
const router = new Hono<{
  Bindings: Env;
}>();


router.post("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response = await client.createLeadScraperMicroserviceApiV1Accounts(data);
  return c.json(response, 201);
});

router.put("/", async (c) => {
  const client = new ApiClient(c.env.API_BASE_URL);
  const data = await c.req.json();
  const response = await client.updateLeadScraperMicroserviceApiV1Accounts(data);
  return c.json(response);
});


export const leadScraperMicroserviceapiv1accountsRouter = router; 