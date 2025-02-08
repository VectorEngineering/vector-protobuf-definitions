import { OpenAPIHono as Hono, createRoute } from "@hono/zod-openapi";

import { BatchBusinessDataSchema } from "../webpage/schemas";
import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { businessIntelligenceHandler } from "../../lib/handlers";
import { handleBatchRequest } from "../../lib/batch";
import { validateRequest } from "../../../../middleware/core";
import { z } from "@hono/zod-openapi";

/**
 * Schema for business data request
 */
const businessDataSchema = z.object({
  data: z.any(),
});

/**
 * Business intelligence routes
 */
const intelligence = new Hono<{ Bindings: Env }>();

/**
 * Analyze business intelligence
 * @openapi
 * /ai/leads/intelligence:
 *   post:
 *     summary: Extract business intelligence
 *     description: Analyzes business data to extract intelligence and market insights
 *     tags: [AI]
 *     requestBody:
 *       required: true
 *       content:
 *         application/json:
 *           schema:
 *             type: object
 *             required: [data]
 *             properties:
 *               data:
 *                 type: object
 *                 description: Business data to analyze
 *     responses:
 *       200:
 *         description: Successful analysis
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/BusinessIntelligenceSchema'
 */
intelligence.post("/", validateRequest(businessDataSchema), async (c) => {
  try {
    const result = await businessIntelligenceHandler(c, await c.req.json());
    return c.json(result);
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

/**
 * Batch analyze business intelligence
 * @openapi
 * /ai/leads/intelligence/batch:
 *   post:
 *     summary: Batch extract business intelligence
 *     description: Analyzes multiple businesses for intelligence and insights in batch
 *     tags: [AI]
 *     requestBody:
 *       required: true
 *       content:
 *         application/json:
 *           schema:
 *             $ref: '#/components/schemas/BatchBusinessDataSchema'
 *     responses:
 *       200:
 *         description: Successful batch analysis
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/BatchResponseSchema'
 */
intelligence.post(
  "/batch",
  validateRequest(BatchBusinessDataSchema),
  async (c) => {
    try {
      return c.json(
        await handleBatchRequest(c, (ctx, data) =>
          businessIntelligenceHandler(ctx, { prompt: JSON.stringify(data) }),
        ),
      );
    } catch (error) {
      if (error instanceof Error) {
        throw new HTTPException(500, { message: error.message });
      }
      throw new HTTPException(500, { message: "Unknown error occurred" });
    }
  },
);

export default intelligence;
