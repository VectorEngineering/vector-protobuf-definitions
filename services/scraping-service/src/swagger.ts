import swaggerJsdoc from 'swagger-jsdoc';

const options = {
  definition: {
    openapi: '3.0.0',
    info: {
      title: 'Scraping Service API',
      version: '1.0.0',
      description: 'API documentation for the web scraping service',
    },
    servers: [
      {
        url: `http://localhost:${process.env.PORT || 3003}`,
        description: 'Development server',
      },
    ],
  },
  apis: ['./src/endpoints/*.ts'], // Path to the API docs
};

export const specs = swaggerJsdoc(options); 