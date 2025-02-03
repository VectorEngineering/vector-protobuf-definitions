import { ApiClient } from './client';
import type { ExecutionContext } from '@cloudflare/workers-types';

export interface Env {
  // Environment variables
  API_BASE_URL: string;
}

export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    const client = new ApiClient(env.API_BASE_URL);

    try {
      // Handle CORS
      if (request.method === 'OPTIONS') {
        return new Response(null, {
          headers: {
            'Access-Control-Allow-Origin': '*',
            'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
            'Access-Control-Allow-Headers': 'Content-Type, Authorization',
          },
        });
      }

      const url = new URL(request.url);
      const path = url.pathname;

      if (path === '/lead-scraper-microservice/api/v1/accounts') {

        if (request.method === 'POST') {
          const data = await request.json();
          const response = await client.createLeadScraperMicroserviceApiV1Accounts(data);
          return new Response(JSON.stringify(response), {
            status: 201,
            headers: { 'Content-Type': 'application/json' },
          });
        }

        if (request.method === 'PUT') {
          const data = await request.json();
          const response = await client.updateLeadScraperMicroserviceApiV1Accounts(data);
          return new Response(JSON.stringify(response), {
            headers: { 'Content-Type': 'application/json' },
          });
        }

      }
      if (path === '/lead-scraper-microservice/api/v1/accounts/{id}') {
        if (request.method === 'GET') {
          const params = {
            id: path.split('/')[1],
          };
          const response = await client.getLeadScraperMicroserviceApiV1AccountsId(params);
          return new Response(JSON.stringify(response), {
            headers: { 'Content-Type': 'application/json' },
          });
        }



        if (request.method === 'DELETE') {
          const params = {
            id: path.split('/')[1],
          };
          const response = await client.deleteLeadScraperMicroserviceApiV1AccountsId(params);
          return new Response(JSON.stringify(response), {
            headers: { 'Content-Type': 'application/json' },
          });
        }
      }
      if (path === '/lead-scraper-microservice/api/v1/jobs') {
        if (request.method === 'GET') {
          const params = {
            userId: url.searchParams.get('userId') || '',
            orgId: url.searchParams.get('orgId') || '',
            tenantId: url.searchParams.get('tenantId') || '',
          };
          const response = await client.getLeadScraperMicroserviceApiV1Jobs(params);
          return new Response(JSON.stringify(response), {
            headers: { 'Content-Type': 'application/json' },
          });
        }

        if (request.method === 'POST') {
          const data = await request.json();
          const response = await client.createLeadScraperMicroserviceApiV1Jobs(data);
          return new Response(JSON.stringify(response), {
            status: 201,
            headers: { 'Content-Type': 'application/json' },
          });
        }


      }
      if (path === '/lead-scraper-microservice/api/v1/jobs/{jobId}') {
        if (request.method === 'GET') {
          const params = {
            jobId: path.split('/')[1],
            userId: url.searchParams.get('userId') || '',
            orgId: url.searchParams.get('orgId') || '',
            tenantId: url.searchParams.get('tenantId') || '',
          };
          const response = await client.getLeadScraperMicroserviceApiV1JobsJobId(params);
          return new Response(JSON.stringify(response), {
            headers: { 'Content-Type': 'application/json' },
          });
        }



        if (request.method === 'DELETE') {
          const params = {
            jobId: path.split('/')[1],
            userId: url.searchParams.get('userId') || '',
            orgId: url.searchParams.get('orgId') || '',
            tenantId: url.searchParams.get('tenantId') || '',
          };
          const response = await client.deleteLeadScraperMicroserviceApiV1JobsJobId(params);
          return new Response(JSON.stringify(response), {
            headers: { 'Content-Type': 'application/json' },
          });
        }
      }
      if (path === '/lead-scraper-microservice/api/v1/jobs/{jobId}/download') {
        if (request.method === 'GET') {
          const params = {
            jobId: path.split('/')[1],
            userId: url.searchParams.get('userId') || '',
            orgId: url.searchParams.get('orgId') || '',
            tenantId: url.searchParams.get('tenantId') || '',
          };
          const response = await client.getLeadScraperMicroserviceApiV1JobsJobIdDownload(params);
          return new Response(JSON.stringify(response), {
            headers: { 'Content-Type': 'application/json' },
          });
        }



      }

      return new Response('Not Found', { status: 404 });
    } catch (error) {
      console.error('Error:', error);
      return new Response(
        JSON.stringify({ 
          error: error instanceof Error ? error.message : 'Internal Server Error' 
        }), {
          status: 500,
          headers: { 'Content-Type': 'application/json' },
        }
      );
    }
  },
}; 