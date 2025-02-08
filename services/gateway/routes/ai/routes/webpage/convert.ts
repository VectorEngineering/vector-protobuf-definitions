import { OpenAPIHono as Hono, createRoute } from "@hono/zod-openapi";

import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { convertHandler } from "../../lib/handlers";
import { validateRequest } from "../../../../middleware/core";
import { z } from "@hono/zod-openapi";

/**
 * Schema for HTML content request
 */
const htmlRequestSchema = z.object({
  html: z.string().min(1),
  stream: z.boolean().optional().default(false),
});

/**
 * HTML to Markdown conversion route
 */
const convert = new Hono<{ Bindings: Env }>();

/**
 * Convert HTML to markdown
 * @openapi
 * /ai/convert:
 *   post:
 *     summary: Convert HTML to markdown
 *     description: Converts HTML content to clean, well-formatted markdown
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
 *                 description: HTML content to convert
 *               stream:
 *                 type: boolean
 *                 description: Whether to stream the response
 *                 default: false
 *     responses:
 *       200:
 *         description: Successful conversion
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 completion:
 *                   type: string
 *                   description: Converted markdown content
 */
convert.post("/", validateRequest(htmlRequestSchema), async (c) => {
  try {
    return await convertHandler(c, await c.req.json());
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default convert;
