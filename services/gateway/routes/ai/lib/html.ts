/**
 * Utility functions for HTML processing and transformation
 */

/**
 * Clean and normalize HTML content for LLM processing
 * @param html - Raw HTML content
 * @returns Cleaned HTML string
 */
export function cleanHtml(html: string): string {
  // Remove script and style tags
  const noScripts = html.replace(
    /<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi,
    "",
  );
  const noStyles = noScripts.replace(
    /<style\b[^<]*(?:(?!<\/style>)<[^<]*)*<\/style>/gi,
    "",
  );

  // Remove HTML comments
  const noComments = noStyles.replace(/<!--[\s\S]*?-->/g, "");

  // Remove extra whitespace
  return noComments.replace(/\s+/g, " ").trim();
}

/**
 * Sanitize HTML by removing potentially dangerous elements and attributes
 * @param html - Raw HTML content
 * @returns Sanitized HTML string
 */
export function sanitizeHtml(html: string): string {
  // List of allowed tags
  const allowedTags = [
    "p",
    "div",
    "span",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "ul",
    "ol",
    "li",
    "a",
    "img",
    "blockquote",
    "em",
    "strong",
    "b",
    "i",
    "code",
    "pre",
    "br",
    "hr",
    "table",
    "tr",
    "td",
    "th",
  ];

  // List of allowed attributes
  const allowedAttrs = ["href", "src", "alt", "title", "class"];

  let sanitized = html;

  // Remove all event handlers
  sanitized = sanitized.replace(/\son\w+="[^"]*"/gi, "");

  // Remove javascript: and data: URLs
  sanitized = sanitized.replace(/(?:javascript|data):/gi, "invalid:");

  // Remove disallowed tags
  const tagPattern = /<\/?([a-z0-9]+)(?: [^>]*)?>/gi;
  sanitized = sanitized.replace(tagPattern, (match, tag) => {
    return allowedTags.includes(tag.toLowerCase()) ? match : "";
  });

  // Clean attributes
  const attrPattern = /(<[^>]+) [^>]*>/gi;
  sanitized = sanitized.replace(attrPattern, (match, prefix) => {
    const attrs =
      match.slice(prefix.length, -1).match(/([a-z-]+)(?:="[^"]*")?/gi) || [];
    const cleanAttrs = attrs
      .filter((attr) => allowedAttrs.includes(attr.split("=")[0].toLowerCase()))
      .join(" ");
    return prefix + (cleanAttrs ? " " + cleanAttrs : "") + ">";
  });

  return sanitized;
}

/**
 * Extract plain text content from HTML
 * @param html - HTML content
 * @returns Plain text string
 */
export function extractText(html: string): string {
  // Remove all HTML tags
  const noTags = html.replace(/<[^>]+>/g, " ");

  // Decode HTML entities
  const decoded = noTags.replace(/&[^;]+;/g, (match) => {
    const entities: Record<string, string> = {
      "&nbsp;": " ",
      "&amp;": "&",
      "&lt;": "<",
      "&gt;": ">",
      "&quot;": '"',
      "&apos;": "'",
    };
    return entities[match] || match;
  });

  // Clean up whitespace
  return decoded.replace(/\s+/g, " ").trim();
}

/**
 * Extract metadata from HTML including Open Graph data
 * @param html - Raw HTML content
 * @returns Object containing metadata
 */
export function extractMetadata(
  html: string,
): Record<string, string | string[]> {
  const metadata: Record<string, string | string[]> = {};

  // Extract basic metadata
  const basicMetaTags = {
    title: /<title[^>]*>([^<]+)<\/title>/i,
    description: /<meta[^>]*name="description"[^>]*content="([^"]*)"[^>]*>/i,
    keywords: /<meta[^>]*name="keywords"[^>]*content="([^"]*)"[^>]*>/i,
    author: /<meta[^>]*name="author"[^>]*content="([^"]*)"[^>]*>/i,
    date: /<meta[^>]*(?:name|property)="(?:date|published_time)"[^>]*content="([^"]*)"[^>]*>/i,
  };

  Object.entries(basicMetaTags).forEach(([key, pattern]) => {
    const match = html.match(pattern);
    if (match) metadata[key] = match[1].trim();
  });

  // Extract Open Graph metadata
  const ogPattern =
    /<meta[^>]*property="og:([^"]+)"[^>]*content="([^"]*)"[^>]*>/gi;
  let ogMatch;
  while ((ogMatch = ogPattern.exec(html)) !== null) {
    const [_, property, content] = ogMatch;
    metadata[`og:${property}`] = content.trim();
  }

  // Extract Twitter Card metadata
  const twitterPattern =
    /<meta[^>]*name="twitter:([^"]+)"[^>]*content="([^"]*)"[^>]*>/gi;
  let twitterMatch;
  while ((twitterMatch = twitterPattern.exec(html)) !== null) {
    const [_, property, content] = twitterMatch;
    metadata[`twitter:${property}`] = content.trim();
  }

  return metadata;
}

/**
 * Analyze HTML structure and create a hierarchical representation
 * @param html - HTML content
 * @returns Object representing the HTML structure
 */
export function analyzeStructure(html: string): object {
  const structure: any = { type: "root", children: [] };
  const stack = [structure];

  // Simple HTML parser
  const tagPattern = /<\/?([a-z0-9]+)(?: ([^>]*))?>/gi;
  let lastIndex = 0;
  let match;

  while ((match = tagPattern.exec(html)) !== null) {
    const [fullMatch, tag, attrs = ""] = match;
    const isClosing = fullMatch.startsWith("</");

    // Handle text nodes
    if (match.index > lastIndex) {
      const text = html.slice(lastIndex, match.index).trim();
      if (text) {
        stack[stack.length - 1].children.push({
          type: "text",
          content: text,
        });
      }
    }

    if (!isClosing) {
      // Opening tag
      const node = {
        type: "element",
        tag: tag.toLowerCase(),
        attributes: parseAttributes(attrs),
        children: [],
      };
      stack[stack.length - 1].children.push(node);
      stack.push(node);
    } else {
      // Closing tag
      if (stack[stack.length - 1].tag === tag.toLowerCase()) {
        stack.pop();
      }
    }

    lastIndex = tagPattern.lastIndex;
  }

  return structure.children;
}

/**
 * Parse HTML attributes string into an object
 * @param attrs - HTML attributes string
 * @returns Object containing attribute key-value pairs
 */
function parseAttributes(attrs: string): Record<string, string> {
  const attributes: Record<string, string> = {};
  const pattern = /([a-z-]+)(?:="([^"]*)")?/gi;
  let match;

  while ((match = pattern.exec(attrs)) !== null) {
    const [_, name, value = ""] = match;
    attributes[name.toLowerCase()] = value;
  }

  return attributes;
}

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
 * Transform HTML to basic markdown structure
 * @param html - Cleaned HTML content
 * @returns Markdown string
 */
export function htmlToMarkdown(html: string): string {
  let markdown = html;

  // Replace headers
  markdown = markdown.replace(
    /<h[1-6][^>]*>(.*?)<\/h[1-6]>/gi,
    (_, content) => `\n# ${content.trim()}\n`,
  );

  // Replace paragraphs
  markdown = markdown.replace(
    /<p[^>]*>(.*?)<\/p>/gi,
    (_, content) => `\n${content.trim()}\n`,
  );

  // Replace links
  markdown = markdown.replace(
    /<a[^>]*href="([^"]*)"[^>]*>(.*?)<\/a>/gi,
    (_, url, text) => `[${text.trim()}](${url})`,
  );

  // Replace lists
  markdown = markdown.replace(
    /<li[^>]*>(.*?)<\/li>/gi,
    (_, content) => `- ${content.trim()}\n`,
  );

  // Remove remaining HTML tags
  markdown = markdown.replace(/<[^>]+>/g, "");

  // Clean up whitespace
  markdown = markdown.replace(/\n\s*\n/g, "\n\n").trim();

  return markdown;
}
