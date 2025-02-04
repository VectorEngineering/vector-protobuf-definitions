import { EmbeddingResponseSchema, htmlRequestSchema } from "./schemas";

import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
import { embeddingHandler } from "../../lib/handlers";
import { validateRequest } from "../../../../middleware/core";

/**
 * Text embeddings route
 */
const embeddings = new Hono<{ Bindings: Env }>();

/**
 * Generate text embeddings from webpage content
 * @openapi
 * /ai/embeddings:
 *   post:
 *     summary: Generate text embeddings
 *     description: Extracts text from webpage content and generates vector embeddings using BAAI's large embedding model
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
 *                 description: HTML content to analyze
 *               options:
 *                 type: object
 *                 properties:
 *                   includeOriginalText:
 *                     type: boolean
 *                     description: Whether to include the extracted text in the response
 *                     default: false
 *     responses:
 *       200:
 *         description: Successfully generated embeddings
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/EmbeddingResponseSchema'
 *       400:
 *         description: Invalid request parameters
 *       405:
 *         description: Method not allowed
 *       500:
 *         description: Processing error
 */
embeddings.post("/", validateRequest(htmlRequestSchema), async (c) => {
    try {
        const result = await embeddingHandler(c, await c.req.json());
        return c.json(result);
    } catch (error) {
        if (error instanceof Error) {
            throw new HTTPException(500, { message: error.message });
        }
        throw new HTTPException(500, { message: "Unknown error occurred" });
    }
});

export default embeddings; 