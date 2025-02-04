import { TextGenerationModel } from "./types";
import { z } from "zod";

/**
 * Schema for AI completion request parameters
 * @openapi
 * components:
 *   schemas:
 *     AiCompletionRequest:
 *       type: object
 *       required:
 *         - model
 *         - prompt
 *       properties:
 *         model:
 *           type: string
 *           description: The Cloudflare AI model to use for text generation
 *           example: "@cf/mistral/7b-instruct-v0.2"
 *         prompt:
 *           type: string
 *           description: The input prompt for text generation
 *           minLength: 1
 *           example: "Write a hello world program in Python"
 *         stream:
 *           type: boolean
 *           description: Whether to stream the response
 *           default: false
 *         temperature:
 *           type: number
 *           description: Controls randomness in the output. Higher values produce more random outputs
 *           minimum: 0
 *           maximum: 2
 *           default: 0.7
 *           example: 0.7
 *         maxTokens:
 *           type: integer
 *           description: The maximum number of tokens to generate
 *           minimum: 1
 *           default: 1000
 *           example: 1000
 *         topP:
 *           type: number
 *           description: Controls diversity via nucleus sampling
 *           minimum: 0
 *           maximum: 1
 *           default: 1
 *           example: 0.9
 *         topK:
 *           type: integer
 *           description: Controls diversity via top-k sampling
 *           minimum: 1
 *           example: 50
 *         repetitionPenalty:
 *           type: number
 *           description: Penalizes repetition in the output
 *           minimum: 0
 *           example: 1.1
 *         frequencyPenalty:
 *           type: number
 *           description: Penalizes tokens based on their frequency
 *           minimum: 0
 *           example: 0
 *         presencePenalty:
 *           type: number
 *           description: Penalizes tokens that have already appeared
 *           minimum: 0
 *           example: 0
 *         stop:
 *           type: array
 *           description: Sequences where the model should stop generating
 *           items:
 *             type: string
 *           example: ["\n\n", "```"]
 */
export const aiCompletionRequestSchema = z.object({
    model: z.custom<TextGenerationModel>((val) => {
        return typeof val === 'string' && val.startsWith('@cf/');
    }, {
        message: "Invalid Cloudflare AI model. Must be a valid @cf/ model identifier."
    }),
    prompt: z.string().min(1),
    stream: z.boolean().optional().default(false),
    temperature: z.number().min(0).max(2).optional().default(0.7),
    maxTokens: z.number().positive().optional().default(1000),
    topP: z.number().min(0).max(1).optional().default(1),
    topK: z.number().positive().optional(),
    repetitionPenalty: z.number().min(0).optional(),
    frequencyPenalty: z.number().min(0).optional(),
    presencePenalty: z.number().min(0).optional(),
    stop: z.array(z.string()).optional(),
});

/**
 * Schema for AI model capabilities
 * @openapi
 * components:
 *   schemas:
 *     AiModelCapabilities:
 *       type: object
 *       required:
 *         - streaming
 *       properties:
 *         streaming:
 *           type: boolean
 *           description: Whether the model supports streaming responses
 *         functionCalling:
 *           type: boolean
 *           description: Whether the model supports function calling
 *         lora:
 *           type: boolean
 *           description: Whether the model supports LoRA adapters
 */
export const aiModelCapabilitiesSchema = z.object({
    streaming: z.boolean(),
    functionCalling: z.boolean().optional(),
    lora: z.boolean().optional(),
});

/**
 * Schema for AI model information
 * @openapi
 * components:
 *   schemas:
 *     AiModel:
 *       type: object
 *       required:
 *         - id
 *         - available
 *         - capabilities
 *       properties:
 *         id:
 *           type: string
 *           description: The model identifier
 *           example: "@cf/mistral/7b-instruct-v0.2"
 *         available:
 *           type: boolean
 *           description: Whether the model is currently available
 *         latency:
 *           type: number
 *           description: The model's current latency in milliseconds
 *         capabilities:
 *           $ref: '#/components/schemas/AiModelCapabilities'
 */
export const aiModelSchema = z.object({
    id: z.custom<TextGenerationModel>(),
    available: z.boolean(),
    latency: z.number().optional(),
    capabilities: aiModelCapabilitiesSchema,
});

/**
 * Schema for AI service status response
 * @openapi
 * components:
 *   schemas:
 *     AiStatusResponse:
 *       type: object
 *       required:
 *         - status
 *         - models
 *         - lastChecked
 *       properties:
 *         status:
 *           type: string
 *           enum: [healthy, degraded, down]
 *           description: The current status of the AI service
 *         models:
 *           type: array
 *           description: List of available AI models
 *           items:
 *             $ref: '#/components/schemas/AiModel'
 *         lastChecked:
 *           type: string
 *           format: date-time
 *           description: When the status was last checked
 */
export const aiStatusResponseSchema = z.object({
    status: z.enum(["healthy", "degraded", "down"]),
    models: z.array(aiModelSchema),
    lastChecked: z.string().datetime(),
});

// Type exports using Zod inference
export type AiCompletionRequest = z.infer<typeof aiCompletionRequestSchema>;
export type AiStatusResponse = z.infer<typeof aiStatusResponseSchema>;
export type AiModel = z.infer<typeof aiModelSchema>; 