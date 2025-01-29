package org.openapitools.configuration;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.info.Contact;
import io.swagger.v3.oas.models.info.License;
import io.swagger.v3.oas.models.Components;
import io.swagger.v3.oas.models.security.SecurityScheme;

@Configuration
public class SpringDocConfiguration {

    @Bean(name = "org.openapitools.configuration.SpringDocConfiguration.apiInfo")
    OpenAPI apiInfo() {
        return new OpenAPI()
                .info(
                        new Info()
                                .title("Lead Scraping Service API")
                                .description("Vector Lead Scraping Service API - Manages Lead Scraping Jobs")
                                .contact(
                                        new Contact()
                                                .name("Vector AI Engineering")
                                                .url("https://vector.ai")
                                                .email("yoanyomba@vector.ai")
                                )
                                .license(
                                        new License()
                                                .name("Apache 2.0 License")
                                                .url("https://github.com/SolomonAIEngineering/backend-monorepo/blob/main/LICENSE")
                                )
                                .version("1.0")
                )
                .components(
                        new Components()
                                .addSecuritySchemes("ApiKeyAuth", new SecurityScheme()
                                        .type(SecurityScheme.Type.APIKEY)
                                        .in(SecurityScheme.In.HEADER)
                                        .name("X-API-Key")
                                )
                )
        ;
    }
}