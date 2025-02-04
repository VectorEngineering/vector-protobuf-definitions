import { BusinessHoursAnalysisSchema, BusinessIntelligenceSchema, BusinessQualificationSchema } from "../routes/webpage/schemas";
import {
    analyzeStructure,
    cleanHtml,
    extractMainContent,
    extractMetadata,
    extractText,
    sanitizeHtml
} from "./html";
import { extractContentSections, extractLists, extractTables } from "./content";

import { HTTPException } from "hono/http-exception";
import { LeadGenResponseSchema } from "../routes/webpage/schemas";
import { createCompletionHandler } from "./completion";
import { htmlToMarkdown } from "./markdown";

/**
 * Handler for converting HTML to markdown
 */
export const convertHandler = createCompletionHandler({
    systemPrompt: "You are an expert at converting HTML to clean, well-formatted markdown. Follow these rules:\n" +
        "1. Preserve the document structure and hierarchy\n" +
        "2. Keep important formatting (bold, italic, lists)\n" +
        "3. Maintain links but simplify them\n" +
        "4. Remove unnecessary formatting and clutter",
    defaultModel: "@cf/mistral/7b-instruct-v0.2",
    inputTransformer: (html) => {
        const cleaned = cleanHtml(html);
        const content = extractMainContent(cleaned);
        const basicMarkdown = htmlToMarkdown(content);
        return `Convert this content to clean markdown:\n\n${basicMarkdown}`;
    },
    defaultParams: {
        temperature: 0.3,
        maxTokens: 2000,
    },
});

/**
 * Handler for enriching content with additional context
 */
export const enrichHandler = createCompletionHandler({
    systemPrompt: "You are an expert at analyzing web content and providing enriched context. For each piece of content:\n" +
        "1. Identify key topics and themes\n" +
        "2. Add relevant context and background information\n" +
        "3. Include citations and references where appropriate\n" +
        "4. Highlight important insights",
    defaultModel: "@cf/mistral/7b-instruct-v0.2",
    inputTransformer: (html) => {
        const cleaned = cleanHtml(html);
        const content = extractMainContent(cleaned);
        const metadata = extractMetadata(html);
        const markdown = htmlToMarkdown(content);

        return `Analyze and enrich this content with additional context:
Title: ${metadata.title || 'Untitled'}
Description: ${metadata.description || 'No description'}
Author: ${metadata.author || 'Unknown'}
Date: ${metadata.date || metadata.published_time || 'Unknown'}

Content:
${markdown}`;
    },
    defaultParams: {
        temperature: 0.7,
        maxTokens: 2000,
    },
});

/**
 * Handler for generating content summaries
 */
export const summaryHandler = createCompletionHandler({
    systemPrompt: "You are an expert at creating concise, informative summaries. For each piece of content:\n" +
        "1. Identify the main points and key takeaways\n" +
        "2. Create a clear, structured summary\n" +
        "3. Include important details but remain concise\n" +
        "4. Use markdown formatting for clarity",
    defaultModel: "@cf/mistral/7b-instruct-v0.2",
    inputTransformer: (html) => {
        const cleaned = cleanHtml(html);
        const content = extractMainContent(cleaned);
        const metadata = extractMetadata(html);
        const markdown = htmlToMarkdown(content);

        return `Create a concise summary of this content:
Title: ${metadata.title || 'Untitled'}
Description: ${metadata.description || 'No description'}

Content:
${markdown}`;
    },
    defaultParams: {
        temperature: 0.5,
        maxTokens: 1000,
    },
});

/**
 * Handler for analyzing HTML structure
 */
export const structureHandler = createCompletionHandler({
    systemPrompt: "You are an expert at analyzing HTML structure and providing insights. For each document:\n" +
        "1. Identify the main structural elements\n" +
        "2. Analyze the content hierarchy\n" +
        "3. Highlight important patterns and relationships\n" +
        "4. Provide recommendations for improvement",
    defaultModel: "@cf/mistral/7b-instruct-v0.2",
    inputTransformer: (html) => {
        const cleaned = cleanHtml(html);
        const structure = analyzeStructure(cleaned);
        const sections = extractContentSections(cleaned);
        const lists = extractLists(cleaned);
        const tables = extractTables(cleaned);

        return `Analyze this HTML structure and provide insights:

Document Structure:
${JSON.stringify(structure, null, 2)}

Content Sections:
${sections.map(s => `# ${s.heading}\n${s.content}`).join('\n\n')}

Lists:
Ordered Lists: ${lists.ordered.length}
Unordered Lists: ${lists.unordered.length}

Tables:
${tables.length} tables found

Please provide a detailed analysis of the structure and recommendations for improvement.`;
    },
    defaultParams: {
        temperature: 0.3,
        maxTokens: 2000,
    },
});

/**
 * Text embedding handler
 */
export const embeddingHandler = async (c: any, input: { html: string, options?: { includeOriginalText?: boolean } }) => {
    try {
        const text = extractText(input.html);

        // Call the Cloudflare AI Embeddings API
        const response = await c.env.AI.run('@cf/baai/bge-large-en-v1.5', {
            text: text
        });

        // Construct the response
        const result = {
            embedding: response,
            metadata: {
                dimensions: 1024,
                model: '@cf/baai/bge-large-en-v1.5',
            }
        };

        // Include original text if requested
        if (input.options?.includeOriginalText) {
            return {
                ...result,
                text
            };
        }

        return result;
    } catch (error) {
        if (error instanceof Error) {
            throw new HTTPException(500, { message: error.message });
        }
        throw new HTTPException(500, { message: "Failed to generate embeddings" });
    }
};

/**
 * Lead generation handler
 */
export const leadgenHandler = createCompletionHandler({
    systemPrompt: `You are an expert at lead generation and analysis. Process the webpage content according to these steps:

1. Extract all contact information:
   - Email addresses (handle various formats)
   - Phone numbers (standardize format)
   - Social media profiles
   - Physical addresses
   - Websites

2. Identify company and industry details:
   - Company name and description
   - Industry classification
   - Business segments
   - Products/services
   - Business model
   - Market focus
   - Technologies used

3. Analyze lead potential:
   - Calculate lead score based on multiple factors
   - Identify intent signals and keywords
   - Detect pain points and challenges
   - Determine buying stage
   - Evaluate engagement indicators

4. Generate personalized outreach:
   - Create compelling subject line
   - Write personalized message body
   - Include relevant call to action
   - Reference specific pain points
   - Highlight value propositions
   - Mention relevant experience

Format the response as a single JSON object containing all extracted and analyzed information.
Ensure all data is factual and properly validated.
Do not include speculative information.`,
    defaultModel: "@cf/mistral/7b-instruct-v0.2",
    inputTransformer: (html) => {
        const text = extractText(html);
        const metadata = extractMetadata(html);
        return `Analyze this webpage content for lead generation. Extract all relevant information and format as JSON:\n\nMetadata:\n${JSON.stringify(metadata)}\n\nContent:\n${text}`;
    },
    outputTransformer: (output) => {
        try {
            const parsed = JSON.parse(output);
            const validated = LeadGenResponseSchema.parse(parsed);
            return JSON.stringify(validated);
        } catch (error) {
            throw new Error("Invalid lead generation response format");
        }
    },
    defaultParams: {
        temperature: 0.2,
        maxTokens: 2500,
    },
});

/**
 * Handler for analyzing business hours and contact timing
 */
export const businessHoursHandler = createCompletionHandler({
    systemPrompt: `You are an expert at analyzing business operational patterns and determining optimal contact times. Analyze the provided business data to:
1. Determine the best times to contact based on operational hours and traffic patterns
2. Identify peak business hours and quiet periods
3. Extract operational insights that could impact engagement
4. Consider day-of-week variations and seasonal patterns
5. Account for business type and customer behavior
6. Return only factual, data-driven insights`,
    defaultModel: "@cf/mistral/7b-instruct-v0.2",
    inputTransformer: (data) => {
        return `Analyze this business data to determine optimal contact times and operational patterns. Format as JSON:

Business Data:
${JSON.stringify(data, null, 2)}`;
    },
    outputTransformer: (output) => {
        try {
            const parsed = JSON.parse(output);
            const validated = BusinessHoursAnalysisSchema.parse(parsed);
            return JSON.stringify(validated);
        } catch (error) {
            throw new Error("Invalid business hours analysis format");
        }
    },
    defaultParams: {
        temperature: 0.2,
        maxTokens: 1500,
    },
});

/**
 * Handler for business qualification analysis
 */
export const businessQualificationHandler = createCompletionHandler({
    systemPrompt: `You are an expert at qualifying business leads and assessing their potential value. Analyze the provided business data to:
1. Calculate overall qualification score based on multiple criteria
2. Evaluate location and market presence
3. Assess business type and operational maturity
4. Identify customer base characteristics
5. Detect risk factors and growth indicators
6. Provide evidence-based scoring
7. Return only factual, verifiable information`,
    defaultModel: "@cf/mistral/7b-instruct-v0.2",
    inputTransformer: (data) => {
        return `Analyze this business data to assess qualification and potential value. Format as JSON:

Business Data:
${JSON.stringify(data, null, 2)}`;
    },
    outputTransformer: (output) => {
        try {
            const parsed = JSON.parse(output);
            const validated = BusinessQualificationSchema.parse(parsed);
            return JSON.stringify(validated);
        } catch (error) {
            throw new Error("Invalid business qualification format");
        }
    },
    defaultParams: {
        temperature: 0.3,
        maxTokens: 2000,
    },
});

/**
 * Handler for business intelligence analysis
 */
export const businessIntelligenceHandler = createCompletionHandler({
    systemPrompt: `You are an expert at extracting business intelligence and market insights. Analyze the provided business data to:
1. Extract customer demographics and behavior patterns
2. Identify competitive strengths and unique selling points
3. Analyze operational metrics and performance indicators
4. Generate actionable recommendations
5. Consider market positioning and differentiation
6. Evaluate customer sentiment and satisfaction
7. Return only factual, data-supported insights`,
    defaultModel: "@cf/mistral/7b-instruct-v0.2",
    inputTransformer: (data) => {
        return `Analyze this business data to extract intelligence and insights. Format as JSON:

Business Data:
${JSON.stringify(data, null, 2)}`;
    },
    outputTransformer: (output) => {
        try {
            const parsed = JSON.parse(output);
            const validated = BusinessIntelligenceSchema.parse(parsed);
            return JSON.stringify(validated);
        } catch (error) {
            throw new Error("Invalid business intelligence format");
        }
    },
    defaultParams: {
        temperature: 0.3,
        maxTokens: 2000,
    },
}); 