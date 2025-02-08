import { OpenAPIHono as Hono, createRoute } from "@hono/zod-openapi";

import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { createCompletionHandler } from "../../lib/completion";
import { htmlToMarkdown } from "../../lib/markdown";
import { validateRequest } from "../../../../middleware/core";
import { z } from "@hono/zod-openapi";

/**
 * Schema for markdown conversion request
 */
const markdownRequestSchema = z.object({
  html: z.string().min(1),
  options: z
    .object({
      cacheResults: z.boolean().optional().default(true),
      enhanceWithAI: z.boolean().optional().default(true),
    })
    .optional(),
});

/**
 * Schema for markdown conversion response
 */
export const MarkdownResponseSchema = z.object({
  markdown: z.string(),
  metadata: z.object({
    model: z.string().optional(),
    cached: z.boolean(),
    enhanced: z.boolean(),
  }),
});

/**
 * Advanced markdown conversion handler
 */
const markdownHandler = createCompletionHandler({
  systemPrompt: `You are an AI assistant that converts webpage content to markdown while filtering out unnecessary information. Please follow these guidelines:
- Remove any inappropriate content, ads, or irrelevant information
- If unsure about including something, err on the side of keeping it
- Answer in English. Include all points in markdown in sufficient detail to be useful
- Aim for clean, readable markdown
- Return the markdown and nothing else`,
  defaultModel: "@cf/qwen/1.5-14b-chat-awq",
  inputTransformer: (
    html,
    options?: { cacheResults?: boolean; enhanceWithAI?: boolean },
  ) => {
    const basicMarkdown = htmlToMarkdown(html);
    return `Convert this content to clean markdown:\n\n${basicMarkdown}`;
  },
  outputTransformer: (output) => {
    // Remove any markdown code block markers that might be in the response
    const cleanMarkdown = output
      .replace(/^\`\`\`markdown\n|\`\`\`$/g, "")
      .trim();
    return cleanMarkdown;
  },
  defaultParams: {
    temperature: 0.3,
    maxTokens: 2000,
  },
});

/**
 * Markdown conversion route
 */
const markdownConvert = new Hono<{ Bindings: Env & { KV: KVNamespace } }>();

/**
 * Convert webpage content to enhanced markdown
 * @openapi
 * /ai/markdown-convert:
 *   post:
 *     summary: Convert webpage to markdown
 *     description: Converts HTML content to clean, enhanced markdown using AI
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
 *               options:
 *                 type: object
 *                 properties:
 *                   cacheResults:
 *                     type: boolean
 *                     description: Whether to cache the conversion results
 *                     default: true
 *                   enhanceWithAI:
 *                     type: boolean
 *                     description: Whether to enhance the markdown with AI
 *                     default: true
 *     responses:
 *       200:
 *         description: Successfully converted to markdown
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/MarkdownResponseSchema'
 */
markdownConvert.post("/", validateRequest(markdownRequestSchema), async (c) => {
  try {
    const { html, options = {} } = await c.req.json();
    const cacheKey = `md:${Buffer.from(html).toString("base64")}`;

    // Check cache if enabled
    if (options.cacheResults) {
      const cached = await c.env.KV.get(cacheKey);
      if (cached) {
        return c.json({
          markdown: cached,
          metadata: {
            cached: true,
            enhanced: true,
          },
        });
      }
    }

    // Convert to basic markdown first
    let markdown = htmlToMarkdown(html);

    // Enhance with AI if enabled
    if (options.enhanceWithAI) {
      const response = await markdownHandler(c, { prompt: html });
      markdown = await response.text();
    }

    // Cache the result if enabled
    if (options.cacheResults) {
      await c.env.KV.put(cacheKey, markdown, { expirationTtl: 3600 });
    }

    return c.json({
      markdown,
      metadata: {
        model: options.enhanceWithAI ? "@cf/qwen/1.5-14b-chat-awq" : undefined,
        cached: false,
        enhanced: options.enhanceWithAI || false,
      },
    });
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Failed to convert to markdown" });
  }
});

export default markdownConvert;
