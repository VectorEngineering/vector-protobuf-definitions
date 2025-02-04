/**
 * Supported Cloudflare text generation models
 */
export type TextGenerationModel =
  | "@cf/deepseek-coder/6.7b-base-awq"
  | "@cf/deepseek-coder/6.7b-instruct-awq"
  | "@cf/deepseek/math-7b-instruct"
  | "@cf/deepseek/r1-distill-qwen-32b"
  | "@cf/falcon/7b-instruct"
  | "@cf/gemma/2b-it-lora"
  | "@cf/gemma/7b-it-lora"
  | "@cf/gemma/7b-it"
  | "@cf/llama-2/7b-chat-fp16"
  | "@cf/llama-2/7b-chat-int8"
  | "@cf/llama-2/13b-chat-awq"
  | "@cf/llama-3/8b-instruct"
  | "@cf/llama-3/8b-instruct-awq"
  | "@cf/llama-3.1/70b-instruct"
  | "@cf/llama-3.1/8b-instruct-awq"
  | "@cf/llama-3.2/1b-instruct"
  | "@cf/llama-3.2/3b-instruct"
  | "@cf/llama-3.3/70b-instruct-fp8-fast"
  | "@cf/mistral/7b-instruct-v0.1"
  | "@cf/mistral/7b-instruct-v0.2"
  | "@cf/mistral/7b-instruct-v0.2-lora"
  | "@cf/openchat/3.5-0106"
  | "@cf/phi-2"
  | "@cf/qwen/1.5-0.5b-chat"
  | "@cf/qwen/1.5-1.8b-chat"
  | "@cf/qwen/1.5-7b-chat-awq"
  | "@cf/qwen/1.5-14b-chat-awq";

/**
 * AI Model information
 */
export interface AiModel {
  id: TextGenerationModel;
  available: boolean;
  latency?: number;
  capabilities: {
    streaming: boolean;
    functionCalling?: boolean;
    lora?: boolean;
  };
}

/**
 * AI completion options
 */
export interface AiOptions {
  temperature?: number;
  maxTokens?: number;
  topP?: number;
  topK?: number;
  repetitionPenalty?: number;
  frequencyPenalty?: number;
  presencePenalty?: number;
  stop?: string[];
}

/**
 * AI Service interface
 */
export interface AiService {
  init(model: TextGenerationModel): Promise<{
    stream(prompt: string, options: AiOptions): Promise<ReadableStream>;
    complete(prompt: string, options: AiOptions): Promise<string>;
  }>;
  listModels(): Promise<AiModel[]>;
} 