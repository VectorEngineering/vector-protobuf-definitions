import { z } from "zod";

/**
 * Schema for HTML content request
 */
export const htmlRequestSchema = z.object({
    html: z.string().min(1),
    options: z.object({
        includeOriginalText: z.boolean().optional().default(false),
    }).optional(),
});

/**
 * Schema for text embedding response
 */
export const EmbeddingResponseSchema = z.object({
    text: z.string(),
    embedding: z.array(z.number()),
    metadata: z.object({
        dimensions: z.number(),
        model: z.string(),
        truncated: z.boolean().optional(),
    }),
});

/**
 * Schema for personalized message generation
 */
export const PersonalizedMessageSchema = z.object({
    subject: z.string(),
    body: z.string(),
    callToAction: z.string(),
    customization: z.object({
        painPoints: z.array(z.string()),
        valuePropositions: z.array(z.string()),
        relevantExperience: z.string().optional(),
    }),
});

/**
 * Schema for integrated lead generation response
 */
export const LeadGenResponseSchema = z.object({
    contacts: z.object({
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
    }),
    company: z.object({
        name: z.string(),
        description: z.string(),
        size: z.enum(['SMALL', 'MEDIUM', 'LARGE', 'ENTERPRISE']).optional(),
        founded: z.string().optional(),
        headquarters: z.string().optional(),
        industry: z.object({
            primary: z.string(),
            secondary: z.array(z.string()),
            segments: z.array(z.string()),
        }),
        products: z.array(z.object({
            name: z.string(),
            description: z.string(),
            category: z.string(),
        })).optional(),
        businessModel: z.enum(['B2B', 'B2C', 'B2B2C', 'D2C']).optional(),
        marketFocus: z.array(z.string()),
        competitors: z.array(z.string()).optional(),
        technologies: z.array(z.string()).optional(),
    }),
    analysis: z.object({
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
            level: z.enum(['HIGH', 'MEDIUM', 'LOW']),
            signals: z.array(z.object({
                type: z.string(),
                description: z.string(),
                confidence: z.number().min(0).max(1),
            })),
            keywords: z.array(z.string()),
        }),
        painPoints: z.array(z.object({
            category: z.string(),
            description: z.string(),
            severity: z.enum(['HIGH', 'MEDIUM', 'LOW']),
        })),
        readiness: z.object({
            stage: z.enum(['AWARENESS', 'CONSIDERATION', 'DECISION']),
            indicators: z.array(z.string()),
            nextSteps: z.array(z.string()),
        }),
    }),
    personalization: PersonalizedMessageSchema,
});

// Export all schemas as a module
export default {
    htmlRequestSchema,
    EmbeddingResponseSchema,
    PersonalizedMessageSchema,
    LeadGenResponseSchema,
}; 