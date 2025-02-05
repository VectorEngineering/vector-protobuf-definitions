# run directly from the repo root directory
# docker build -f ./packages/api/Dockerfile .
FROM node:20-alpine3.21 AS base

# Install dependencies
RUN apk add --no-cache libc6-compat openssl git
RUN apk update

# Install pnpm with npm instead of using corepack
RUN npm install -g pnpm@8.15.1

WORKDIR /app

# Copy package files
COPY package.json pnpm-lock.yaml ./

# Install dependencies with no frozen lockfile to allow for updates
RUN pnpm install

# Copy source files
COPY . .

# Build the application
RUN pnpm run build

# Expose port
EXPOSE 3000

# Start the application
CMD ["node", "dist/src/main.js"]

# # # # # # # # # # # # 
# Production: Injects env vars via Doppler - DOPPLER_TOKEN variable must be set
# Install Doppler CLI
# RUN wget -q -t3 'https://packages.doppler.com/public/cli/rsa.8004D9FF50437357.key' -O /etc/apk/keys/cli@doppler-8004D9FF50437357.rsa.pub && \
#     echo 'https://packages.doppler.com/public/cli/alpine/any-version/main' | tee -a /etc/apk/repositories && \
#     apk add doppler
# CMD doppler run --command="node dist/src/main.js"