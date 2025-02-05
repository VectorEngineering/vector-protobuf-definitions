import {
  analyzeStructure,
  cleanHtml,
  extractMetadata,
  extractText,
  sanitizeHtml,
} from "../lib/html";
import {
  convertHandler,
  enrichHandler,
  structureHandler,
  summaryHandler,
} from "../lib/handlers";

import { Env } from "../../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
import { validateRequest } from "../../../middleware/core";
import { z } from "zod";

/**
 * Schema for HTML content request
 */
const htmlRequestSchema = z.object({
  html: z.string().min(1),
  stream: z.boolean().optional().default(false),
});

/**
 * HTML processing routes
 */
const html = new Hono<{ Bindings: Env }>();

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
html.post("/convert", validateRequest(htmlRequestSchema), async (c) => {
  try {
    return await convertHandler(c, await c.req.json());
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

/**
 * Enrich HTML content with additional context
 * @openapi
 * /ai/enrich:
 *   post:
 *     summary: Enrich HTML content
 *     description: Analyzes HTML content and returns enriched details with additional context
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
 *                 description: HTML content to enrich
 *               stream:
 *                 type: boolean
 *                 description: Whether to stream the response
 *                 default: false
 *     responses:
 *       200:
 *         description: Successful enrichment
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 completion:
 *                   type: string
 *                   description: Enriched markdown content
 */
html.post("/enrich", validateRequest(htmlRequestSchema), async (c) => {
  try {
    return await enrichHandler(c, await c.req.json());
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

/**
 * Generate summary of HTML content
 * @openapi
 * /ai/summary:
 *   post:
 *     summary: Generate content summary
 *     description: Generates a concise summary of the HTML content
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
 *                 description: HTML content to summarize
 *               stream:
 *                 type: boolean
 *                 description: Whether to stream the response
 *                 default: false
 *     responses:
 *       200:
 *         description: Successful summary generation
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 completion:
 *                   type: string
 *                   description: Generated summary in markdown format
 */
html.post("/summary", validateRequest(htmlRequestSchema), async (c) => {
  try {
    return await summaryHandler(c, await c.req.json());
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

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
html.post("/sanitize", validateRequest(htmlRequestSchema), async (c) => {
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
html.post("/extract-text", validateRequest(htmlRequestSchema), async (c) => {
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
html.post("/metadata", validateRequest(htmlRequestSchema), async (c) => {
  try {
    const { html: rawHtml } = await c.req.json();
    const metadata = extractMetadata(rawHtml);
    return c.json({ metadata });
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

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
html.post("/structure", validateRequest(htmlRequestSchema), async (c) => {
  try {
    return await structureHandler(c, await c.req.json());
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default html;
