import { Global, Module } from '@nestjs/common';

import { BullQueueModule } from './queues/queue.module';
import { CategoryConnectionRegistry } from './registries/connections-categories.registry';
import { ConnectionUtils } from '../connections/@utils/index';
import { CoreSyncRegistry } from './registries/core-sync.registry';
import { CoreUnification } from './unification/core-unification.service';
import { EncryptionService } from './encryption/encryption.service';
import { FieldMappingService } from '../field-mapping/field-mapping.service';
import { LoggerService } from './logger/logger.service';
import { MappersRegistry } from './registries/mappers.registry';
import { PrismaService } from './prisma/prisma.service';
import { RetryModule } from './request-retry/module';
import { UnificationRegistry } from './registries/unification.registry';

@Global()
@Module({
  imports: [BullQueueModule, RetryModule],
  providers: [
    PrismaService,
    MappersRegistry,
    UnificationRegistry,
    CoreSyncRegistry,
    EncryptionService,
    CategoryConnectionRegistry,
    CoreUnification,
    LoggerService,
    ConnectionUtils,
    FieldMappingService,
  ],
  exports: [
    PrismaService,
    MappersRegistry,
    UnificationRegistry,
    CoreSyncRegistry,
    EncryptionService,
    CategoryConnectionRegistry,
    CoreUnification,
    LoggerService,
    ConnectionUtils,
    FieldMappingService,
    BullQueueModule,
    RetryModule,
  ],
})
export class CoreSharedModule {}
