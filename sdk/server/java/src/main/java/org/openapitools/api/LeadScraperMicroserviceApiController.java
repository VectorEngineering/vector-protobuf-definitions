package org.openapitools.api;

import org.openapitools.model.AuthenticationErrorMessageResponse;
import org.openapitools.model.BadGatewayErrorMessageResponse;
import org.openapitools.model.ConflictErrorMessageResponse;
import org.openapitools.model.CreateAccountRequest;
import org.openapitools.model.CreateAccountResponse;
import org.openapitools.model.CreateScrapingJobRequest;
import org.openapitools.model.CreateScrapingJobResponse;
import org.openapitools.model.DeleteAccountResponse;
import org.openapitools.model.DeleteScrapingJobResponse;
import org.openapitools.model.DownloadScrapingResultsResponse;
import org.openapitools.model.ForbiddenErrorMessageResponse;
import org.openapitools.model.GatewayTimeoutErrorMessageResponse;
import org.openapitools.model.GetAccountResponse;
import org.openapitools.model.GetScrapingJobResponse;
import org.openapitools.model.GoneErrorMessageResponse;
import org.openapitools.model.InternalErrorMessageResponse;
import org.openapitools.model.ListAccountsResponse;
import org.openapitools.model.ListScrapingJobsResponse;
import org.openapitools.model.MethodNotAllowedErrorMessageResponse;
import org.openapitools.model.NotFoundErrorMessageResponse;
import org.openapitools.model.NotImplementedErrorMessageResponse;
import org.openapitools.model.PaymentRequiredErrorMessageResponse;
import org.openapitools.model.PreconditionFailedErrorMessageResponse;
import org.openapitools.model.RateLimitErrorMessageResponse;
import org.openapitools.model.ServiceUnavailableErrorMessageResponse;
import org.openapitools.model.Status;
import org.openapitools.model.TooEarlyErrorMessageResponse;
import org.openapitools.model.UnprocessableEntityErrorMessageResponse;
import org.openapitools.model.UpdateAccountRequest;
import org.openapitools.model.UpdateAccountResponse;
import org.openapitools.model.ValidationErrorMessageResponse;


import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestHeader;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.CookieValue;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RequestPart;
import org.springframework.web.multipart.MultipartFile;
import org.springframework.web.context.request.NativeWebRequest;

import javax.validation.constraints.*;
import javax.validation.Valid;

import java.util.List;
import java.util.Map;
import java.util.Optional;
import javax.annotation.Generated;

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-01-29T05:05:43.494624-05:00[America/New_York]", comments = "Generator version: 7.7.0")
@Controller
@RequestMapping("${openapi.leadScrapingService.base-path:}")
public class LeadScraperMicroserviceApiController implements LeadScraperMicroserviceApi {

    private final NativeWebRequest request;

    @Autowired
    public LeadScraperMicroserviceApiController(NativeWebRequest request) {
        this.request = request;
    }

    @Override
    public Optional<NativeWebRequest> getRequest() {
        return Optional.ofNullable(request);
    }

}
