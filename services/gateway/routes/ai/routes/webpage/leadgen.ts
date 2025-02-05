import { LeadGenResponseSchema, htmlRequestSchema } from "./schemas";

import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
import { leadgenHandler } from "../../lib/handlers";
import { validateRequest } from "../../../../middleware/core";

/**
 * Lead generation route
 */
const leadgen = new Hono<{ Bindings: Env }>();

/**
 * Process webpage content for lead generation
 * @openapi
 * /ai/leadgen:
 *   post:
 *     summary: Process webpage for lead generation
 *     description: Analyzes webpage content to extract contacts, identify company details, score leads, and generate personalized messages
 *     tags: [AI]
 *     requestBody:
 *       required: true
 *       content:
 *         application/json:
 *           schema:
 *             type: object
 *             required: [html]
 *             properties:
 *               html:
 *                 type: string
 *                 description: Webpage content to analyze
 *     responses:
 *       200:
 *         description: Successful lead generation analysis
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/LeadGenResponseSchema'
 *       400:
 *         description: Invalid request parameters
 *       405:
 *         description: Method not allowed
 *       500:
 *         description: Processing error
 */
leadgen.post("/", validateRequest(htmlRequestSchema), async (c) => {
  try {
    const result = await leadgenHandler(c, await c.req.json());
    return c.json(result);
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default leadgen;
