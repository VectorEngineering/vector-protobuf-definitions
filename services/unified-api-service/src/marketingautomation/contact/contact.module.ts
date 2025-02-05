import { ContactController } from './contact.controller';
import { ContactService } from './services/contact.service';
import { CoreUnification } from '@@core/@core-services/unification/core-unification.service';
import { IngestDataService } from '@@core/@core-services/unification/ingest-data.service';
import { Module } from '@nestjs/common';
import { ServiceRegistry } from './services/registry.service';
import { SyncService } from './sync/sync.service';
import { WebhookService } from '@@core/@core-services/webhooks/panora-webhooks/webhook.service';

@Module({
  controllers: [ContactController],
  providers: [
    ContactService,

    CoreUnification,

    SyncService,
    WebhookService,

    ServiceRegistry,

    IngestDataService,
    /* PROVIDERS SERVICES */
  ],
  exports: [SyncService],
})
export class ContactModule {}
