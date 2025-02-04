import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
import { structureHandler } from "../../lib/handlers";
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
 * HTML structure analysis route
 */
const structure = new Hono<{ Bindings: Env }>();

/**
 * Analyze HTML structure
 * @openapi
 * /ai/structure:
 *   post:
 *     summary: Analyze HTML structure
 *     description: Provides a detailed analysis of the HTML document structure
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
 *               stream:
 *                 type: boolean
 *                 description: Whether to stream the response
 *                 default: false
 *     responses:
 *       200:
 *         description: Successful structure analysis
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 completion:
 *                   type: string
 *                   description: Structure analysis in markdown format
 */
structure.post("/", validateRequest(htmlRequestSchema), async (c) => {
    try {
        return await structureHandler(c, await c.req.json());
    } catch (error) {
        if (error instanceof Error) {
            throw new HTTPException(500, { message: error.message });
        }
        throw new HTTPException(500, { message: "Unknown error occurred" });
    }
});

export default structure; 