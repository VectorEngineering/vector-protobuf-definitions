import { z } from 'zod';

/**
 * Schema for custom headers
 */
export const HeadersSchema = z.record(z.string());

/**
 * Schema for URL model request body
 */
export const UrlModelSchema = z.object({
    url: z.string().url({ message: "Invalid URL format" }),
    wait_after_load: z.number().int().nonnegative().optional().default(0),
    timeout: z.number().int().positive().optional().default(30000),
    headers: HeadersSchema.optional(),
    check_selector: z.string().optional(),
});

/**
 * Schema for URL extraction request
 */
export const UrlExtractionRequestSchema = z.object({
    url: z.string().url({ message: "Invalid URL format" }),
    includeRelative: z.boolean().optional().default(false),
    filterDomain: z.string().optional(),
    timeout: z.number().int().positive().optional().default(30000),
});

/**
 * Schema for URL extraction response
 */
export const UrlExtractionResponseSchema = z.object({
    urls: z.array(z.object({
        url: z.string(),
        isRelative: z.boolean(),
        domain: z.string(),
    })),
    totalCount: z.number(),
    pageUrl: z.string(),
});

/**
 * Schema for scrape options
 */
export const ScrapeOptionsSchema = z.object({
    waitAfterLoad: z.number().int().nonnegative().optional(),
    timeout: z.number().int().positive().optional(),
    headers: HeadersSchema.optional(),
    checkSelector: z.string().optional(),
});

/**
 * Schema for scrape response
 */
export const ScrapeResponseSchema = z.object({
    content: z.string(),
    pageStatusCode: z.number().int(),
    pageError: z.string().optional(),
});

// Type exports
export type UrlModel = z.infer<typeof UrlModelSchema>;
export type ScrapeOptions = z.infer<typeof ScrapeOptionsSchema>;
export type ScrapeResponse = z.infer<typeof ScrapeResponseSchema>;
export type Headers = z.infer<typeof HeadersSchema>;
export type UrlExtractionRequest = z.infer<typeof UrlExtractionRequestSchema>;
export type UrlExtractionResponse = z.infer<typeof UrlExtractionResponseSchema>;

// Constants
export const DEFAULT_TIMEOUT = 30000;
export const DEFAULT_WAIT_AFTER_LOAD = 1000;

// Validation functions
export const validateUrlModel = (data: unknown): UrlModel => {
    return UrlModelSchema.parse(data);
};

export const validateScrapeOptions = (data: unknown): ScrapeOptions => {
    return ScrapeOptionsSchema.parse(data);
};

export const validateScrapeResponse = (data: unknown): ScrapeResponse => {
    return ScrapeResponseSchema.parse(data);
};

export const validateUrlExtractionRequest = (data: unknown): UrlExtractionRequest => {
    return UrlExtractionRequestSchema.parse(data);
}; 