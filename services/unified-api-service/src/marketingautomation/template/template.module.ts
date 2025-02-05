import { CoreUnification } from '@@core/@core-services/unification/core-unification.service';
import { FieldMappingService } from '@@core/field-mapping/field-mapping.service';
import { IngestDataService } from '@@core/@core-services/unification/ingest-data.service';
import { Module } from '@nestjs/common';
import { ServiceRegistry } from './services/registry.service';
import { SyncService } from './sync/sync.service';
import { TemplateController } from './template.controller';
import { TemplateService } from './services/template.service';
import { WebhookService } from '@@core/@core-services/webhooks/panora-webhooks/webhook.service';

@Module({
  controllers: [TemplateController],
  providers: [
    TemplateService,

    SyncService,
    WebhookService,
    CoreUnification,

    FieldMappingService,
    ServiceRegistry,

    IngestDataService,
    /* PROVIDERS SERVICES */
  ],
  exports: [SyncService],
})
export class TemplateModule {}
