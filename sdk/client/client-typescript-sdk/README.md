# Vector TypeScript Client SDK

[![npm package][npm-img]][npm-url]
[![Build Status][build-img]][build-url]
[![Downloads][downloads-img]][downloads-url]
[![Issues][issues-img]][issues-url]
[![Code Coverage][codecov-img]][codecov-url]
[![Semantic Release][semantic-release-img]][semantic-release-url]

> Official TypeScript SDK for Vector's Backend Services

## Overview

This package provides a modern, type-safe TypeScript client for interacting with Vector's backend services. Built with TypeScript and following best practices, it offers a seamless integration experience with comprehensive type definitions and Promise-based APIs.

## Features

- 🔒 **Type Safety**: Full TypeScript support with comprehensive type definitions
- 🚀 **Modern Architecture**: Promise-based API with async/await support
- 🛡️ **Error Handling**: Built-in error handling and retry mechanisms
- 📦 **Zero Dependencies**: Minimal production dependencies
- 🔄 **Automatic Retries**: Smart retry logic for failed requests
- 📚 **Rich Documentation**: Extensive documentation and examples
- ✨ **IDE Support**: Great IDE integration with TypeScript

## Installation

```bash
# Using npm
npm install @playbookmedia/backend-typescript-sdk

# Using yarn
yarn add @playbookmedia/backend-typescript-sdk

# Using pnpm
pnpm add @playbookmedia/backend-typescript-sdk
```

## Quick Start

```typescript
import { VectorClient } from '@playbookmedia/backend-typescript-sdk';

// Initialize the client
const client = new VectorClient({
  apiKey: 'your-api-key',
  environment: 'production' // or 'staging'
});

// Example: Using the client
async function example() {
  try {
    // Make API calls
    const response = await client.someEndpoint({
      // request parameters
    });
    
    console.log('Response:', response);
  } catch (error) {
    if (error instanceof VectorAPIError) {
      console.error('API Error:', error.message);
      console.error('Status:', error.status);
      console.error('Code:', error.code);
    } else {
      console.error('Unexpected error:', error);
    }
  }
}
```

## API Reference

### Client Configuration

```typescript
interface VectorClientConfig {
  apiKey: string;
  environment?: 'production' | 'staging';
  timeout?: number;
  retryConfig?: {
    maxRetries: number;
    initialDelayMs: number;
  };
}
```

### Available Methods

- `client.method1()`: Description of method 1
- `client.method2()`: Description of method 2
- More methods documented in our [API Reference](https://docs.vector.engineering/typescript)

## Development

### Prerequisites

- Node.js >= 12.0
- npm or yarn

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/VectorEngineering/vector-protobuf-definitions.git
   cd vector-protobuf-definitions/sdk/client/client-typescript-sdk
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

### Available Scripts

- `npm run build` - Builds the package using TypeScript
- `npm run clean` - Removes build artifacts
- `npm run test` - Runs tests with Jest
- `npm run lint` - Lints code using ESLint
- `npm run format` - Formats code using Prettier
- `npm run typecheck` - Runs TypeScript type checking
- `npm run semantic-release` - Handles semantic versioning and releases

### Testing

```bash
# Run all tests
npm test

# Run tests in watch mode
npm run test:watch

# Generate coverage report
npm run test:coverage
```

## Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create your feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `npm run cm` (uses Commitizen)
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## Support

- 📧 Email: yoanyomba@solomon-ai.co
- 📚 Documentation: [docs.vector.engineering](https://docs.vector.engineering)
- 🐛 Issue Tracker: [GitHub Issues](https://github.com/VectorEngineering/vector-protobuf-definitions/issues)

## License

MIT © [Vector Engineering]

[npm-img]: https://img.shields.io/npm/v/@playbookmedia/backend-typescript-sdk
[npm-url]: https://www.npmjs.com/package/@playbookmedia/backend-typescript-sdk
[build-img]: https://github.com/VectorEngineering/vector-protobuf-definitions/actions/workflows/typescript-sdk.yml/badge.svg
[build-url]: https://github.com/VectorEngineering/vector-protobuf-definitions/actions/workflows/typescript-sdk.yml
[downloads-img]: https://img.shields.io/npm/dt/@playbookmedia/backend-typescript-sdk
[downloads-url]: https://www.npmtrends.com/@playbookmedia/backend-typescript-sdk
[issues-img]: https://img.shields.io/github/issues/VectorEngineering/vector-protobuf-definitions
[issues-url]: https://github.com/VectorEngineering/vector-protobuf-definitions/issues
[codecov-img]: https://codecov.io/gh/VectorEngineering/vector-protobuf-definitions/branch/main/graph/badge.svg
[codecov-url]: https://codecov.io/gh/VectorEngineering/vector-protobuf-definitions
[semantic-release-img]: https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg
[semantic-release-url]: https://github.com/semantic-release/semantic-release
