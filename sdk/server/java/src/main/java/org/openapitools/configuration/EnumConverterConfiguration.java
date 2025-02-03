package org.openapitools.configuration;

import org.openapitools.model.APIKeyStatus;
import org.openapitools.model.AccountStatus;
import org.openapitools.model.AppCategory;
import org.openapitools.model.AuthErrorCode;
import org.openapitools.model.BackgroundJobStatus;
import org.openapitools.model.ComplianceLevel;
import org.openapitools.model.DayOfWeek;
import org.openapitools.model.DocumentStatus;
import org.openapitools.model.EmployeeBenefit;
import org.openapitools.model.IncludedField;
import org.openapitools.model.InternalErrorCode;
import org.openapitools.model.Language;
import org.openapitools.model.NotFoundErrorCode;
import org.openapitools.model.NullValue;
import org.openapitools.model.OutputFormat;
import org.openapitools.model.PayloadFormat;
import org.openapitools.model.Permission;
import org.openapitools.model.PricingModel;
import org.openapitools.model.RevenueRange;
import org.openapitools.model.Role;
import org.openapitools.model.SignatureStatus;
import org.openapitools.model.TemplateType;
import org.openapitools.model.Timezone;
import org.openapitools.model.TriggerEvent;
import org.openapitools.model.ValidationErrorCode;
import org.openapitools.model.WorkflowStatus;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.convert.converter.Converter;

@Configuration
public class EnumConverterConfiguration {

    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.apIKeyStatusConverter")
    Converter<String, APIKeyStatus> apIKeyStatusConverter() {
        return new Converter<String, APIKeyStatus>() {
            @Override
            public APIKeyStatus convert(String source) {
                return APIKeyStatus.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.accountStatusConverter")
    Converter<String, AccountStatus> accountStatusConverter() {
        return new Converter<String, AccountStatus>() {
            @Override
            public AccountStatus convert(String source) {
                return AccountStatus.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.appCategoryConverter")
    Converter<String, AppCategory> appCategoryConverter() {
        return new Converter<String, AppCategory>() {
            @Override
            public AppCategory convert(String source) {
                return AppCategory.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.authErrorCodeConverter")
    Converter<String, AuthErrorCode> authErrorCodeConverter() {
        return new Converter<String, AuthErrorCode>() {
            @Override
            public AuthErrorCode convert(String source) {
                return AuthErrorCode.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.backgroundJobStatusConverter")
    Converter<String, BackgroundJobStatus> backgroundJobStatusConverter() {
        return new Converter<String, BackgroundJobStatus>() {
            @Override
            public BackgroundJobStatus convert(String source) {
                return BackgroundJobStatus.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.complianceLevelConverter")
    Converter<String, ComplianceLevel> complianceLevelConverter() {
        return new Converter<String, ComplianceLevel>() {
            @Override
            public ComplianceLevel convert(String source) {
                return ComplianceLevel.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.dayOfWeekConverter")
    Converter<String, DayOfWeek> dayOfWeekConverter() {
        return new Converter<String, DayOfWeek>() {
            @Override
            public DayOfWeek convert(String source) {
                return DayOfWeek.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.documentStatusConverter")
    Converter<String, DocumentStatus> documentStatusConverter() {
        return new Converter<String, DocumentStatus>() {
            @Override
            public DocumentStatus convert(String source) {
                return DocumentStatus.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.employeeBenefitConverter")
    Converter<String, EmployeeBenefit> employeeBenefitConverter() {
        return new Converter<String, EmployeeBenefit>() {
            @Override
            public EmployeeBenefit convert(String source) {
                return EmployeeBenefit.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.includedFieldConverter")
    Converter<String, IncludedField> includedFieldConverter() {
        return new Converter<String, IncludedField>() {
            @Override
            public IncludedField convert(String source) {
                return IncludedField.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.internalErrorCodeConverter")
    Converter<String, InternalErrorCode> internalErrorCodeConverter() {
        return new Converter<String, InternalErrorCode>() {
            @Override
            public InternalErrorCode convert(String source) {
                return InternalErrorCode.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.languageConverter")
    Converter<String, Language> languageConverter() {
        return new Converter<String, Language>() {
            @Override
            public Language convert(String source) {
                return Language.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.notFoundErrorCodeConverter")
    Converter<String, NotFoundErrorCode> notFoundErrorCodeConverter() {
        return new Converter<String, NotFoundErrorCode>() {
            @Override
            public NotFoundErrorCode convert(String source) {
                return NotFoundErrorCode.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.nullValueConverter")
    Converter<String, NullValue> nullValueConverter() {
        return new Converter<String, NullValue>() {
            @Override
            public NullValue convert(String source) {
                return NullValue.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.outputFormatConverter")
    Converter<String, OutputFormat> outputFormatConverter() {
        return new Converter<String, OutputFormat>() {
            @Override
            public OutputFormat convert(String source) {
                return OutputFormat.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.payloadFormatConverter")
    Converter<String, PayloadFormat> payloadFormatConverter() {
        return new Converter<String, PayloadFormat>() {
            @Override
            public PayloadFormat convert(String source) {
                return PayloadFormat.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.permissionConverter")
    Converter<String, Permission> permissionConverter() {
        return new Converter<String, Permission>() {
            @Override
            public Permission convert(String source) {
                return Permission.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.pricingModelConverter")
    Converter<String, PricingModel> pricingModelConverter() {
        return new Converter<String, PricingModel>() {
            @Override
            public PricingModel convert(String source) {
                return PricingModel.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.revenueRangeConverter")
    Converter<String, RevenueRange> revenueRangeConverter() {
        return new Converter<String, RevenueRange>() {
            @Override
            public RevenueRange convert(String source) {
                return RevenueRange.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.roleConverter")
    Converter<String, Role> roleConverter() {
        return new Converter<String, Role>() {
            @Override
            public Role convert(String source) {
                return Role.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.signatureStatusConverter")
    Converter<String, SignatureStatus> signatureStatusConverter() {
        return new Converter<String, SignatureStatus>() {
            @Override
            public SignatureStatus convert(String source) {
                return SignatureStatus.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.templateTypeConverter")
    Converter<String, TemplateType> templateTypeConverter() {
        return new Converter<String, TemplateType>() {
            @Override
            public TemplateType convert(String source) {
                return TemplateType.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.timezoneConverter")
    Converter<String, Timezone> timezoneConverter() {
        return new Converter<String, Timezone>() {
            @Override
            public Timezone convert(String source) {
                return Timezone.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.triggerEventConverter")
    Converter<String, TriggerEvent> triggerEventConverter() {
        return new Converter<String, TriggerEvent>() {
            @Override
            public TriggerEvent convert(String source) {
                return TriggerEvent.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.validationErrorCodeConverter")
    Converter<String, ValidationErrorCode> validationErrorCodeConverter() {
        return new Converter<String, ValidationErrorCode>() {
            @Override
            public ValidationErrorCode convert(String source) {
                return ValidationErrorCode.fromValue(source);
            }
        };
    }
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.workflowStatusConverter")
    Converter<String, WorkflowStatus> workflowStatusConverter() {
        return new Converter<String, WorkflowStatus>() {
            @Override
            public WorkflowStatus convert(String source) {
                return WorkflowStatus.fromValue(source);
            }
        };
    }

}
