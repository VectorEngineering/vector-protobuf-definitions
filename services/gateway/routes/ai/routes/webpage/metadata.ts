import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
import { extractMetadata } from "../../lib/html";
import { validateRequest } from "../../../../middleware/core";
import { z } from "zod";

/**
 * Schema for HTML content request
 */
const htmlRequestSchema = z.object({
    html: z.string().min(1),
});

/**
 * Metadata extraction route
 */
const metadata = new Hono<{ Bindings: Env }>();

/**
 * Extract metadata from HTML
 * @openapi
 * /ai/metadata:
 *   post:
 *     summary: Extract metadata from HTML
 *     description: Extracts metadata including title, description, and Open Graph data
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
 *         description: Successful metadata extraction
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 metadata:
 *                   type: object
 *                   description: Extracted metadata
 */
metadata.post("/", validateRequest(htmlRequestSchema), async (c) => {
    try {
        const { html: rawHtml } = await c.req.json();
        const metadataResult = extractMetadata(rawHtml);
        return c.json({ metadata: metadataResult });
    } catch (error) {
        if (error instanceof Error) {
            throw new HTTPException(500, { message: error.message });
        }
        throw new HTTPException(500, { message: "Unknown error occurred" });
    }
});

export default metadata; 