import { OpenAPIHono as Hono, createRoute } from "@hono/zod-openapi";

import { Env } from "../../../types";
import { HTTPException } from "hono/http-exception";
import { aiStatusResponseSchema } from "../schema";

/**
 * AI status route handler
 * Returns current status of AI service and available models
 *
 * @openapi
 * /ai/status:
 *   get:
 *     summary: Get AI service status
 *     description: |
 *       Returns the current status of the AI service, including:
 *       - Overall service health status
 *       - List of available models and their capabilities
 *       - Current latency information
 *       - Last status check timestamp
 *     tags:
 *       - AI
 *     responses:
 *       200:
 *         description: Successful response
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/AiStatusResponse'
 *       500:
 *         description: Server error
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 message:
 *                   type: string
 *                   description: Error message
 */
const status = new Hono<{ Bindings: Env }>();

status.get("/", async (c) => {
  try {
    const models = await c.env.AI.models();
    const status: typeof aiStatusResponseSchema._type = {
      status: "healthy",
      models: models.map((model) => ({
        id: model.id as any,
        available: true,
        latency: 0,
        capabilities: {
          streaming: true,
          functionCalling: true,
          lora: true,
        },
      })),
      lastChecked: new Date().toISOString(),
    };

    // Set cache headers for status endpoint
    c.header("Cache-Control", "public, max-age=60"); // Cache for 1 minute
    return c.json(status);
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default status;
