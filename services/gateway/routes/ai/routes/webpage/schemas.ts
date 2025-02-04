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

/**
 * Schema for business hours analysis
 */
export const BusinessHoursAnalysisSchema = z.object({
    bestTimeToContact: z.array(z.object({
        day: z.string(),
        timeRanges: z.array(z.object({
            start: z.string(),
            end: z.string(),
            score: z.number().min(0).max(100),
            reason: z.string()
        })),
        busynessScore: z.number().min(0).max(100)
    })),
    peakHours: z.array(z.object({
        day: z.string(),
        time: z.string(),
        score: z.number().min(0).max(100)
    })),
    operationalInsights: z.array(z.object({
        type: z.string(),
        insight: z.string(),
        relevance: z.number().min(0).max(100)
    }))
});

/**
 * Schema for business qualification
 */
export const BusinessQualificationSchema = z.object({
    qualificationScore: z.object({
        total: z.number().min(0).max(100),
        breakdown: z.object({
            location: z.number().min(0).max(100),
            businessType: z.number().min(0).max(100),
            customerBase: z.number().min(0).max(100),
            operationalMaturity: z.number().min(0).max(100),
            marketPresence: z.number().min(0).max(100)
        })
    }),
    matchCriteria: z.array(z.object({
        criterion: z.string(),
        score: z.number().min(0).max(100),
        evidence: z.array(z.string())
    })),
    riskFactors: z.array(z.object({
        factor: z.string(),
        severity: z.enum(['HIGH', 'MEDIUM', 'LOW']),
        mitigation: z.string().optional()
    })),
    growthIndicators: z.array(z.object({
        indicator: z.string(),
        trend: z.enum(['INCREASING', 'STABLE', 'DECREASING']),
        confidence: z.number().min(0).max(100)
    }))
});

/**
 * Schema for business intelligence
 */
export const BusinessIntelligenceSchema = z.object({
    customerInsights: z.object({
        demographics: z.array(z.string()),
        preferences: z.array(z.string()),
        behavior: z.array(z.object({
            pattern: z.string(),
            significance: z.number().min(0).max(100)
        }))
    }),
    competitivePosition: z.object({
        strengths: z.array(z.string()),
        uniqueSellingPoints: z.array(z.string()),
        marketDifferentiators: z.array(z.string())
    }),
    operationalMetrics: z.object({
        averageRating: z.number().min(0).max(5),
        reviewSentiment: z.object({
            positive: z.number().min(0).max(100),
            negative: z.number().min(0).max(100),
            neutral: z.number().min(0).max(100)
        }),
        peakPerformance: z.array(z.object({
            metric: z.string(),
            value: z.number(),
            context: z.string()
        }))
    }),
    recommendations: z.array(z.object({
        area: z.string(),
        suggestion: z.string(),
        impact: z.enum(['HIGH', 'MEDIUM', 'LOW']),
        feasibility: z.enum(['HIGH', 'MEDIUM', 'LOW'])
    }))
});

/**
 * Schema for batch business data request
 */
export const BatchBusinessDataSchema = z.object({
    items: z.array(z.object({
        id: z.string(),  // Unique identifier for each item in batch
        data: z.any()
    })).min(1).max(100),  // Limit batch size to 100 items
    options: z.object({
        failFast: z.boolean().optional().default(false),  // Stop on first error
        concurrency: z.number().min(1).max(10).optional().default(5)  // Max concurrent processing
    }).optional()
});

/**
 * Schema for batch processing response
 */
export const BatchResponseSchema = z.object({
    results: z.array(z.object({
        id: z.string(),
        status: z.enum(['success', 'error']),
        data: z.any().optional(),
        error: z.string().optional()
    })),
    summary: z.object({
        total: z.number(),
        successful: z.number(),
        failed: z.number(),
        processingTime: z.number()  // in milliseconds
    })
});

// Export all schemas as a module
export default {
    htmlRequestSchema,
    EmbeddingResponseSchema,
    PersonalizedMessageSchema,
    LeadGenResponseSchema,
    BusinessHoursAnalysisSchema,
    BusinessQualificationSchema,
    BusinessIntelligenceSchema,
    BatchBusinessDataSchema,
    BatchResponseSchema,
}; 