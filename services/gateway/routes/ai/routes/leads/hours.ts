import { OpenAPIHono as Hono, createRoute } from "@hono/zod-openapi";

import { BatchBusinessDataSchema } from "../webpage/schemas";
import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { businessHoursHandler } from "../../lib/handlers";
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
 * Business hours analysis routes
 */
const hours = new Hono<{ Bindings: Env }>();

/**
 * Analyze business hours and contact timing
 * @openapi
 * /ai/leads/hours:
 *   post:
 *     summary: Analyze business hours
 *     description: Analyzes business hours and traffic patterns to determine optimal contact times
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
 *               $ref: '#/components/schemas/BusinessHoursAnalysisSchema'
 */
hours.post("/", validateRequest(businessDataSchema), async (c) => {
  try {
    const result = await businessHoursHandler(c, await c.req.json());
    return c.json(result);
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

/**
 * Batch analyze business hours
 * @openapi
 * /ai/leads/hours/batch:
 *   post:
 *     summary: Batch analyze business hours
 *     description: Analyzes multiple business hours and traffic patterns in batch
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
hours.post("/batch", validateRequest(BatchBusinessDataSchema), async (c) => {
  try {
    return c.json(
      await handleBatchRequest(c, (ctx, data) =>
        businessHoursHandler(ctx, { prompt: JSON.stringify(data) }),
      ),
    );
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default hours;
