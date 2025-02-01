package org.openapitools.configuration;

import org.openapitools.model.APIKeyStatus;
import org.openapitools.model.AccountStatus;
import org.openapitools.model.AuthErrorCode;
import org.openapitools.model.BackgroundJobStatus;
import org.openapitools.model.DayOfWeek;
import org.openapitools.model.EmployeeBenefit;
import org.openapitools.model.InternalErrorCode;
import org.openapitools.model.NotFoundErrorCode;
import org.openapitools.model.OutputFormat;
import org.openapitools.model.RevenueRange;
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
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.dayOfWeekConverter")
    Converter<String, DayOfWeek> dayOfWeekConverter() {
        return new Converter<String, DayOfWeek>() {
            @Override
            public DayOfWeek convert(String source) {
                return DayOfWeek.fromValue(source);
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
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.internalErrorCodeConverter")
    Converter<String, InternalErrorCode> internalErrorCodeConverter() {
        return new Converter<String, InternalErrorCode>() {
            @Override
            public InternalErrorCode convert(String source) {
                return InternalErrorCode.fromValue(source);
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
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.outputFormatConverter")
    Converter<String, OutputFormat> outputFormatConverter() {
        return new Converter<String, OutputFormat>() {
            @Override
            public OutputFormat convert(String source) {
                return OutputFormat.fromValue(source);
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
