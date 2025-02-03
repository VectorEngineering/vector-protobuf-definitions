import { Hono } from 'hono';
import type { Env } from '../types';

const router = new Hono<{ Bindings: Env }>();

import { usersRouter } from './users';
router.route('/users', usersRouter);

export const SampleAPIRouter = router;
