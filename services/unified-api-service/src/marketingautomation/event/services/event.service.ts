import { FieldMappingService } from '@@core/field-mapping/field-mapping.service';
import { Injectable } from '@nestjs/common';
import { LoggerService } from '@@core/@core-services/logger/logger.service';
import { PrismaService } from '@@core/@core-services/prisma/prisma.service';
import { ServiceRegistry } from './registry.service';
import { UnifiedMarketingautomationEventOutput } from '../types/model.unified';
import { WebhookService } from '@@core/@core-services/webhooks/panora-webhooks/webhook.service';

@Injectable()
export class EventService {
  constructor(
    private prisma: PrismaService,
    private logger: LoggerService,
    private webhook: WebhookService,
    private fieldMappingService: FieldMappingService,
    private serviceRegistry: ServiceRegistry,
  ) {
    this.logger.setContext(EventService.name);
  }

  async getEvent(
    id_eventing_event: string,
    linkedUserId: string,
    integrationId: string,
    connection_id: string,
    project_id: string,
    remote_data?: boolean,
  ): Promise<UnifiedMarketingautomationEventOutput> {
    return;
  }

  async getEvents(
    connectionId: string,
    projectId: string,
    integrationId: string,
    linkedUserId: string,
    limit: number,
    remote_data?: boolean,
    cursor?: string,
  ): Promise<UnifiedMarketingautomationEventOutput[]> {
    return;
  }
}
