import { initializeBrowser, shutdownBrowser } from './services/scraper';

import bodyParser from 'body-parser';
import cors from 'cors';
import dotenv from 'dotenv';
import express from 'express';
import { extractUrlsHandler } from './endpoints/extract-urls';
import { scrapeHandler } from './endpoints/scrape';
import { specs } from './swagger';
import swaggerUi from 'swagger-ui-express';

// Load environment variables
dotenv.config();

const app = express();
const port = process.env.PORT || 3003;

// Middleware
app.use(cors());
app.use(express.json({ limit: '50mb' }));
app.use(express.urlencoded({ extended: true }));
app.use(bodyParser.json());

// Swagger documentation
app.use('/api-docs', swaggerUi.serve, swaggerUi.setup(specs));

// Health check
app.get('/health', (req, res) => {
    res.json({ status: 'ok' });
});

// Routes
app.post('/scrape', scrapeHandler);
app.post('/extract-urls', extractUrlsHandler);

// Start server
const startServer = async () => {
    try {
        await initializeBrowser();
        app.listen(port, () => {
            console.log(`Server is running on port ${port}`);
            console.log(`API Documentation available at http://localhost:${port}/api-docs`);
        });
    } catch (error) {
        console.error('Failed to start server:', error);
        process.exit(1);
    }
};

// Graceful shutdown
process.on('SIGINT', () => {
    shutdownBrowser().then(() => {
        console.log('Browser closed');
        process.exit(0);
    });
});

export { app, startServer };