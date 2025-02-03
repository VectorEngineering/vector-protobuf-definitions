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

      if (path === '/users') {
        if (request.method === 'GET') {
          const response = await client.getUsers();
          return new Response(JSON.stringify(response), {
            headers: { 'Content-Type': 'application/json' },
          });
        }

        if (request.method === 'POST') {
          const data = await request.json();
          const response = await client.createUsers(data);
          return new Response(JSON.stringify(response), {
            status: 201,
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