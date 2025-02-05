import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
import { summaryHandler } from "../../lib/handlers";
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
 * Content summarization route
 */
const summary = new Hono<{ Bindings: Env }>();

/**
 * Generate summary of HTML content
 * @openapi
 * /ai/summary:
 *   post:
 *     summary: Generate content summary
 *     description: Generates a concise summary of the HTML content
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
 *                 description: HTML content to summarize
 *               stream:
 *                 type: boolean
 *                 description: Whether to stream the response
 *                 default: false
 *     responses:
 *       200:
 *         description: Successful summary generation
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 completion:
 *                   type: string
 *                   description: Generated summary in markdown format
 */
summary.post("/", validateRequest(htmlRequestSchema), async (c) => {
  try {
    return await summaryHandler(c, await c.req.json());
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default summary;
