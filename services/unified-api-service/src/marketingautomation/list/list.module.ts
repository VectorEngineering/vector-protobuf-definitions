import { CoreUnification } from '@@core/@core-services/unification/core-unification.service';
import { FieldMappingService } from '@@core/field-mapping/field-mapping.service';
import { IngestDataService } from '@@core/@core-services/unification/ingest-data.service';
import { ListController } from './list.controller';
import { ListService } from './services/list.service';
import { Module } from '@nestjs/common';
import { ServiceRegistry } from './services/registry.service';
import { SyncService } from './sync/sync.service';
import { WebhookService } from '@@core/@core-services/webhooks/panora-webhooks/webhook.service';

@Module({
  controllers: [ListController],
  providers: [
    ListService,

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
export class ListModule {}
