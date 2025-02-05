import { extractMetadata, extractText } from "../../lib/html";

import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { Hono } from "hono";
import { createCompletionHandler } from "../../lib/completion";
import { validateRequest } from "../../../../middleware/core";
import { z } from "zod";

/**
 * Schema for company analysis
 */
export const CompanyAnalysisSchema = z.object({
  company: z.object({
    name: z.string(),
    description: z.string(),
    size: z.enum(["SMALL", "MEDIUM", "LARGE", "ENTERPRISE"]).optional(),
    founded: z.string().optional(),
    headquarters: z.string().optional(),
  }),
  industry: z.object({
    primary: z.string(),
    secondary: z.array(z.string()),
    segments: z.array(z.string()),
  }),
  products: z
    .array(
      z.object({
        name: z.string(),
        description: z.string(),
        category: z.string(),
      }),
    )
    .optional(),
  businessModel: z.enum(["B2B", "B2C", "B2B2C", "D2C"]).optional(),
  marketFocus: z.array(z.string()),
  competitors: z.array(z.string()).optional(),
  technologies: z.array(z.string()).optional(),
});

/**
 * Schema for HTML content request
 */
const htmlRequestSchema = z.object({
  html: z.string().min(1),
});

/**
 * Company analysis handler
 */
const companyHandler = createCompletionHandler({
  systemPrompt: `You are an expert at analyzing company and industry information from web content. Analyze the content and extract key business information according to these rules:
1. Identify company name and core details
2. Determine primary and secondary industries
3. Identify key business segments and market focus
4. Extract product/service information
5. Determine business model and company size
6. Identify technologies and competitors if mentioned
7. Return only factual information, no speculation`,
  defaultModel: "@cf/mistral/7b-instruct-v0.2",
  inputTransformer: (html) => {
    const text = extractText(html);
    const metadata = extractMetadata(html);
    return `Analyze this content and metadata to extract company and industry information. Format as JSON:\n\nMetadata:\n${JSON.stringify(metadata)}\n\nContent:\n${text}`;
  },
  outputTransformer: (output) => {
    try {
      const parsed = JSON.parse(output);
      const validated = CompanyAnalysisSchema.parse(parsed);
      return JSON.stringify(validated);
    } catch (error) {
      throw new Error("Invalid company analysis format");
    }
  },
  defaultParams: {
    temperature: 0.2,
    maxTokens: 1500,
  },
});

/**
 * Company analysis route
 */
const analyzeCompany = new Hono<{ Bindings: Env }>();

/**
 * Analyze company and industry information
 * @openapi
 * /ai/analyze-company:
 *   post:
 *     summary: Analyze company information
 *     description: Extracts and analyzes company details, industry focus, and business segments
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
 *         description: Successful company analysis
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/CompanyAnalysisSchema'
 */
analyzeCompany.post("/", validateRequest(htmlRequestSchema), async (c) => {
  try {
    const result = await companyHandler(c, await c.req.json());
    return c.json(result);
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default analyzeCompany;
