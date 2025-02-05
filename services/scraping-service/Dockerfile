FROM mcr.microsoft.com/playwright:v1.41.2-focal

WORKDIR /app

# Create node user and group
RUN groupadd -r node && useradd -r -g node -G audio,video node \
    && mkdir -p /home/node \
    && chown -R node:node /home/node

# Create app directory and set permissions
RUN mkdir -p /app/node_modules && chown -R node:node /app

# Install pnpm
RUN npm install -g pnpm

# Install nodemon globally
RUN npm install -g nodemon

# Switch to non-root user
USER node

# Copy package files with correct ownership
COPY --chown=node:node package*.json pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy source code with correct ownership
COPY --chown=node:node . .

# Build TypeScript
RUN pnpm run build

# Install only Chromium browser
RUN npx playwright install chromium

# Add this environment variable to ensure Node.js uses CommonJS
ENV NODE_OPTIONS="--no-warnings"

EXPOSE 3003

CMD ["nodemon", "dist/api.js"]