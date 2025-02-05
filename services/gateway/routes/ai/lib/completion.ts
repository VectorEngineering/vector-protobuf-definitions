import { AiCompletionRequest } from "../schema";
import { Env } from "../../../types";
import { HTTPException } from "hono/http-exception";
import { TextGenerationModel } from "../types";

/**
 * Configuration options for the completion handler
 */
export interface CompletionConfig {
  /**
   * System prompt to be prepended to user input
   * @example "You are a helpful assistant that speaks in a friendly tone."
   */
  systemPrompt?: string;

  /**
   * Function to transform the user input before sending to the model
   * @param input - The raw user input
   * @returns The transformed input
   */
  inputTransformer?: (input: string) => string;

  /**
   * Function to transform the model's output before sending to the user
   * @param output - The raw model output
   * @returns The transformed output
   */
  outputTransformer?: (output: string) => string;

  /**
   * Default model to use if none specified
   */
  defaultModel?: TextGenerationModel;

  /**
   * Default parameters for the completion
   */
  defaultParams?: Partial<AiCompletionRequest>;
}

/**
 * Creates a completion handler with the specified configuration
 *
 * @param config - Configuration options for the completion handler
 * @returns A function that handles completions with the specified configuration
 *
 * @example
 * ```typescript
 * const chatHandler = createCompletionHandler({
 *   systemPrompt: "You are a helpful assistant.",
 *   defaultModel: "@cf/mistral/7b-instruct-v0.2",
 *   defaultParams: { temperature: 0.7 }
 * });
 *
 * // Use in route handler
 * app.post("/chat", async (c) => {
 *   const result = await chatHandler(c, await c.req.json());
 *   return result;
 * });
 * ```
 */
export function createCompletionHandler(config: CompletionConfig = {}) {
  const {
    systemPrompt = "",
    inputTransformer = (input) => input,
    outputTransformer = (output) => output,
    defaultModel = "@cf/mistral/7b-instruct-v0.2",
    defaultParams = {},
  } = config;

  /**
   * Handles an AI completion request with the specified configuration
   *
   * @param c - The Hono context
   * @param params - The completion request parameters
   * @returns A Response object with either streaming or non-streaming content
   * @throws {HTTPException} If an error occurs during completion
   */
  return async function handleCompletion(
    c: { env: Env },
    params: Partial<AiCompletionRequest>,
  ): Promise<Response> {
    try {
      const { model = defaultModel, prompt, stream = false, ...rest } = params;

      if (!prompt) {
        throw new HTTPException(400, { message: "Prompt is required" });
      }

      // Merge default parameters with request parameters
      const completionParams = {
        ...defaultParams,
        ...rest,
      };

      // Transform input with system prompt
      const fullPrompt = systemPrompt
        ? `${systemPrompt}\n\n${inputTransformer(prompt)}`
        : inputTransformer(prompt);

      // Initialize AI instance
      const ai = await c.env.AI.init(model);

      if (stream) {
        // Handle streaming response
        const stream = await ai.stream(fullPrompt, completionParams);
        return new Response(stream, {
          headers: {
            "Content-Type": "text/event-stream",
            "Cache-Control": "no-cache",
            Connection: "keep-alive",
          },
        });
      }

      // Handle non-streaming response
      const completion = await ai.complete(fullPrompt, completionParams);
      return new Response(
        JSON.stringify({ completion: outputTransformer(completion) }),
        {
          headers: {
            "Content-Type": "application/json",
          },
        },
      );
    } catch (error) {
      if (error instanceof Error) {
        throw new HTTPException(500, { message: error.message });
      }
      throw new HTTPException(500, { message: "Unknown error occurred" });
    }
  };
}

/**
 * Common completion configurations for different use cases
 */
export const completionConfigs = {
  /**
   * Configuration for a chat-like completion handler
   */
  chat: {
    systemPrompt:
      "You are a helpful AI assistant. Respond in a friendly and concise manner.",
    defaultModel: "@cf/mistral/7b-instruct-v0.2",
    defaultParams: {
      temperature: 0.7,
      maxTokens: 1000,
      topP: 0.9,
    },
  },

  /**
   * Configuration for a code completion handler
   */
  code: {
    systemPrompt:
      "You are an expert programmer. Provide clear, well-documented code with explanations.",
    defaultModel: "@cf/deepseek-coder/6.7b-instruct-awq",
    defaultParams: {
      temperature: 0.3,
      maxTokens: 2000,
      topP: 0.95,
      stop: ["```"],
    },
  },

  /**
   * Configuration for a math problem solver
   */
  math: {
    systemPrompt:
      "You are a mathematics expert. Show your work and explain each step clearly.",
    defaultModel: "@cf/deepseek/math-7b-instruct",
    defaultParams: {
      temperature: 0.2,
      maxTokens: 1500,
      topP: 0.9,
    },
  },
} as const;
