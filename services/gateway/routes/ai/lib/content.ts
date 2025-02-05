/**
 * Extract main content from HTML
 * Focuses on article content, main tags, and relevant text
 * @param html - Cleaned HTML content
 * @returns Main content HTML
 */
export function extractMainContent(html: string): string {
  // Try to find main content in priority order
  const mainContentSelectors = [
    "article",
    "main",
    '[role="main"]',
    ".main-content",
    "#main-content",
    ".content",
    "#content",
    "body",
  ];

  for (const selector of mainContentSelectors) {
    const match = html.match(
      new RegExp(`<${selector}[^>]*>(.*?)<\/${selector}>`, "is"),
    );
    if (match) return match[1];
  }

  return html;
}

/**
 * Extract content sections from HTML
 * @param html - HTML content
 * @returns Array of content sections with headings
 */
export function extractContentSections(
  html: string,
): Array<{ heading: string; content: string }> {
  const sections: Array<{ heading: string; content: string }> = [];
  const headingPattern =
    /<h[1-6][^>]*>(.*?)<\/h[1-6]>([\s\S]*?)(?=<h[1-6]|$)/gi;

  let match;
  while ((match = headingPattern.exec(html)) !== null) {
    const [_, heading, content] = match;
    sections.push({
      heading: heading.trim(),
      content: content.trim(),
    });
  }

  return sections;
}

/**
 * Extract lists from HTML
 * @param html - HTML content
 * @returns Object containing ordered and unordered lists
 */
export function extractLists(html: string): {
  ordered: string[][];
  unordered: string[][];
} {
  const lists = {
    ordered: [] as string[][],
    unordered: [] as string[][],
  };

  // Extract ordered lists
  const olPattern = /<ol[^>]*>([\s\S]*?)<\/ol>/gi;
  let olMatch;
  while ((olMatch = olPattern.exec(html)) !== null) {
    const items = olMatch[1].match(/<li[^>]*>([\s\S]*?)<\/li>/gi) || [];
    lists.ordered.push(
      items.map((item) => item.replace(/<[^>]+>/g, "").trim()),
    );
  }

  // Extract unordered lists
  const ulPattern = /<ul[^>]*>([\s\S]*?)<\/ul>/gi;
  let ulMatch;
  while ((ulMatch = ulPattern.exec(html)) !== null) {
    const items = ulMatch[1].match(/<li[^>]*>([\s\S]*?)<\/li>/gi) || [];
    lists.unordered.push(
      items.map((item) => item.replace(/<[^>]+>/g, "").trim()),
    );
  }

  return lists;
}

/**
 * Extract tables from HTML
 * @param html - HTML content
 * @returns Array of table data
 */
export function extractTables(
  html: string,
): Array<{ headers: string[]; rows: string[][] }> {
  const tables: Array<{ headers: string[]; rows: string[][] }> = [];
  const tablePattern = /<table[^>]*>([\s\S]*?)<\/table>/gi;

  let tableMatch;
  while ((tableMatch = tablePattern.exec(html)) !== null) {
    const table = { headers: [] as string[], rows: [] as string[][] };

    // Extract headers
    const headerMatch = tableMatch[1].match(/<th[^>]*>([\s\S]*?)<\/th>/gi);
    if (headerMatch) {
      table.headers = headerMatch.map((header) =>
        header.replace(/<[^>]+>/g, "").trim(),
      );
    }

    // Extract rows
    const rowPattern = /<tr[^>]*>([\s\S]*?)<\/tr>/gi;
    let rowMatch;
    while ((rowMatch = rowPattern.exec(tableMatch[1])) !== null) {
      const cells = rowMatch[1].match(/<td[^>]*>([\s\S]*?)<\/td>/gi) || [];
      table.rows.push(cells.map((cell) => cell.replace(/<[^>]+>/g, "").trim()));
    }

    tables.push(table);
  }

  return tables;
}
