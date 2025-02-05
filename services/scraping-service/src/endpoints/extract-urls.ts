import { Request, Response } from 'express';

import { extractUrls } from '../services/url-extractor';
import { isValidUrl } from '../utils/validation';
import { validateUrlExtractionRequest } from '../types/scraping';

/**
 * @openapi
 * /extract-urls:
 *   post:
 *     tags:
 *       - URL Extraction
 *     summary: Extract URLs from a webpage
 *     description: Scrapes a webpage and extracts all URLs (links, images, scripts, etc.)
 *     requestBody:
 *       required: true
 *       content:
 *         application/json:
 *           schema:
 *             type: object
 *             required:
 *               - url
 *             properties:
 *               url:
 *                 type: string
 *                 description: The URL to scrape
 *                 example: "https://example.com"
 *               includeRelative:
 *                 type: boolean
 *                 description: Whether to include relative URLs
 *                 default: false
 *                 example: true
 *               filterDomain:
 *                 type: string
 *                 description: Only include URLs from this domain
 *                 example: "example.com"
 *               timeout:
 *                 type: number
 *                 description: Maximum time to wait for page load (in milliseconds)
 *                 default: 30000
 *                 example: 30000
 *     responses:
 *       200:
 *         description: Successfully extracted URLs
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 urls:
 *                   type: array
 *                   items:
 *                     type: object
 *                     properties:
 *                       url:
 *                         type: string
 *                         description: The extracted URL
 *                         example: "https://example.com/about"
 *                       isRelative:
 *                         type: boolean
 *                         description: Whether this was a relative URL
 *                         example: false
 *                       domain:
 *                         type: string
 *                         description: The domain of the URL
 *                         example: "example.com"
 *                 totalCount:
 *                   type: number
 *                   description: Total number of URLs found
 *                   example: 42
 *                 pageUrl:
 *                   type: string
 *                   description: The URL that was scraped
 *                   example: "https://example.com"
 *       400:
 *         description: Invalid request parameters
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 error:
 *                   type: string
 *                   example: "Invalid URL format"
 *       500:
 *         description: Server error
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 error:
 *                   type: string
 *                   example: "An error occurred while extracting URLs"
 */
export const extractUrlsHandler = async (req: Request, res: Response) => {
    try {
        const validatedBody = validateUrlExtractionRequest(req.body);
        const { url, includeRelative = false, filterDomain, timeout = 30000 } = validatedBody;

        console.log(`================= URL Extraction Request =================`);
        console.log(`URL: ${url}`);
        console.log(`Include Relative: ${includeRelative}`);
        console.log(`Filter Domain: ${filterDomain || 'None'}`);
        console.log(`Timeout: ${timeout}`);
        console.log(`======================================================`);

        if (!isValidUrl(url)) {
            return res.status(400).json({ error: 'Invalid URL' });
        }

        const extractedUrls = await extractUrls(url, {
            includeRelative,
            filterDomain,
            timeout,
        });

        console.log(`✅ Successfully extracted ${extractedUrls.length} URLs`);

        return res.json({
            urls: extractedUrls,
            totalCount: extractedUrls.length,
            pageUrl: url,
        });
    } catch (error) {
        console.error('Error extracting URLs:', error);
        if (error instanceof Error) {
            return res.status(400).json({ error: error.message });
        }
        return res.status(500).json({ error: 'An error occurred while extracting URLs' });
    }
}; 