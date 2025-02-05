import { EventController } from './event.controller';
import { EventService } from './services/event.service';
import { IngestDataService } from '@@core/@core-services/unification/ingest-data.service';
import { Module } from '@nestjs/common';
import { ServiceRegistry } from './services/registry.service';
import { SyncService } from './sync/sync.service';
import { WebhookService } from '@@core/@core-services/webhooks/panora-webhooks/webhook.service';

@Module({
  controllers: [EventController],
  providers: [
    EventService,

    SyncService,
    WebhookService,

    ServiceRegistry,

    IngestDataService,
    /* PROVIDERS SERVICES */
  ],
  exports: [SyncService],
})
export class EventModule {}
