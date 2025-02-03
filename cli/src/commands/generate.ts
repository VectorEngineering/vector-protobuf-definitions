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
    // Remove leading slash and split by slashes
    const parts = path.replace(/^\//, '').split('/');
    // Get the last meaningful part
    return parts[parts.length - 1];
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

interface GenerateOptions {
    input: string;
    output: string;
    template?: string;
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

        for (const templateFile of templateFiles) {
            const templatePath = join(templateDir, templateFile);
            const templateContent = await readFile(templatePath, 'utf-8');
            const template = Handlebars.compile(templateContent);

            const outputPath = join(
                options.output,
                templateFile.replace('.hbs', '.ts')
            );

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