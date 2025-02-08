import { access, mkdir, readFile, writeFile } from 'fs/promises';
import { dirname, join } from 'path';

import Handlebars from 'handlebars';
import type { OpenAPI } from 'openapi-types';
import chalk from 'chalk';
import { constants } from 'fs';
import { generateZodClientFromOpenAPI } from 'openapi-zod-client';
import { glob } from 'glob';
import ora from 'ora';
import { parse as parseYaml } from 'yaml';

// Register Handlebars helpers
Handlebars.registerHelper('uppercase', (str: string) => str.toUpperCase());
Handlebars.registerHelper('lowercase', (str: string) => str.toLowerCase());
Handlebars.registerHelper(
  'replace',
  (str: string, find: string, replace: string) => str.split(find).join(replace)
);
Handlebars.registerHelper('or', function () {
  return Array.prototype.slice.call(arguments, 0, -1).some(Boolean);
});
Handlebars.registerHelper('includes', (str: string, search: string) => {
  if (!str || typeof str !== 'string') return false;
  return str.includes(search);
});
Handlebars.registerHelper('toOpenAPIPath', (path: string) => {
  if (!path || typeof path !== 'string') return '';
  return path.replace(/:([^/]+)/g, '{$1}');
});
Handlebars.registerHelper('capitalize', (str: string) => {
  if (!str) return '';
  return str.charAt(0).toUpperCase() + str.slice(1);
});
Handlebars.registerHelper('basename', (path: string) => {
  if (!path) return '';
  // Remove leading slash and special characters, then convert to PascalCase
  const clean = path
    .replace(/^\//, '')
    .replace(/[^a-zA-Z0-9]/g, ' ')
    .trim();
  return clean
    .split(/\s+/)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join('');
});
Handlebars.registerHelper('eq', (a: any, b: any) => a === b);
Handlebars.registerHelper('add', (a: number, b: number) => a + b);
Handlebars.registerHelper('lookup', (ref: string, prefix: string) => {
  if (!ref || typeof ref !== 'string') return '';
  // Handle OpenAPI refs like "#/components/schemas/User"
  if (ref.startsWith(prefix)) {
    return ref.slice(prefix.length);
  }
  return ref;
});
Handlebars.registerHelper('routeFilename', (path: string) => {
  if (!path) return '';
  // Remove leading slash and convert path segments to camelCase
  return path
    .replace(/^\//, '')
    .split('/')
    .map((segment) =>
      segment
        .replace(/[^a-zA-Z0-9]/g, ' ')
        .trim()
        .split(/\s+/)
        .map((word, i) =>
          i === 0
            ? word.toLowerCase()
            : word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()
        )
        .join('')
    )
    .join('');
});

Handlebars.registerHelper('routePath', (path: string) => {
  if (!path) return '';
  return path.replace(/\{([^}]+)\}/g, ':$1');
});

Handlebars.registerHelper('replacePathParams', (path: string) => {
  if (!path) return '';
  // Convert {param} to :param format
  return path.replace(/\{([^}]+)\}/g, ':$1');
});

Handlebars.registerHelper('hasQueryParams', function (parameters) {
  return parameters?.some((p: any) => p.in === 'query');
});

interface GenerateOptions {
  input: string;
  output: string;
  template?: string;
  framework?: 'worker' | 'hono';
}

class GenerationError extends Error {
  constructor(message: string, public readonly cause?: unknown) {
    super(message);
    this.name = 'GenerationError';
  }
}

interface SecurityScheme {
  type: 'apiKey' | 'http' | 'oauth2' | 'openIdConnect';
  name?: string;
  in?: 'header' | 'query' | 'cookie';
  scheme?: string;
  flows?: Record<string, unknown>;
  openIdConnectUrl?: string;
}

async function validateOpenAPISpec(spec: any): Promise<OpenAPI.Document> {
  if (!spec || typeof spec !== 'object') {
    throw new GenerationError('Invalid OpenAPI specification: must be an object');
  }

  if (!spec.openapi || !spec.info || !spec.paths) {
    throw new GenerationError('Missing required OpenAPI fields: openapi, info, or paths');
  }

  if (!spec.info.title || !spec.info.version) {
    throw new GenerationError('Missing required info fields: title or version');
  }

  // Validate security schemes
  if (spec.components?.securitySchemes) {
    for (const [name, scheme] of Object.entries<SecurityScheme>(spec.components.securitySchemes)) {
      if (!scheme || typeof scheme !== 'object') {
        throw new GenerationError(`Invalid security scheme ${name}: must be an object`);
      }

      if (!scheme.type) {
        throw new GenerationError(`Security scheme ${name} missing required field: type`);
      }

      // Validate apiKey scheme
      if (scheme.type === 'apiKey') {
        if (!scheme.name) {
          throw new GenerationError(`API key security scheme ${name} missing required field: name`);
        }
        if (!scheme.in || !['header', 'query', 'cookie'].includes(scheme.in)) {
          throw new GenerationError(`API key security scheme ${name} has invalid 'in' field: must be header, query, or cookie`);
        }
      }

      // Validate OAuth2 scheme
      if (scheme.type === 'oauth2') {
        if (!scheme.flows) {
          throw new GenerationError(`OAuth2 security scheme ${name} missing required field: flows`);
        }
      }

      // Validate HTTP scheme
      if (scheme.type === 'http') {
        if (!scheme.scheme) {
          throw new GenerationError(`HTTP security scheme ${name} missing required field: scheme`);
        }
      }
    }
  }

  return spec as OpenAPI.Document;
}

async function validatePaths(options: GenerateOptions): Promise<void> {
  try {
    // Check if input file exists and is readable
    await access(options.input, constants.R_OK);

    // Check if template directory exists if provided
    if (options.template) {
      await access(options.template, constants.R_OK);
    }

    // Check if output directory parent exists and is writable
    const outputParent = dirname(options.output);
    await access(outputParent, constants.W_OK).catch(async () => {
      throw new GenerationError(`Output parent directory ${outputParent} is not writable`);
    });
  } catch (error) {
    if (error instanceof GenerationError) throw error;
    throw new GenerationError('Path validation failed', error);
  }
}

async function compileTemplate(templatePath: string): Promise<HandlebarsTemplateDelegate> {
  try {
    const content = await readFile(templatePath, 'utf-8');
    return Handlebars.compile(content);
  } catch (error) {
    throw new GenerationError(`Failed to compile template ${templatePath}`, error);
  }
}

async function ensureDirectories(srcDir: string, routesDir: string, middlewareDir: string): Promise<void> {
  try {
    await Promise.all([
      mkdir(srcDir, { recursive: true }),
      mkdir(routesDir, { recursive: true }),
      mkdir(middlewareDir, { recursive: true })
    ]);
  } catch (error) {
    throw new GenerationError('Failed to create required directories', error);
  }
}

export async function generate(options: GenerateOptions) {
  const spinner = ora('Generating API client').start();

  try {
    // Validate paths before proceeding
    await validatePaths(options);

    // Read and validate OpenAPI spec
    const openApiContent = await readFile(options.input, 'utf-8');
    let openApiSpec: OpenAPI.Document;

    try {
      const parsedSpec = parseYaml(openApiContent);
      openApiSpec = await validateOpenAPISpec(parsedSpec);
    } catch (error) {
      throw new GenerationError('Failed to parse OpenAPI specification', error);
    }

    // Create directories
    const srcDir = options.output;
    const routesDir = join(srcDir, 'routes');
    const middlewareDir = join(srcDir, 'middleware');
    await ensureDirectories(srcDir, routesDir, middlewareDir);

    // Generate Zod schemas with error handling
    let zodClientResult: string;
    try {
      zodClientResult = await generateZodClientFromOpenAPI({
        openApiDoc: openApiSpec as any,
        disableWriteToFile: true,
        options: {
          withAlias: true,
          defaultStatusBehavior: 'auto-correct',
        },
      });
    } catch (error) {
      throw new GenerationError('Failed to generate Zod schemas', error);
    }

    const modifiedZodClientResult = zodClientResult.replace(
      'import { z } from "zod";',
      'import { z } from "@hono/zod-openapi";'
    );

    // Load and validate templates
    const templateDir = options.template || join(process.cwd(), 'templates');
    let templateFiles: string[];
    try {
      templateFiles = await glob('**/*.hbs', { cwd: templateDir });
      if (templateFiles.length === 0) {
        throw new GenerationError('No template files found');
      }
    } catch (error) {
      throw new GenerationError('Failed to load template files', error);
    }

    // Process templates with proper error handling
    for (const templateFile of templateFiles) {
      const templatePath = join(templateDir, templateFile);

      try {
        if (templateFile === 'route.hbs') {
          // Generate route files
          const template = await compileTemplate(templatePath);

          for (const [path, pathItem] of Object.entries(openApiSpec.paths || {})) {
            const routeFileName = Handlebars.helpers.routeFilename(path);
            const routeFilePath = join(routesDir, `${routeFileName}.ts`);

            const routeContent = template({
              path,
              pathItem,
              openApiSpec,
              zodSchemas: modifiedZodClientResult,
            });

            const modifiedRouteContent = routeContent.replace(
              'import { Hono } from "hono";',
              'import { OpenAPIHono as Hono } from "@hono/zod-openapi";'
            );

            await writeFile(routeFilePath, modifiedRouteContent);
          }

          // Generate index file
          await generateRoutesIndex(routesDir, openApiSpec);
        } else {
          await processOtherTemplate(templateFile, templatePath, srcDir, {
            zodSchemas: modifiedZodClientResult,
            openApiSpec,
          });
        }
      } catch (error) {
        throw new GenerationError(`Failed to process template ${templateFile}`, error);
      }
    }

    spinner.succeed(chalk.green('API client generated successfully'));
  } catch (error) {
    spinner.fail(chalk.red(error instanceof GenerationError ? error.message : 'Failed to generate API client'));
    if (error instanceof GenerationError && error.cause) {
      console.error(chalk.red('Cause:'), error.cause);
    }
    process.exit(1);
  }
}

async function generateRoutesIndex(routesDir: string, openApiSpec: OpenAPI.Document): Promise<void> {
  const indexContent = `import { OpenAPIHono as Hono } from "@hono/zod-openapi";
import type { Env } from '../types';

const router = new Hono<{ Bindings: Env }>();

${Object.entries(openApiSpec.paths || {})
      .map(([path, _]) => {
        const routeName = Handlebars.helpers.routeFilename(path);
        const routePath = Handlebars.helpers.routePath(path);
        return `import { ${routeName}Router } from './${routeName}';
router.route('${routePath}', ${routeName}Router);`;
      })
      .join('\n\n')}

export const ${Handlebars.helpers.basename(openApiSpec.info.title)}Router = router;
`;

  await writeFile(join(routesDir, 'index.ts'), indexContent);
}

async function processOtherTemplate(
  templateFile: string,
  templatePath: string,
  srcDir: string,
  context: any
): Promise<void> {
  const template = await compileTemplate(templatePath);
  let outputPath = join(srcDir, templateFile.replace('.hbs', ''));

  if (!outputPath.endsWith('.json') && !outputPath.endsWith('.toml')) {
    outputPath += '.ts';
  }

  const rendered = template(context);
  await mkdir(dirname(outputPath), { recursive: true });
  await writeFile(outputPath, rendered);
}
