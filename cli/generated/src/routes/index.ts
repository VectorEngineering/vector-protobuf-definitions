import { Hono } from 'hono';
import type { Env } from '../types';

const router = new Hono<{ Bindings: Env }>();

import { leadScraperMicroserviceapiv1accountsRouter } from './leadScraperMicroserviceapiv1accounts';
router.route('/lead-scraper-microservice/api/v1/accounts', leadScraperMicroserviceapiv1accountsRouter);

import { leadScraperMicroserviceapiv1accountsidRouter } from './leadScraperMicroserviceapiv1accountsid';
router.route('/lead-scraper-microservice/api/v1/accounts/:id', leadScraperMicroserviceapiv1accountsidRouter);

import { leadScraperMicroserviceapiv1jobsRouter } from './leadScraperMicroserviceapiv1jobs';
router.route('/lead-scraper-microservice/api/v1/jobs', leadScraperMicroserviceapiv1jobsRouter);

import { leadScraperMicroserviceapiv1jobsjobidRouter } from './leadScraperMicroserviceapiv1jobsjobid';
router.route('/lead-scraper-microservice/api/v1/jobs/:jobId', leadScraperMicroserviceapiv1jobsjobidRouter);

import { leadScraperMicroserviceapiv1jobsjobiddownloadRouter } from './leadScraperMicroserviceapiv1jobsjobiddownload';
router.route('/lead-scraper-microservice/api/v1/jobs/:jobId/download', leadScraperMicroserviceapiv1jobsjobiddownloadRouter);

export const LeadScrapingServiceAPIRouter = router;
