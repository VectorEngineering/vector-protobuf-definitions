// Generated TypeScript API Client
import { type ZodiosInstance } from "@zodios/core";

import { makeApi, Zodios, type ZodiosOptions } from "@zodios/core";
import { z } from "zod";

const User = z
  .object({
    id: z.string().uuid(),
    name: z.string(),
    email: z.string().email(),
    createdAt: z.string().datetime({ offset: true }),
  })
  .passthrough();
const CreateUser = z
  .object({ name: z.string(), email: z.string().email() })
  .passthrough();

export const schemas = {
  User,
  CreateUser,
};

const endpoints = makeApi([
  {
    method: "get",
    path: "/users",
    alias: "getUsers",
    requestFormat: "json",
    response: z.array(User),
  },
  {
    method: "post",
    path: "/users",
    alias: "postUsers",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        type: "Body",
        schema: CreateUser,
      },
    ],
    response: User,
  },
]);

export const api = new Zodios(endpoints);

export function createApiClient(baseUrl: string, options?: ZodiosOptions) {
  return new Zodios(baseUrl, endpoints, options);
}


export class ApiClient {
  private readonly client: ZodiosInstance<typeof endpoints>;

  constructor(baseUrl: string, options?: ZodiosOptions) {
    this.client = new Zodios(baseUrl, endpoints, options);
  }

  /**
   * List users
   * 
   */
  async getUsers() {
    return this.client.get('/users');
  }

  /**
   * Create user
   * 
   */
  async createUsers(data: z.infer<typeof schemas.CreateUser>) {
    return this.client.post('/users', data);
  }


}

// Export types for public use
export type User = z.infer<typeof schemas.User>;
export type CreateUser = z.infer<typeof schemas.CreateUser>;
