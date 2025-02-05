import { Request, Response } from 'express';

import { ScrapeRequest } from '../types/express';
import { getError } from '../helpers/errors';
import { isValidUrl } from '../utils/validation';
import { scrapePage } from '../services/scraper';
import { validateUrlModel } from '../types/scraping';

/**
 * @openapi
 * components:
 *   schemas:
 *     UrlModel:
 *       type: object
 *       required:
 *         - url
 *       properties:
 *         url:
 *           type: string
 *           description: The URL to scrape
 *         wait_after_load:
 *           type: number
 *           description: Time to wait after page load in milliseconds
 *         timeout:
 *           type: number
 *           description: Maximum time to wait for page load
 *         headers:
 *           type: object
 *           description: Custom headers to send with request
 *         check_selector:
 *           type: string
 *           description: CSS selector to wait for
 */

/**
 * @openapi
 * /scrape:
 *   post:
 *     summary: Scrape a webpage
 *     description: Fetches content from a specified URL using Playwright
 *     requestBody:
 *       required: true
 *       content:
 *         application/json:
 *           schema:
 *             $ref: '#/components/schemas/UrlModel'
 *     responses:
 *       200:
 *         description: Successfully scraped webpage
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 content:
 *                   type: string
 *                   description: The scraped HTML content
 *                 pageStatusCode:
 *                   type: number
 *                   description: HTTP status code
 *                 pageError:
 *                   type: string
 *                   description: Error message if any
 *       400:
 *         description: Invalid request parameters
 *       500:
 *         description: Server error
 */
export const scrapeHandler = async (req: ScrapeRequest, res: Response) => {
    try {
        const validatedBody = validateUrlModel(req.body);
        const { url, wait_after_load = 0, timeout = 30000, headers, check_selector } = validatedBody;

        console.log(`================= Scrape Request =================`);
        console.log(`URL: ${url}`);
        console.log(`Wait After Load: ${wait_after_load}`);
        console.log(`Timeout: ${timeout}`);
        console.log(`Headers: ${headers ? JSON.stringify(headers) : 'None'}`);
        console.log(`Check Selector: ${check_selector ? check_selector : 'None'}`);
        console.log(`==================================================`);

        if (!isValidUrl(url)) {
            return res.status(400).json({ error: 'Invalid URL' });
        }

        if (!process.env.PROXY_SERVER) {
            console.warn('⚠️ WARNING: No proxy server provided. Your IP address may be blocked.');
        }

        let pageContent;
        let pageStatusCode: number | null = null;
        try {
            // Strategy 1: Normal
            console.log('Attempting strategy 1: Normal load');
            const result = await scrapePage(url, {
                waitAfterLoad: wait_after_load,
                timeout: timeout,
                headers,
                checkSelector: check_selector
            });
            pageContent = result;
            pageStatusCode = 200;
        } catch (error) {
            console.log('Strategy 1 failed, attempting strategy 2: Wait until networkidle');
            try {
                // Strategy 2: Wait until networkidle
                const result = await scrapePage(url, {
                    waitAfterLoad: wait_after_load,
                    timeout: timeout,
                    headers,
                    checkSelector: check_selector
                });
                pageContent = result;
                pageStatusCode = 200;
            } catch (finalError) {
                return res.status(500).json({ error: 'An error occurred while fetching the page.' });
            }
        }

        const pageError = pageStatusCode !== 200 ? getError(pageStatusCode) : undefined;

        if (!pageError) {
            console.log(`✅ Scrape successful!`);
        } else {
            console.log(`🚨 Scrape failed with status code: ${pageStatusCode} ${pageError}`);
        }

        return res.json({
            content: pageContent,
            pageStatusCode,
            ...(pageError && { pageError }),
        });
    } catch (error) {
        if (error instanceof Error) {
            return res.status(400).json({ error: error.message });
        }
        return res.status(400).json({ error: 'Invalid request body' });
    }
}; 