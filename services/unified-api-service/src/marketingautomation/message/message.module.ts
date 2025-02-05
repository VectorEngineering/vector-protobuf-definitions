import { IngestDataService } from '@@core/@core-services/unification/ingest-data.service';
import { MessageController } from './message.controller';
import { MessageService } from './services/message.service';
import { Module } from '@nestjs/common';
import { ServiceRegistry } from './services/registry.service';
import { SyncService } from './sync/sync.service';
import { WebhookService } from '@@core/@core-services/webhooks/panora-webhooks/webhook.service';

@Module({
  controllers: [MessageController],
  providers: [
    MessageService,
    SyncService,
    WebhookService,
    ServiceRegistry,
    IngestDataService,

    /* PROVIDERS SERVICES */
  ],
  exports: [SyncService],
})
export class MessageModule {}
