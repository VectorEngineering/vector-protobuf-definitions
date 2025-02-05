import { extractMetadata, extractText } from "../../lib/html";

import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
import { createCompletionHandler } from "../../lib/completion";
import { validateRequest } from "../../../../middleware/core";
import { z } from "zod";

/**
 * Schema for lead analysis
 */
export const LeadAnalysisSchema = z.object({
  score: z.object({
    total: z.number().min(0).max(100),
    breakdown: z.object({
      companyFit: z.number().min(0).max(100),
      intent: z.number().min(0).max(100),
      engagement: z.number().min(0).max(100),
      budget: z.number().min(0).max(100),
    }),
  }),
  intent: z.object({
    level: z.enum(["HIGH", "MEDIUM", "LOW"]),
    signals: z.array(
      z.object({
        type: z.string(),
        description: z.string(),
        confidence: z.number().min(0).max(1),
      }),
    ),
    keywords: z.array(z.string()),
  }),
  painPoints: z.array(
    z.object({
      category: z.string(),
      description: z.string(),
      severity: z.enum(["HIGH", "MEDIUM", "LOW"]),
    }),
  ),
  readiness: z.object({
    stage: z.enum(["AWARENESS", "CONSIDERATION", "DECISION"]),
    indicators: z.array(z.string()),
    nextSteps: z.array(z.string()),
  }),
  engagement: z.object({
    level: z.enum(["HIGH", "MEDIUM", "LOW"]),
    metrics: z.object({
      contentDepth: z.number().min(0).max(100),
      interactionSignals: z.number().min(0).max(100),
      timeSpent: z.number().optional(),
    }),
  }),
});

/**
 * Schema for HTML content request
 */
const htmlRequestSchema = z.object({
  html: z.string().min(1),
});

/**
 * Lead analysis handler
 */
const leadHandler = createCompletionHandler({
  systemPrompt: `You are an expert at analyzing leads and determining purchase intent from web content. Analyze the content and score the lead according to these rules:
1. Calculate overall lead score based on multiple factors
2. Identify intent signals and keywords
3. Detect pain points and challenges
4. Determine buying stage and readiness
5. Analyze engagement levels and interaction signals
6. Provide specific evidence for all assessments
7. Return only factual information, no speculation`,
  defaultModel: "@cf/mistral/7b-instruct-v0.2",
  inputTransformer: (html) => {
    const text = extractText(html);
    const metadata = extractMetadata(html);
    return `Analyze this content and metadata to score and evaluate the lead. Format as JSON:\n\nMetadata:\n${JSON.stringify(metadata)}\n\nContent:\n${text}`;
  },
  outputTransformer: (output) => {
    try {
      const parsed = JSON.parse(output);
      const validated = LeadAnalysisSchema.parse(parsed);
      return JSON.stringify(validated);
    } catch (error) {
      throw new Error("Invalid lead analysis format");
    }
  },
  defaultParams: {
    temperature: 0.3,
    maxTokens: 2000,
  },
});

/**
 * Lead analysis route
 */
const analyzeLead = new Hono<{ Bindings: Env }>();

/**
 * Analyze and score lead
 * @openapi
 * /ai/analyze-lead:
 *   post:
 *     summary: Analyze and score lead
 *     description: Analyzes content to determine lead quality, intent, and engagement
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
 *     responses:
 *       200:
 *         description: Successful lead analysis
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/LeadAnalysisSchema'
 */
analyzeLead.post("/", validateRequest(htmlRequestSchema), async (c) => {
  try {
    const result = await leadHandler(c, await c.req.json());
    return c.json(result);
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default analyzeLead;
