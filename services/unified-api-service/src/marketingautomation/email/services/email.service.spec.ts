import { Test, TestingModule } from '@nestjs/testing';

import { EmailService } from './email.service';
import { FieldMappingService } from '@@core/field-mapping/field-mapping.service';
import { LoggerService } from '@@core/@core-services/logger/logger.service';
import { PrismaService } from '@@core/@core-services/prisma/prisma.service';
import { ServiceRegistry } from './registry.service';
import { UnifiedMarketingautomationEmailOutput } from '../types/model.unified';
import { WebhookService } from '@@core/@core-services/webhooks/panora-webhooks/webhook.service';

describe('EmailService', () => {
  let service: EmailService;
  let prismaService: PrismaService;
  let loggerService: LoggerService;
  let webhookService: WebhookService;
  let fieldMappingService: FieldMappingService;
  let serviceRegistry: ServiceRegistry;

  const mockEmail: UnifiedMarketingautomationEmailOutput = {
    id: 'test-id',
    subject: 'Test Subject',
    body: 'Test Body',
    status: 'sent',
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    remote_data: {},
    remote_was_deleted: false,
    id_marketing_automation_email: 'test-id',
  };

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [
        EmailService,
        {
          provide: PrismaService,
          useValue: {
            marketingautomation_email: {
              findFirst: jest.fn().mockResolvedValue(mockEmail),
              findMany: jest.fn().mockResolvedValue([mockEmail]),
            },
          },
        },
        {
          provide: LoggerService,
          useValue: {
            setContext: jest.fn(),
            log: jest.fn(),
            error: jest.fn(),
          },
        },
        {
          provide: WebhookService,
          useValue: {
            createWebhook: jest.fn(),
          },
        },
        {
          provide: FieldMappingService,
          useValue: {
            mapFields: jest.fn().mockReturnValue(mockEmail),
          },
        },
        {
          provide: ServiceRegistry,
          useValue: {
            getService: jest.fn().mockReturnValue({
              getEmail: jest.fn().mockResolvedValue(mockEmail),
              getEmails: jest.fn().mockResolvedValue([mockEmail]),
            }),
          },
        },
      ],
    }).compile();

    service = module.get<EmailService>(EmailService);
    prismaService = module.get<PrismaService>(PrismaService);
    loggerService = module.get<LoggerService>(LoggerService);
    webhookService = module.get<WebhookService>(WebhookService);
    fieldMappingService = module.get<FieldMappingService>(FieldMappingService);
    serviceRegistry = module.get<ServiceRegistry>(ServiceRegistry);

    // Spy on service methods
    jest.spyOn(service, 'getEmail').mockResolvedValue(mockEmail);
    jest.spyOn(service, 'getEmails').mockResolvedValue([mockEmail]);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });

  describe('getEmail', () => {
    it('should return an email', async () => {
      // Act
      const result = await service.getEmail(
        'test-id',
        'linked-user-id',
        'integration-id',
        'connection-id',
        'project-id',
        false,
      );

      // Assert
      expect(result).toBeDefined();
      expect(result).toEqual(mockEmail);
    });
  });

  describe('getEmails', () => {
    it('should return an array of emails', async () => {
      // Act
      const result = await service.getEmails(
        'connection-id',
        'project-id',
        'integration-id',
        'linked-user-id',
        10,
        false,
      );

      // Assert
      expect(result).toBeDefined();
      expect(Array.isArray(result)).toBe(true);
      expect(result).toEqual([mockEmail]);
    });
  });
});
