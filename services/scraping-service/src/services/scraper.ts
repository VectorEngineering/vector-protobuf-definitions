import { Browser, BrowserContext, Request as PlaywrightRequest, Route, chromium } from 'playwright';

import { ScrapeOptions } from '../types/scraping';
import UserAgent from 'user-agents';

let browser: Browser;
let context: BrowserContext;

const DEFAULT_TIMEOUT = 30000;
const DEFAULT_WAIT_AFTER_LOAD = 1000;

const AD_SERVING_DOMAINS = [
    'doubleclick.net',
    'adservice.google.com',
    'googlesyndication.com',
    'googletagservices.com',
    'googletagmanager.com',
    'google-analytics.com',
    'adsystem.com',
    'adservice.com',
    'adnxs.com',
    'ads-twitter.com',
    'facebook.net',
    'fbcdn.net',
    'amazon-adsystem.com',
];

export const initializeBrowser = async (): Promise<{ browser: Browser; context: BrowserContext }> => {
    try {
        if (!browser) {
            console.log('Launching browser with configuration...');
            browser = await chromium.launch({
                headless: true,
                args: [
                    '--no-sandbox',
                    '--disable-setuid-sandbox',
                    '--disable-dev-shm-usage',
                    '--disable-accelerated-2d-canvas',
                    '--no-first-run',
                    '--no-zygote',
                    '--single-process',
                    '--disable-gpu',
                    '--ignore-certificate-errors',
                    '--ignore-certificate-errors-spki-list',
                    '--disable-web-security',
                    '--proxy-bypass-list=<-loopback>',  // Don't use proxy for localhost
                ],
            });
        }

        if (!context) {
            const userAgent = new UserAgent().toString();
            const viewport = { width: 1280, height: 800 };

            const contextOptions: any = {
                userAgent,
                viewport,
                ignoreHTTPSErrors: true,
                bypassCSP: true,
            };

            // Only use proxy if explicitly set
            if (process.env.PROXY_SERVER && process.env.PROXY_USERNAME && process.env.PROXY_PASSWORD) {
                console.log('Configuring authenticated proxy server:', process.env.PROXY_SERVER);
                const proxyUrl = process.env.PROXY_SERVER.startsWith('http')
                    ? process.env.PROXY_SERVER
                    : `http://${process.env.PROXY_SERVER}`;

                contextOptions.proxy = {
                    server: proxyUrl,
                    username: process.env.PROXY_USERNAME,
                    password: process.env.PROXY_PASSWORD,
                    bypass: '<-loopback>'  // Don't use proxy for localhost
                };
                console.log('Proxy configuration:', {
                    server: proxyUrl,
                    username: process.env.PROXY_USERNAME?.substring(0, 10) + '...',
                    hasPassword: !!process.env.PROXY_PASSWORD
                });
            } else if (process.env.PROXY_SERVER) {
                console.log('Using unauthenticated proxy server');
                const proxyUrl = process.env.PROXY_SERVER.startsWith('http')
                    ? process.env.PROXY_SERVER
                    : `http://${process.env.PROXY_SERVER}`;
                contextOptions.proxy = {
                    server: proxyUrl,
                };
            } else {
                console.log('No proxy configuration found, connecting directly');
            }

            try {
                console.log('Creating new browser context...');
                context = await browser.newContext(contextOptions);
                console.log('Browser context created successfully');

                // Set default navigation timeout
                context.setDefaultNavigationTimeout(60000);

                // Add error handler for all pages
                context.on('page', page => {
                    page.on('pageerror', error => {
                        console.error('Page error:', error);
                    });
                    page.on('console', msg => {
                        if (msg.type() === 'error' || msg.type() === 'warning') {
                            console.log(`Browser ${msg.type()}: ${msg.text()}`);
                        }
                    });
                });

            } catch (contextError) {
                console.error('Failed to create browser context:', contextError);
                throw contextError;
            }

            if (process.env.BLOCK_MEDIA === 'true') {
                await context.route(
                    '**/*.{png,jpg,jpeg,gif,svg,mp3,mp4,avi,flac,ogg,wav,webm}',
                    async (route: Route, request: PlaywrightRequest) => {
                        await route.abort();
                    }
                );
            }

            // Intercept all requests to avoid loading ads
            await context.route('**/*', (route: Route, request: PlaywrightRequest) => {
                const requestUrl = new URL(request.url());
                const hostname = requestUrl.hostname;

                if (AD_SERVING_DOMAINS.some((domain) => hostname.includes(domain))) {
                    console.log(hostname);
                    return route.abort();
                }
                return route.continue();
            });
        }

        return { browser, context };
    } catch (error: unknown) {
        console.error('Failed to initialize browser:', error);
        if (error instanceof Error && error.message.includes('Executable doesn\'t exist')) {
            console.error('\nPlaywright browsers are not installed. Try running:\n' +
                'npx playwright install chromium\n' +
                'or update your Docker image to match Playwright version.');
        }
        throw error;
    }
};

export const shutdownBrowser = async () => {
    if (context) {
        await context.close();
    }
    if (browser) {
        await browser.close();
    }
};

export const scrapePage = async (url: string, options: ScrapeOptions = {}) => {
    if (!browser || !context) {
        await initializeBrowser();
    }

    const page = await context.newPage();

    try {
        // Set headers if provided
        if (options.headers) {
            await page.setExtraHTTPHeaders(options.headers);
        }

        // Configure timeout
        const timeout = options.timeout || DEFAULT_TIMEOUT;
        page.setDefaultTimeout(timeout);

        // Navigate with better error handling
        try {
            await page.goto(url, {
                waitUntil: 'networkidle',
                timeout: timeout
            });

            // Wait additional time if specified
            const waitAfterLoad = options.waitAfterLoad || DEFAULT_WAIT_AFTER_LOAD;
            if (waitAfterLoad > 0) {
                await page.waitForTimeout(waitAfterLoad);
            }

            // If a selector is specified, wait for it
            if (options.checkSelector) {
                await page.waitForSelector(options.checkSelector, { timeout });
            }

            const content = await page.content();
            return content;

        } catch (navigationError) {
            console.error('Navigation failed:', navigationError);
            throw new Error(`Failed to load page: ${navigationError instanceof Error ? navigationError.message : 'Unknown error'}`);
        }

    } catch (error) {
        console.error('Scraping error:', error);
        throw new Error(`An error occurred while scraping: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
        await page.close();
    }
}; 