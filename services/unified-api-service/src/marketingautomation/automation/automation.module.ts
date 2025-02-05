import { AutomationController } from './automation.controller';
import { AutomationService } from './services/automation.service';
import { CoreUnification } from '@@core/@core-services/unification/core-unification.service';
import { IngestDataService } from '@@core/@core-services/unification/ingest-data.service';
import { Module } from '@nestjs/common';
import { ServiceRegistry } from './services/registry.service';
import { SyncService } from './sync/sync.service';
import { WebhookService } from '@@core/@core-services/webhooks/panora-webhooks/webhook.service';

@Module({
  controllers: [AutomationController],
  providers: [
    AutomationService,

    SyncService,
    WebhookService,

    ServiceRegistry,

    IngestDataService,

    CoreUnification,

    /* PROVIDERS SERVICES */
  ],
  exports: [SyncService],
})
export class AutomationModule {}
