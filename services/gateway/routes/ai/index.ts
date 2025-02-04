import { Env } from "../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
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
 * Provides endpoints for AI completions, service status, and HTML processing
 */
const aiRouter = new Hono<{ Bindings: Env }>();

// Mount completion routes
aiRouter.route("/", completion);

// Mount status routes
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