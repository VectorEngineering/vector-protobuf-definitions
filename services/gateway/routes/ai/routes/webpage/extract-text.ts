import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
import { extractText } from "../../lib/html";
import { validateRequest } from "../../../../middleware/core";
import { z } from "zod";

/**
 * Schema for HTML content request
 */
const htmlRequestSchema = z.object({
    html: z.string().min(1),
});

/**
 * Plain text extraction route
 */
const extractTextRoute = new Hono<{ Bindings: Env }>();

/**
 * Extract plain text from HTML
 * @openapi
 * /ai/extract-text:
 *   post:
 *     summary: Extract plain text from HTML
 *     description: Strips all HTML tags and returns plain text content
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
 *                 description: HTML content to process
 *     responses:
 *       200:
 *         description: Successful text extraction
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 text:
 *                   type: string
 *                   description: Extracted plain text content
 */
extractTextRoute.post("/", validateRequest(htmlRequestSchema), async (c) => {
    try {
        const { html: rawHtml } = await c.req.json();
        const text = extractText(rawHtml);
        return c.json({ text });
    } catch (error) {
        if (error instanceof Error) {
            throw new HTTPException(500, { message: error.message });
        }
        throw new HTTPException(500, { message: "Unknown error occurred" });
    }
});

export default extractTextRoute; 