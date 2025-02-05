import { chromium } from 'playwright';
import dotenv from 'dotenv';

dotenv.config({ path: '.env.local' });

async function testProxy() {
    console.log('Testing proxy connection...');

    const proxyUrl = process.env.PROXY_SERVER?.startsWith('http')
        ? process.env.PROXY_SERVER
        : `http://${process.env.PROXY_SERVER}`;

    console.log('Proxy configuration:', {
        server: proxyUrl,
        username: process.env.PROXY_USERNAME?.substring(0, 10) + '...',
        hasPassword: !!process.env.PROXY_PASSWORD
    });

    const browser = await chromium.launch({
        headless: true,
        args: ['--no-sandbox']
    });

    try {
        const context = await browser.newContext({
            proxy: {
                server: proxyUrl!,
                username: process.env.PROXY_USERNAME!,
                password: process.env.PROXY_PASSWORD!
            }
        });

        console.log('Browser context created, testing connection...');
        const page = await context.newPage();

        console.log('Attempting to access a test URL...');
        const response = await page.goto('http://example.com');

        if (response) {
            console.log('Connection successful!');
            console.log('Status:', response.status());
            console.log('Headers:', response.headers());
        } else {
            console.log('No response received');
        }
    } catch (error) {
        console.error('Error during proxy test:', error);
    } finally {
        await browser.close();
    }
}

testProxy().catch(console.error); 