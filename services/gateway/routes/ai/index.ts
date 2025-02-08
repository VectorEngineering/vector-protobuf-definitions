import { OpenAPIHono as Hono, createRoute } from "@hono/zod-openapi";

import { Env } from "../../types";
import { HTTPException } from "hono/http-exception";
import analyzeCompany from "./routes/webpage/analyze-company";
import analyzeLead from "./routes/webpage/analyze-lead";
import completion from "./routes/completion";
import convert from "./routes/webpage/convert";
import embeddings from "./routes/webpage/embeddings";
import enrich from "./routes/webpage/enrich";
import extractContacts from "./routes/webpage/extract-contacts";
import extractText from "./routes/webpage/extract-text";
import leadgen from "./routes/webpage/leadgen";
import leadsRouter from "./routes/leads";
import markdownConvert from "./routes/webpage/markdown-convert";
import metadata from "./routes/webpage/metadata";
import sanitize from "./routes/webpage/sanitize";
import status from "./routes/status";
import structure from "./routes/webpage/structure";
import summary from "./routes/webpage/summary";

/**
 * AI routes handler
 * @openapi
 * tags:
 *   name: AI
 *   description: AI-powered text processing and analysis endpoints
 */
const aiRouter = new Hono<{ Bindings: Env }>();

/**
 * @openapi
 * /ai:
 *   post:
 *     summary: Generate AI completion
 *     description: Generate text using AI models with optional streaming
 *     tags: [AI]
 *     requestBody:
 *       required: true
 *       content:
 *         application/json:
 *           schema:
 *             $ref: '#/components/schemas/AiCompletionRequest'
 *     responses:
 *       200:
 *         description: Successful completion
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 completion:
 *                   type: string
 *           text/event-stream:
 *             schema:
 *               type: string
 */
aiRouter.route("/", completion);

/**
 * @openapi
 * /ai/status:
 *   get:
 *     summary: Get AI service status
 *     description: Returns current status of AI service and available models
 *     tags: [AI]
 *     responses:
 *       200:
 *         description: Service status
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/AiStatusResponse'
 */
aiRouter.route("/status", status);

// Mount HTML processing routes
aiRouter.route("/convert", convert);
aiRouter.route("/embeddings", embeddings);
aiRouter.route("/enrich", enrich);
aiRouter.route("/markdown-convert", markdownConvert);
aiRouter.route("/summary", summary);
aiRouter.route("/sanitize", sanitize);
aiRouter.route("/extract-text", extractText);
aiRouter.route("/metadata", metadata);
aiRouter.route("/structure", structure);

// Mount lead generation routes
aiRouter.route("/extract-contacts", extractContacts);
aiRouter.route("/analyze-company", analyzeCompany);
aiRouter.route("/analyze-lead", analyzeLead);
aiRouter.route("/leadgen", leadgen);

// Mount leads analysis routes
aiRouter.route("/leads", leadsRouter);

/**
 * Error handler for unsupported methods
 */
aiRouter.all("*", (c) => {
  throw new HTTPException(405, { message: "Method not allowed" });
});

export default aiRouter;
