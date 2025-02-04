import { env } from "hono/adapter";
import type { Context } from "hono";

// Log levels for controlling output verbosity
export enum LogLevel {
  ERROR = 0,
  WARN = 1,
  INFO = 2,
  DEBUG = 3,
  TRACE = 4,
}

// Interface for structured log metadata
interface LogMetadata {
  requestId?: string;
  path?: string;
  method?: string;
  ip?: string;
  userAgent?: string;
  duration?: number;
  [key: string]: any;
}

// Configuration interface for logger
interface LoggerConfig {
  level: LogLevel;
  enableColors?: boolean;
  redactFields?: string[];
  format?: "json" | "text";
}

// Default configuration
const DEFAULT_CONFIG: LoggerConfig = {
  level: LogLevel.INFO,
  enableColors: false,
  redactFields: ["password", "token", "secret", "key"],
  format: "json",
};

// ANSI color codes for terminal output
const Colors = {
  reset: "\x1b[0m",
  red: "\x1b[31m",
  yellow: "\x1b[33m",
  blue: "\x1b[34m",
  gray: "\x1b[90m",
  green: "\x1b[32m",
};

export class Logger {
  private config: LoggerConfig;

  constructor(config: Partial<LoggerConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  private getRequestMetadata(c: Context): LogMetadata {
    const req = c.req;
    return {
      requestId: c.req.raw.headers.get("cf-ray") || crypto.randomUUID(),
      path: req.path,
      method: req.method,
      ip: c.req.raw.headers.get("cf-connecting-ip") || undefined,
      userAgent: req.header("user-agent"),
      cfCountry: c.req.raw.headers.get("cf-ipcountry"),
      timestamp: new Date().toISOString(),
    };
  }

  private redactSensitiveData(data: any): any {
    if (typeof data !== "object" || data === null) return data;

    const redacted = { ...data };
    for (const field of this.config.redactFields!) {
      if (field in redacted) {
        redacted[field] = "[REDACTED]";
      }
    }
    return redacted;
  }

  private formatMessage(
    level: string,
    message: string,
    metadata: LogMetadata,
    args: any[],
  ): string {
    const redactedMetadata = this.redactSensitiveData(metadata);
    const redactedArgs = args.map((arg) => this.redactSensitiveData(arg));

    if (this.config.format === "json") {
      return JSON.stringify({
        level,
        message,
        ...redactedMetadata,
        ...(redactedArgs.length && { args: redactedArgs }),
      });
    }

    const color = this.config.enableColors
      ? {
          ERROR: Colors.red,
          WARN: Colors.yellow,
          INFO: Colors.blue,
          DEBUG: Colors.gray,
          TRACE: Colors.green,
        }[level]
      : "";

    const reset = this.config.enableColors ? Colors.reset : "";
    const metadataStr = Object.entries(redactedMetadata)
      .map(([k, v]) => `${k}=${v}`)
      .join(" ");

    return `${color}[${level}]${reset} ${metadata.timestamp} [${metadata.requestId}] ${message} ${metadataStr} ${redactedArgs.length ? JSON.stringify(redactedArgs) : ""}`;
  }

  log(
    c: Context,
    level: LogLevel,
    message: string,
    metadata: Partial<LogMetadata> = {},
    ...args: any[]
  ) {
    const { NODE_ENV } = env(c);

    // Skip logging if level is higher than configured
    if (level > this.config.level) return;

    // Skip test logs in test environment
    if (NODE_ENV === "test" && level > LogLevel.ERROR) return;

    const combinedMetadata = {
      ...this.getRequestMetadata(c),
      ...metadata,
    };

    const formattedMessage = this.formatMessage(
      LogLevel[level],
      message,
      combinedMetadata,
      args,
    );

    switch (level) {
      case LogLevel.ERROR:
        console.error(formattedMessage);
        break;
      case LogLevel.WARN:
        console.warn(formattedMessage);
        break;
      case LogLevel.INFO:
        console.log(formattedMessage);
        break;
      case LogLevel.DEBUG:
      case LogLevel.TRACE:
        console.debug(formattedMessage);
        break;
    }
  }

  // Convenience methods
  error(
    c: Context,
    message: string,
    metadata?: Partial<LogMetadata>,
    ...args: any[]
  ) {
    this.log(c, LogLevel.ERROR, message, metadata, ...args);
  }

  warn(
    c: Context,
    message: string,
    metadata?: Partial<LogMetadata>,
    ...args: any[]
  ) {
    this.log(c, LogLevel.WARN, message, metadata, ...args);
  }

  info(
    c: Context,
    message: string,
    metadata?: Partial<LogMetadata>,
    ...args: any[]
  ) {
    this.log(c, LogLevel.INFO, message, metadata, ...args);
  }

  debug(
    c: Context,
    message: string,
    metadata?: Partial<LogMetadata>,
    ...args: any[]
  ) {
    this.log(c, LogLevel.DEBUG, message, metadata, ...args);
  }

  trace(
    c: Context,
    message: string,
    metadata?: Partial<LogMetadata>,
    ...args: any[]
  ) {
    this.log(c, LogLevel.TRACE, message, metadata, ...args);
  }
}

// Create default logger instance
export const logger = new Logger();

// Example usage:
/*
import { logger } from './logger';

app.use('*', async (c, next) => {
  const startTime = Date.now();
  
  try {
    await next();
  } catch (error) {
    logger.error(c, 'Request failed', { error });
    throw error;
  }

  logger.info(c, 'Request completed', {
    duration: Date.now() - startTime,
    status: c.res.status
  });
});
*/
