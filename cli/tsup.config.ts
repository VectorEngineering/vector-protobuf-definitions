import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs'],
  dts: true,
  clean: true,
  noExternal: ['commander', 'chalk', 'ora'],
  banner: {
    js: '#!/usr/bin/env node',
  },
}); 