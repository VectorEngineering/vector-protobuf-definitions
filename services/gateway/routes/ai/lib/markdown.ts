/**
 * Transform HTML to markdown with advanced formatting
 * @param html - Cleaned HTML content
 * @returns Markdown string
 */
export function htmlToMarkdown(html: string): string {
    let markdown = html;

    // Replace headers with proper level
    for (let i = 6; i >= 1; i--) {
        const headerPattern = new RegExp(`<h${i}[^>]*>(.*?)<\/h${i}>`, 'gi');
        markdown = markdown.replace(headerPattern, (_, content) =>
            `\n${'#'.repeat(i)} ${content.trim()}\n`
        );
    }

    // Replace emphasis and strong
    markdown = markdown
        .replace(/<strong[^>]*>(.*?)<\/strong>/gi, '**$1**')
        .replace(/<b[^>]*>(.*?)<\/b>/gi, '**$1**')
        .replace(/<em[^>]*>(.*?)<\/em>/gi, '*$1*')
        .replace(/<i[^>]*>(.*?)<\/i>/gi, '*$1*');

    // Replace links with titles
    markdown = markdown.replace(
        /<a[^>]*href="([^"]*)"[^>]*(?:title="([^"]*)")?[^>]*>(.*?)<\/a>/gi,
        (_, url, title, text) => `[${text.trim()}](${url})${title ? ` "${title}"` : ''}`
    );

    // Replace images with alt text and titles
    markdown = markdown.replace(
        /<img[^>]*src="([^"]*)"[^>]*(?:alt="([^"]*)")?[^>]*(?:title="([^"]*)")?[^>]*>/gi,
        (_, src, alt, title) => `![${alt || ''}](${src})${title ? ` "${title}"` : ''}`
    );

    // Replace lists
    markdown = markdown
        .replace(/<ul[^>]*>([\s\S]*?)<\/ul>/gi, (_, content) =>
            `\n${content.trim()}\n`
        )
        .replace(/<ol[^>]*>([\s\S]*?)<\/ol>/gi, (_, content) => {
            let counter = 1;
            return `\n${content.replace(/<li[^>]*>(.*?)<\/li>/gi, (_: string, item: string) =>
                `${counter++}. ${item.trim()}\n`
            )}\n`;
        })
        .replace(/<li[^>]*>(.*?)<\/li>/gi, (_, content) =>
            `- ${content.trim()}\n`
        );

    // Replace blockquotes
    markdown = markdown.replace(
        /<blockquote[^>]*>([\s\S]*?)<\/blockquote>/gi,
        (_, content) => `\n> ${content.trim().replace(/\n/g, '\n> ')}\n`
    );

    // Replace code blocks
    markdown = markdown
        .replace(/<pre[^>]*><code[^>]*>([\s\S]*?)<\/code><\/pre>/gi, (_, content) =>
            `\n\`\`\`\n${content.trim()}\n\`\`\`\n`
        )
        .replace(/<code[^>]*>(.*?)<\/code>/gi, '`$1`');

    // Replace horizontal rules
    markdown = markdown.replace(/<hr[^>]*>/gi, '\n---\n');

    // Replace paragraphs
    markdown = markdown.replace(/<p[^>]*>(.*?)<\/p>/gi, (_, content) =>
        `\n${content.trim()}\n`
    );

    // Replace tables
    markdown = markdown.replace(
        /<table[^>]*>([\s\S]*?)<\/table>/gi,
        (_, tableContent) => {
            let result = '\n';

            // Process headers
            const headerMatch = tableContent.match(/<th[^>]*>([\s\S]*?)<\/th>/gi);
            if (headerMatch) {
                result += '| ' + headerMatch.map((header: string) =>
                    header.replace(/<[^>]+>/g, '').trim()
                ).join(' | ') + ' |\n';
                result += '|' + headerMatch.map(() => '---').join('|') + '|\n';
            }

            // Process rows
            const rowPattern = /<tr[^>]*>([\s\S]*?)<\/tr>/gi;
            let rowMatch;
            while ((rowMatch = rowPattern.exec(tableContent)) !== null) {
                const cells = rowMatch[1].match(/<td[^>]*>([\s\S]*?)<\/td>/gi) || [];
                if (cells.length > 0) {
                    result += '| ' + cells.map(cell =>
                        cell.replace(/<[^>]+>/g, '').trim()
                    ).join(' | ') + ' |\n';
                }
            }

            return result;
        }
    );

    // Remove remaining HTML tags
    markdown = markdown.replace(/<[^>]+>/g, '');

    // Clean up whitespace
    markdown = markdown
        .replace(/\n\s*\n/g, '\n\n')
        .replace(/^\s+|\s+$/g, '')
        .replace(/\n{3,}/g, '\n\n');

    return markdown;
}

/**
 * Convert a table object to markdown format
 * @param table - Table data object
 * @returns Markdown table string
 */
export function tableToMarkdown(table: { headers: string[]; rows: string[][] }): string {
    if (!table.headers.length) return '';

    let markdown = '\n| ' + table.headers.join(' | ') + ' |\n';
    markdown += '|' + table.headers.map(() => '---').join('|') + '|\n';

    table.rows.forEach(row => {
        markdown += '| ' + row.join(' | ') + ' |\n';
    });

    return markdown;
}

/**
 * Convert a list to markdown format
 * @param items - List items
 * @param ordered - Whether the list should be ordered
 * @returns Markdown list string
 */
export function listToMarkdown(items: string[], ordered = false): string {
    return items.map((item, index) =>
        ordered ? `${index + 1}. ${item}` : `- ${item}`
    ).join('\n');
} 