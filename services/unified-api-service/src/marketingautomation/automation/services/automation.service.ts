import {
  UnifiedMarketingautomationAutomationInput,
  UnifiedMarketingautomationAutomationOutput,
} from '../types/model.unified';

import { FieldMappingService } from '@@core/field-mapping/field-mapping.service';
import { Injectable } from '@nestjs/common';
import { LoggerService } from '@@core/@core-services/logger/logger.service';
import { PrismaService } from '@@core/@core-services/prisma/prisma.service';
import { ServiceRegistry } from './registry.service';
import { WebhookService } from '@@core/@core-services/webhooks/panora-webhooks/webhook.service';

@Injectable()
export class AutomationService {
  constructor(
    private prisma: PrismaService,
    private logger: LoggerService,
    private webhook: WebhookService,
    private fieldMappingService: FieldMappingService,
    private serviceRegistry: ServiceRegistry,
  ) {
    this.logger.setContext(AutomationService.name);
  }

  async addAutomation(
    unifiedAutomationData: UnifiedMarketingautomationAutomationInput,
    connectionId: string,
    projectId: string,
    integrationId: string,
    linkedUserId: string,
    remote_data?: boolean,
  ): Promise<UnifiedMarketingautomationAutomationOutput> {
    return;
  }

  async getAutomation(
    id_automationing_automation: string,
    linkedUserId: string,
    integrationId: string,
    connection_id: string,
    project_id: string,
    remote_data?: boolean,
  ): Promise<UnifiedMarketingautomationAutomationOutput> {
    return;
  }

  async getAutomations(
    connectionId: string,
    projectId: string,
    integrationId: string,
    linkedUserId: string,
    limit: number,
    remote_data?: boolean,
    cursor?: string,
  ): Promise<UnifiedMarketingautomationAutomationOutput[]> {
    return;
  }
}
