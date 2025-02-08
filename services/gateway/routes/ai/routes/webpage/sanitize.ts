import { OpenAPIHono as Hono, createRoute } from "@hono/zod-openapi";

import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { sanitizeHtml } from "../../lib/html";
import { validateRequest } from "../../../../middleware/core";
import { z } from "@hono/zod-openapi";

/**
 * Schema for HTML content request
 */
const htmlRequestSchema = z.object({
  html: z.string().min(1),
});

/**
 * HTML sanitization route
 */
const sanitize = new Hono<{ Bindings: Env }>();

/**
 * Sanitize HTML content
 * @openapi
 * /ai/sanitize:
 *   post:
 *     summary: Sanitize HTML content
 *     description: Removes potentially dangerous elements and attributes from HTML
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
 *                 description: HTML content to sanitize
 *     responses:
 *       200:
 *         description: Successful sanitization
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 html:
 *                   type: string
 *                   description: Sanitized HTML content
 */
sanitize.post("/", validateRequest(htmlRequestSchema), async (c) => {
  try {
    const { html: rawHtml } = await c.req.json();
    const sanitized = sanitizeHtml(rawHtml);
    return c.json({ html: sanitized });
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default sanitize;
