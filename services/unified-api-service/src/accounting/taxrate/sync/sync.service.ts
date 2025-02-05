import { IBaseSync, SyncLinkedUserType } from '@@core/utils/types/interface';
import { Injectable, OnModuleInit } from '@nestjs/common';

import { ACCOUNTING_PROVIDERS } from '@@core/shared/connectors/index';
import { acc_tax_rates as AccTaxRate } from '@prisma/client';
import { ApiResponse } from '@@core/utils/types';
import { CoreSyncRegistry } from '@@core/@core-services/registries/core-sync.registry';
import { CoreUnification } from '@@core/@core-services/unification/core-unification.service';
import { Cron } from '@nestjs/schedule';
import { FieldMappingService } from '@@core/field-mapping/field-mapping.service';
import { ITaxRateService } from '../types';
import { IngestDataService } from '@@core/@core-services/unification/ingest-data.service';
import { LoggerService } from '@@core/@core-services/logger/logger.service';
import { OriginalTaxRateOutput } from '@@core/utils/types/original/original.accounting';
import { PrismaService } from '@@core/@core-services/prisma/prisma.service';
import { ServiceRegistry } from '../services/registry.service';
import { UnifiedAccountingTaxrateOutput } from '../types/model.unified';
import { WebhookService } from '@@core/@core-services/webhooks/panora-webhooks/webhook.service';
import { v4 as uuidv4 } from 'uuid';

@Injectable()
export class SyncService implements OnModuleInit, IBaseSync {
  constructor(
    private prisma: PrismaService,
    private logger: LoggerService,
    private webhook: WebhookService,
    private fieldMappingService: FieldMappingService,
    private serviceRegistry: ServiceRegistry,
    private coreUnification: CoreUnification,
    private registry: CoreSyncRegistry,
    private ingestService: IngestDataService,
  ) {
    this.logger.setContext(SyncService.name);
    this.registry.registerService('accounting', 'taxrate', this);
  }
  onModuleInit() {
    //
  }

  @Cron('0 */8 * * *') // every 8 hours
  async kickstartSync(id_project?: string) {
    try {
      const linkedUsers = await this.prisma.linked_users.findMany({
        where: {
          id_project: id_project,
        },
      });
      linkedUsers.map(async (linkedUser) => {
        try {
          const providers = ACCOUNTING_PROVIDERS;
          for (const provider of providers) {
            try {
              await this.syncForLinkedUser({
                integrationId: provider,
                linkedUserId: linkedUser.id_linked_user,
              });
            } catch (error) {
              throw error;
            }
          }
        } catch (error) {
          throw error;
        }
      });
    } catch (error) {
      throw error;
    }
  }

  async syncForLinkedUser(param: SyncLinkedUserType) {
    try {
      const { integrationId, linkedUserId } = param;
      const service: ITaxRateService =
        this.serviceRegistry.getService(integrationId);
      if (!service) return;

      await this.ingestService.syncForLinkedUser<
        UnifiedAccountingTaxrateOutput,
        OriginalTaxRateOutput,
        ITaxRateService
      >(integrationId, linkedUserId, 'accounting', 'tax_rate', service, []);
    } catch (error) {
      throw error;
    }
  }

  async saveToDb(
    connection_id: string,
    linkedUserId: string,
    taxRates: UnifiedAccountingTaxrateOutput[],
    originSource: string,
    remote_data: Record<string, any>[],
  ): Promise<AccTaxRate[]> {
    try {
      const taxRateResults: AccTaxRate[] = [];

      for (let i = 0; i < taxRates.length; i++) {
        const taxRate = taxRates[i];
        const originId = taxRate.remote_id;

        let existingTaxRate = await this.prisma.acc_tax_rates.findFirst({
          where: {
            remote_id: originId,
            id_connection: connection_id,
          },
        });

        const taxRateData = {
          description: taxRate.description,
          total_tax_ratge: taxRate.total_tax_ratge
            ? Number(taxRate.total_tax_ratge)
            : null,
          effective_tax_rate: taxRate.effective_tax_rate
            ? Number(taxRate.effective_tax_rate)
            : null,
          company: taxRate.company_id,
          remote_id: originId,
          modified_at: new Date(),
        };

        if (existingTaxRate) {
          existingTaxRate = await this.prisma.acc_tax_rates.update({
            where: { id_acc_tax_rate: existingTaxRate.id_acc_tax_rate },
            data: taxRateData,
          });
        } else {
          existingTaxRate = await this.prisma.acc_tax_rates.create({
            data: {
              ...taxRateData,
              id_acc_tax_rate: uuidv4(),
              created_at: new Date(),
              id_connection: connection_id,
            },
          });
        }

        taxRateResults.push(existingTaxRate);

        // Process field mappings
        await this.ingestService.processFieldMappings(
          taxRate.field_mappings,
          existingTaxRate.id_acc_tax_rate,
          originSource,
          linkedUserId,
        );

        // Process remote data
        await this.ingestService.processRemoteData(
          existingTaxRate.id_acc_tax_rate,
          remote_data[i],
        );
      }

      return taxRateResults;
    } catch (error) {
      throw error;
    }
  }
}
