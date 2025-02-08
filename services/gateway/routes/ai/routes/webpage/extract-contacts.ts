import { OpenAPIHono as Hono, createRoute } from "@hono/zod-openapi";

import { Env } from "../../../../types";
import { HTTPException } from "hono/http-exception";
import { createCompletionHandler } from "../../lib/completion";
import { extractText } from "../../lib/html";
import { validateRequest } from "../../../../middleware/core";
import { z } from "@hono/zod-openapi";

/**
 * Schema for contact information
 */
export const ContactSchema = z.object({
  emails: z.array(z.string().email()),
  phoneNumbers: z.array(z.string()),
  socialMedia: z.object({
    linkedin: z.array(z.string().url()).optional(),
    twitter: z.array(z.string().url()).optional(),
    facebook: z.array(z.string().url()).optional(),
    instagram: z.array(z.string().url()).optional(),
  }),
  addresses: z.array(z.string()),
  websites: z.array(z.string().url()),
});

/**
 * Schema for HTML content request
 */
const htmlRequestSchema = z.object({
  html: z.string().min(1),
});

/**
 * Contact extraction handler
 */
const contactHandler = createCompletionHandler({
  systemPrompt: `You are an expert at extracting contact information from web content. Extract all contact details and format them according to these rules:
1. Identify all email addresses
2. Extract phone numbers in standardized format
3. Find social media profile URLs
4. Locate physical addresses
5. Extract website URLs
6. Ensure all extracted information is valid and properly formatted
7. Return only factual information, no guessing or inference`,
  defaultModel: "@cf/mistral/7b-instruct-v0.2",
  inputTransformer: (html) => {
    const text = extractText(html);
    return `Extract all contact information from this content and format as JSON:\n\n${text}`;
  },
  outputTransformer: (output) => {
    try {
      const parsed = JSON.parse(output);
      const validated = ContactSchema.parse(parsed);
      return JSON.stringify(validated);
    } catch (error) {
      throw new Error("Invalid contact information format");
    }
  },
  defaultParams: {
    temperature: 0.1,
    maxTokens: 1000,
  },
});

/**
 * Contact information extraction route
 */
const extractContacts = new Hono<{ Bindings: Env }>();

/**
 * Extract contact information from HTML
 * @openapi
 * /ai/extract-contacts:
 *   post:
 *     summary: Extract contact information
 *     description: Extracts contact details including emails, phone numbers, and social media profiles
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
 *         description: Successful contact extraction
 *         content:
 *           application/json:
 *             schema:
 *               $ref: '#/components/schemas/ContactSchema'
 */
extractContacts.post("/", validateRequest(htmlRequestSchema), async (c) => {
  try {
    const result = await contactHandler(c, await c.req.json());
    return c.json(result);
  } catch (error) {
    if (error instanceof Error) {
      throw new HTTPException(500, { message: error.message });
    }
    throw new HTTPException(500, { message: "Unknown error occurred" });
  }
});

export default extractContacts;
