import { Env } from "../../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
import { aiCompletionRequestSchema } from "../schema";
import { createCompletionHandler } from "../lib/completion";
import { validateRequest } from "../../../middleware/core";

/**
 * AI completion route handler
 * Handles text generation requests with both streaming and non-streaming responses
 * 
 * @openapi
 * /ai:
 *   post:
 *     summary: Generate text using AI models
 *     description: |
 *       Generates text using Cloudflare's AI models. Supports both streaming and non-streaming responses.
 *       The endpoint can be configured with various parameters to control the generation process.
 *     tags:
 *       - AI
 *     requestBody:
 *       required: true
 *       content:
 *         application/json:
 *           schema:
 *             $ref: '#/components/schemas/AiCompletionRequest'
 *     responses:
 *       200:
 *         description: Successful response
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 completion:
 *                   type: string
 *                   description: The generated text
 *           text/event-stream:
 *             schema:
 *               type: string
 *               description: Stream of generated text chunks
 *       400:
 *         description: Invalid request parameters
 *       500:
 *         description: Server error
 */
const completion = new Hono<{ Bindings: Env }>();

// Create a default completion handler
const defaultHandler = createCompletionHandler();

completion.post("/", validateRequest(aiCompletionRequestSchema), async (c) => {
    try {
        return await defaultHandler(c, await c.req.json());
    } catch (error) {
        if (error instanceof Error) {
            throw new HTTPException(500, { message: error.message });
        }
        throw new HTTPException(500, { message: "Unknown error occurred" });
    }
});

export default completion; 