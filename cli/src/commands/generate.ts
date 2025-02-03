import { dirname, join } from 'path';
import { mkdir, readFile, writeFile } from 'fs/promises';

import Handlebars from 'handlebars';
import type { OpenAPI } from 'openapi-types';
import chalk from 'chalk';
import { generateZodClientFromOpenAPI } from 'openapi-zod-client';
import { glob } from 'glob';
import ora from 'ora';
import { parse as parseYaml } from 'yaml';

// Register Handlebars helpers
Handlebars.registerHelper('uppercase', (str: string) => str.toUpperCase());
Handlebars.registerHelper('lowercase', (str: string) => str.toLowerCase());
Handlebars.registerHelper('capitalize', (str: string) => {
    if (!str) return '';
    return str.charAt(0).toUpperCase() + str.slice(1);
});
Handlebars.registerHelper('basename', (path: string) => {
    if (!path) return '';
    // Remove leading slash and special characters, then convert to PascalCase
    const clean = path.replace(/^\//, '').replace(/[^a-zA-Z0-9]/g, ' ').trim();
    return clean
        .split(/\s+/)
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join('');
});
Handlebars.registerHelper('eq', (a: any, b: any) => a === b);
Handlebars.registerHelper('add', (a: number, b: number) => a + b);
Handlebars.registerHelper('lookup', (ref: string, prefix: string) => {
    if (!ref) return '';
    // Handle OpenAPI refs like "#/components/schemas/User"
    if (ref.startsWith(prefix)) {
        return `schemas.${ref.slice(prefix.length)}`;
    }
    return ref;
});
Handlebars.registerHelper('routeFilename', (path: string) => {
    if (!path) return '';
    // Remove leading slash and convert path segments to filename
    const clean = path.replace(/^\//, '').replace(/\//g, '-').replace(/[^a-zA-Z0-9-]/g, '');
    return clean || 'index';
});

interface GenerateOptions {
    input: string;
    output: string;
    template?: string;
    framework?: 'worker' | 'hono';
}

export async function generate(options: GenerateOptions) {
    const spinner = ora('Generating API client').start();

    try {
        // Read OpenAPI spec
        const openApiContent = await readFile(options.input, 'utf-8');
        const openApiSpec = parseYaml(openApiContent) as OpenAPI.Document;

        // Ensure output directory exists
        await mkdir(options.output, { recursive: true });

        // Generate Zod schemas and client
        const zodClientResult = await generateZodClientFromOpenAPI({
            openApiDoc: openApiSpec as any,
            disableWriteToFile: true,
            options: {
                withAlias: true,
                defaultStatusBehavior: 'auto-correct',
            },
        });

        // Load templates
        const templateDir = options.template || join(process.cwd(), 'templates');
        const templateFiles = await glob('**/*.hbs', { cwd: templateDir });

        // Create src directory for Hono framework
        if (options.framework === 'hono') {
            await mkdir(join(options.output, 'src'), { recursive: true });
            await mkdir(join(options.output, 'src', 'routes'), { recursive: true });
        }

        for (const templateFile of templateFiles) {
            const templatePath = join(templateDir, templateFile);
            const templateContent = await readFile(templatePath, 'utf-8');
            const template = Handlebars.compile(templateContent);

            // Special handling for route template
            if (templateFile === 'route.hbs' && options.framework === 'hono') {
                // Generate a route file for each path
                for (const [path, pathItem] of Object.entries(openApiSpec.paths || {})) {
                    const routeData = {
                        path,
                        pathItem,
                        openApiSpec,
                        zodSchemas: zodClientResult,
                    };

                    const routeFileName = Handlebars.helpers.routeFilename(path);
                    const outputPath = join(options.output, 'src', 'routes', `${routeFileName}.ts`);

                    // Create route file
                    const rendered = template(routeData);
                    await writeFile(outputPath, rendered);
                }

                // Generate routes/index.ts to export all routes
                const routeIndexPath = join(options.output, 'src', 'routes', 'index.ts');
                const routeIndexContent = `import { Hono } from 'hono';
import type { Env } from '../types';

const router = new Hono<{ Bindings: Env }>();

${Object.keys(openApiSpec.paths || {}).map(path => {
                    const routeName = Handlebars.helpers.routeFilename(path);
                    return `import { ${routeName}Router } from './${routeName}';
router.route('${path}', ${routeName}Router);`;
                }).join('\n\n')}

export const ${Handlebars.helpers.basename(openApiSpec.info.title)}Router = router;
`;
                await writeFile(routeIndexPath, routeIndexContent);
                continue;
            }

            let outputPath = join(
                options.output,
                templateFile.replace('.hbs', '.ts')
            );

            // Handle Hono framework specific paths
            if (options.framework === 'hono') {
                switch (templateFile) {
                    case 'index.hbs':
                        outputPath = join(options.output, 'src', 'index.ts');
                        break;
                    case 'types.hbs':
                        outputPath = join(options.output, 'src', 'types.ts');
                        break;
                    case 'client.hbs':
                        outputPath = join(options.output, 'src', 'client.ts');
                        break;
                    default:
                        outputPath = join(options.output, templateFile.replace('.hbs', '.ts'));
                }
            }

            // Create output directory if it doesn't exist
            await mkdir(dirname(outputPath), { recursive: true });

            // Render template with generated code
            const rendered = template({
                zodSchemas: zodClientResult,
                openApiSpec,
            });

            await writeFile(outputPath, rendered);
        }

        spinner.succeed(chalk.green('API client generated successfully'));
    } catch (error) {
        spinner.fail(chalk.red('Failed to generate API client'));
        console.error(error);
        process.exit(1);
    }
} 