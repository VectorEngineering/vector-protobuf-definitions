import { Request } from 'express';
import { UrlModel } from './scraping';

export interface ScrapeRequest extends Request {
    body: UrlModel;
} 