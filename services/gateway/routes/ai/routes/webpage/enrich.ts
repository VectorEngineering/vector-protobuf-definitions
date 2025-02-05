import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
import { enrichHandler } from "../../lib/handlers";
import { validateRequest } from "../../../../middleware/core";
import { z } from "zod";

/**
 * Schema for HTML content request
 */
const htmlRequestSchema = z.object({
  html: z.string().min(1),
  stream: z.boolean().optional().default(false),
});

/**
 * Content enrichment route
 */
const enrich = new Hono<{ Bindings: Env }>();

/**
 * Enrich HTML content with additional context
 * @openapi
 * /ai/enrich:
 *   post:
 *     summary: Enrich content with context
 *     description: Analyzes HTML content and provides enriched context and insights
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
 *                 description: HTML content to enrich
 *               stream:
 *                 type: boolean
 *                 description: Whether to stream the response
 *                 default: false
 *     responses:
 *       200:
 *         description: Successful enrichment
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 completion:
 *                   type: string
 *                   description: Enriched content in markdown format
 */
enrich.post("/", validateRequest(htmlRequestSchema), async (c) => {
  try {
    return await enrichHandler(c, await c.req.json());
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default enrich;
