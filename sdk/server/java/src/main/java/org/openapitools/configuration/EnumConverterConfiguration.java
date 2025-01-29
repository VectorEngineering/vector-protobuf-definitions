package org.openapitools.configuration;

import org.openapitools.model.AuthErrorCode;
import org.openapitools.model.BackgroundJobStatus;
import org.openapitools.model.InternalErrorCode;
import org.openapitools.model.NotFoundErrorCode;
import org.openapitools.model.ValidationErrorCode;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.convert.converter.Converter;

@Configuration
public class EnumConverterConfiguration {

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
    @Bean(name = "org.openapitools.configuration.EnumConverterConfiguration.validationErrorCodeConverter")
    Converter<String, ValidationErrorCode> validationErrorCodeConverter() {
        return new Converter<String, ValidationErrorCode>() {
            @Override
            public ValidationErrorCode convert(String source) {
                return ValidationErrorCode.fromValue(source);
            }
        };
    }

}
