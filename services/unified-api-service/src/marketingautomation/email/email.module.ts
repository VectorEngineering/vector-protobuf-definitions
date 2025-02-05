import { CoreUnification } from '@@core/@core-services/unification/core-unification.service';
import { EmailController } from './email.controller';
import { EmailService } from './services/email.service';
import { FieldMappingService } from '@@core/field-mapping/field-mapping.service';
import { IngestDataService } from '@@core/@core-services/unification/ingest-data.service';
import { Module } from '@nestjs/common';
import { ServiceRegistry } from './services/registry.service';
import { SyncService } from './sync/sync.service';
import { WebhookService } from '@@core/@core-services/webhooks/panora-webhooks/webhook.service';

@Module({
  controllers: [EmailController],
  providers: [
    EmailService,

    SyncService,
    WebhookService,

    FieldMappingService,
    ServiceRegistry,
    CoreUnification,

    IngestDataService,
    /* PROVIDERS SERVICES */
  ],
  exports: [SyncService],
})
export class EmailModule {}
