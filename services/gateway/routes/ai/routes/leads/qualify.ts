import { OpenAPIHono as Hono, createRoute } from "@hono/zod-openapi";

import { BatchBusinessDataSchema } from "../webpage/schemas";
import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { businessQualificationHandler } from "../../lib/handlers";
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
 * Business qualification routes
 */
const qualify = new Hono<{ Bindings: Env }>();

/**
 * Analyze business qualification
 * @openapi
 * /ai/leads/qualify:
 *   post:
 *     summary: Qualify business lead
 *     description: Analyzes business data to assess qualification and potential value
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
 *         description: Successful qualification
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/BusinessQualificationSchema'
 */
qualify.post("/", validateRequest(businessDataSchema), async (c) => {
  try {
    const result = await businessQualificationHandler(c, await c.req.json());
    return c.json(result);
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

/**
 * Batch analyze business qualification
 * @openapi
 * /ai/leads/qualify/batch:
 *   post:
 *     summary: Batch qualify business leads
 *     description: Analyzes multiple business leads for qualification in batch
 *     tags: [AI]
 *     requestBody:
 *       required: true
 *       content:
 *         application/json:
 *           schema:
 *             $ref: '#/components/schemas/BatchBusinessDataSchema'
 *     responses:
 *       200:
 *         description: Successful batch qualification
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/BatchResponseSchema'
 */
qualify.post("/batch", validateRequest(BatchBusinessDataSchema), async (c) => {
  try {
    return c.json(
      await handleBatchRequest(c, (ctx, data) =>
        businessQualificationHandler(ctx, { prompt: JSON.stringify(data) }),
      ),
    );
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default qualify;
