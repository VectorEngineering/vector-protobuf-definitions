# Vector Backend Node SDK

[![npm package][npm-img]][npm-url]
[![Build Status][build-img]][build-url]
[![Downloads][downloads-img]][downloads-url]
[![Issues][issues-img]][issues-url]
[![Code Coverage][codecov-img]][codecov-url]
[![Semantic Release][semantic-release-img]][semantic-release-url]

> Official Node.js SDK for interacting with the Vector API

## Overview

This package provides a TypeScript/JavaScript client SDK for interacting with Vector's backend services. It's part of the Vector protobuf definitions repository which contains API definitions, SDKs, and related tooling for Vector's microservices architecture.

## Features

- Full TypeScript support with comprehensive type definitions
- Promise-based API
- Automatic request/response handling
- Built-in error handling and retry logic
- Semantic versioning and releases
- Comprehensive test coverage
- Modern ES6+ codebase

## Installation

```bash
npm install @playbookmedia/backend-node-sdk
```

Or with yarn:

```bash
yarn add @playbookmedia/backend-node-sdk
```

## Requirements

- Node.js >= 12.0
- npm or yarn package manager

## Quick Start

```typescript
import { VectorClient } from '@playbookmedia/backend-node-sdk';

// Initialize the client
const client = new VectorClient({
  // Add your configuration options here
});

// Example usage
async function example() {
  try {
    const response = await client.someEndpoint();
    console.log(response);
  } catch (error) {
    console.error('Error:', error);
  }
}
```

## Development

### Setup

1. Clone the repository
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

### Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes using conventional commits (`npm run cm`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Project Structure

```
├── src/           # Source code
├── lib/           # Compiled JavaScript
├── tests/         # Test files
└── types/         # TypeScript type definitions
```

## Documentation

For detailed API documentation and examples, please visit our [documentation site](https://docs.vector.engineering).

## License

MIT © [Vector Engineering]

[build-img]: https://github.com/VectorEngineering/Vector-Lead-Management/actions/workflows/release.yml/badge.svg
[build-url]: https://github.com/VectorEngineering/Vector-Lead-Management/actions/workflows/release.yml
[downloads-img]: https://img.shields.io/npm/dt/@playbookmedia/backend-node-sdk
[downloads-url]: https://www.npmtrends.com/@playbookmedia/backend-node-sdk
[npm-img]: https://img.shields.io/npm/v/@playbookmedia/backend-node-sdk
[npm-url]: https://www.npmjs.com/package/@playbookmedia/backend-node-sdk
[issues-img]: https://img.shields.io/github/issues/VectorEngineering/Vector-Lead-Management
[issues-url]: https://github.com/VectorEngineering/Vector-Lead-Management/issues
[codecov-img]: https://codecov.io/gh/VectorEngineering/Vector-Lead-Management/branch/main/graph/badge.svg
[codecov-url]: https://codecov.io/gh/VectorEngineering/Vector-Lead-Management
[semantic-release-img]: https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg
[semantic-release-url]: https://github.com/semantic-release/semantic-release
