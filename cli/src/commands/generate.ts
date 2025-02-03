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
    // Remove leading slash and convert path segments to camelCase
    const clean = path.replace(/^\//, '')
        .split('/')
        .map(segment => segment.replace(/[^a-zA-Z0-9]/g, ' '))
        .map(segment => segment.trim())
        .map(segment => segment.split(/\s+/)
            .map((word, i) => i === 0 ? word.toLowerCase() : word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
            .join('')
        )
        .join('-');
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

        // Create src, routes, and middleware directories
        const srcDir = join(options.output, 'src');
        const routesDir = join(srcDir, 'routes');
        const middlewareDir = join(srcDir, 'middleware');
        await mkdir(srcDir, { recursive: true });
        await mkdir(routesDir, { recursive: true });
        await mkdir(middlewareDir, { recursive: true });

        // Copy middleware templates
        const rateLimitTemplate = await readFile(join(templateDir, 'middleware', 'rateLimit.hbs'), 'utf-8');
        await writeFile(join(middlewareDir, 'rateLimit.ts'), rateLimitTemplate);

        const authTemplate = await readFile(join(templateDir, 'middleware', 'auth.hbs'), 'utf-8');
        await writeFile(join(middlewareDir, 'auth.ts'), authTemplate);

        const middlewareIndexTemplate = await readFile(join(templateDir, 'middleware', 'index.hbs'), 'utf-8');
        await writeFile(join(middlewareDir, 'index.ts'), middlewareIndexTemplate);

        for (const templateFile of templateFiles) {
            const templatePath = join(templateDir, templateFile);
            const templateContent = await readFile(templatePath, 'utf-8');
            const template = Handlebars.compile(templateContent);

            if (templateFile === 'route.hbs') {
                // Generate individual route files
                for (const [path, pathItem] of Object.entries(openApiSpec.paths || {})) {
                    const routeFileName = Handlebars.helpers.routeFilename(path);
                    const routeFilePath = join(routesDir, `${routeFileName}.ts`);

                    const routeContent = template({
                        path,
                        pathItem,
                        openApiSpec,
                        zodSchemas: zodClientResult,
                    });

                    await writeFile(routeFilePath, routeContent);
                }

                // Generate routes index file
                const indexContent = `import { Hono } from 'hono';
import type { Env } from '../types';

const router = new Hono<{ Bindings: Env }>();

${Object.entries(openApiSpec.paths || {}).map(([path, _]) => {
                    const routeName = Handlebars.helpers.routeFilename(path);
                    return `import { ${routeName}Router } from './${routeName}';
router.route('${path}', ${routeName}Router);`;
                }).join('\n\n')}

export const ${Handlebars.helpers.basename(openApiSpec.info.title)}Router = router;
`;
                await writeFile(join(routesDir, 'index.ts'), indexContent);
            } else {
                // Handle other templates
                let outputPath = join(srcDir, templateFile.replace('.hbs', '.ts'));
                const rendered = template({
                    zodSchemas: zodClientResult,
                    openApiSpec,
                });

                await mkdir(dirname(outputPath), { recursive: true });
                await writeFile(outputPath, rendered);
            }
        }

        spinner.succeed(chalk.green('API client generated successfully'));
    } catch (error) {
        spinner.fail(chalk.red('Failed to generate API client'));
        console.error(error);
        process.exit(1);
    }
} 