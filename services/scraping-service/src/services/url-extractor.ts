import { Browser, BrowserContext, Page } from 'playwright';

import { initializeBrowser } from './scraper';

interface ExtractedUrl {
    url: string;
    isRelative: boolean;
    domain: string;
}

interface PageContext {
    includeRelative: boolean;
    filterDomain?: string;
}

/**
 * Extracts all URLs from a given page
 * @param url The URL to scrape
 * @param options Options for URL extraction
 * @returns Array of extracted URLs with metadata
 */
export const extractUrls = async (
    url: string,
    options: {
        includeRelative?: boolean;
        filterDomain?: string;
        timeout?: number;
    } = {}
): Promise<ExtractedUrl[]> => {
    console.log('Initializing browser for URL extraction...');
    const { browser, context } = await initializeBrowser();
    const page = await context.newPage();

    try {
        console.log(`Attempting to navigate to ${url}...`);
        const response = await page.goto(url, {
            waitUntil: 'networkidle',
            timeout: options.timeout || 30000
        });

        if (!response) {
            throw new Error('Failed to get response from page');
        }

        const status = response.status();
        console.log(`Page response status: ${status}`);

        if (status >= 400) {
            throw new Error(`Page returned error status: ${status}`);
        }

        console.log('Page loaded successfully, extracting URLs...');
        const extractedUrls = await page.evaluate(({ includeRelative, filterDomain }: PageContext) => {
            const urls: Array<{ url: string; isRelative: boolean; domain: string }> = [];
            const seenUrls = new Set<string>();
            const baseUrl = location.href;

            // Helper function to process URLs
            const processUrl = (urlString: string) => {
                try {
                    const isRelative = !urlString.startsWith('http://') && !urlString.startsWith('https://');
                    const fullUrl = new URL(urlString, baseUrl);

                    if (isRelative && !includeRelative) return null;
                    if (filterDomain && !fullUrl.hostname.includes(filterDomain)) return null;
                    if (seenUrls.has(fullUrl.href)) return null;

                    seenUrls.add(fullUrl.href);
                    return {
                        url: fullUrl.href,
                        isRelative,
                        domain: fullUrl.hostname,
                    };
                } catch {
                    return null;
                }
            };

            // Process all elements with URLs
            ['a[href]', 'img[src]', 'script[src]', 'link[href]'].forEach(selector => {
                document.querySelectorAll(selector).forEach((element: Element) => {
                    const urlAttr = selector.includes('href') ? 'href' : 'src';
                    const urlString = element.getAttribute(urlAttr);
                    if (urlString) {
                        const url = processUrl(urlString);
                        if (url) urls.push(url);
                    }
                });
            });

            return urls;
        }, { includeRelative: options.includeRelative || false, filterDomain: options.filterDomain });

        return extractedUrls;
    } finally {
        await page.close();
    }
}; 