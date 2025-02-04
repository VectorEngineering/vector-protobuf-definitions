import { BatchBusinessDataSchema, BatchResponseSchema } from "../routes/webpage/schemas";

import { Context } from "hono";
import { z } from "zod";

type BatchProcessor<T> = (c: Context, data: any) => Promise<T>;
type BatchOptions = z.infer<typeof BatchBusinessDataSchema>["options"];
type BatchItem = z.infer<typeof BatchBusinessDataSchema>["items"][number];

interface BatchResult {
    id: string;
    status: 'success' | 'error';
    data?: any;
    error?: string;
}

/**
 * Process items in a batch with controlled concurrency
 */
async function processBatch<T>(
    items: BatchItem[],
    processor: BatchProcessor<T>,
    c: Context,
    concurrency: number,
    failFast: boolean
): Promise<BatchResult[]> {
    const results: BatchResult[] = [];
    const queue = [...items];
    const inProgress = new Set<Promise<void>>();

    while (queue.length > 0 || inProgress.size > 0) {
        // Fill up to concurrency limit
        while (queue.length > 0 && inProgress.size < concurrency) {
            const item = queue.shift()!;

            const promise = (async () => {
                try {
                    const data = await processor(c, item.data);
                    results.push({
                        id: item.id,
                        status: 'success',
                        data
                    });
                } catch (error) {
                    const errorResult = {
                        id: item.id,
                        status: 'error' as const,
                        error: error instanceof Error ? error.message : 'Unknown error'
                    };
                    results.push(errorResult);

                    if (failFast) {
                        // Clear queue if failFast is true
                        queue.length = 0;
                        throw error;
                    }
                }
            })();

            inProgress.add(promise);

            // Clean up promise from inProgress after completion
            promise.finally(() => {
                inProgress.delete(promise);
            });
        }

        // Wait for at least one promise to complete before next iteration
        if (inProgress.size >= concurrency || (queue.length === 0 && inProgress.size > 0)) {
            await Promise.race(inProgress);
        }
    }

    return results;
}

/**
 * Handle batch processing request with proper error handling and response formatting
 */
export async function handleBatchRequest<T>(
    c: Context,
    processor: BatchProcessor<T>
): Promise<typeof BatchResponseSchema._type> {
    const startTime = Date.now();
    const body = await c.req.json();
    const { items, options = { failFast: false, concurrency: 5 } } = BatchBusinessDataSchema.parse(body);

    const { failFast, concurrency } = options;

    try {
        const results = await processBatch(
            items,
            processor,
            c,
            concurrency,
            failFast
        );

        const successful = results.filter(r => r.status === 'success').length;
        const failed = results.filter(r => r.status === 'error').length;

        return {
            results,
            summary: {
                total: items.length,
                successful,
                failed,
                processingTime: Date.now() - startTime
            }
        };
    } catch (error) {
        // If failFast is true and an error occurred, include all unprocessed items as failed
        const processedIds = new Set(items.map(i => i.id));
        const results = items.map(item => ({
            id: item.id,
            status: 'error' as const,
            error: error instanceof Error ? error.message : 'Batch processing failed'
        }));

        return {
            results,
            summary: {
                total: items.length,
                successful: 0,
                failed: items.length,
                processingTime: Date.now() - startTime
            }
        };
    }
} 