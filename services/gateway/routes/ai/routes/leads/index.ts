import { OpenAPIHono as Hono, createRoute } from "@hono/zod-openapi";

import { Env } from "../../../../types";
import hours from "./hours";
import intelligence from "./intelligence";
import qualify from "./qualify";

/**
 * Lead analysis routes
 * Provides endpoints for analyzing and qualifying business leads
 */
const leadsRouter = new Hono<{ Bindings: Env }>();

// Mount lead analysis routes
leadsRouter.route("/hours", hours);
leadsRouter.route("/intelligence", intelligence);
leadsRouter.route("/qualify", qualify);

export default leadsRouter;
