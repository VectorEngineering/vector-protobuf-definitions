package org.openapitools.api;

import org.openapitools.model.AuthenticationErrorMessageResponse1;
import org.openapitools.model.ConflictErrorMessageResponse;
import org.openapitools.model.CreateAccountRequest1;
import org.openapitools.model.CreateAccountResponse1;
import org.openapitools.model.CreateWorkspaceRequest1;
import org.openapitools.model.CreateWorkspaceResponse1;
import org.springframework.format.annotation.DateTimeFormat;
import org.openapitools.model.DeleteAccountResponse;
import org.openapitools.model.DeleteWorkspaceResponse;
import org.openapitools.model.ForbiddenErrorMessageResponse;
import org.openapitools.model.GetAccountResponse1;
import org.openapitools.model.GetWorkspaceAnalyticsResponse1;
import org.openapitools.model.GetWorkspaceComplianceReportResponse;
import org.openapitools.model.GetWorkspaceResponse;
import org.openapitools.model.GetWorkspaceStorageStatsResponse;
import org.openapitools.model.InternalErrorMessageResponse;
import org.openapitools.model.ListWorkspaceSharingsResponse;
import org.openapitools.model.ListWorkspacesResponse1;
import org.openapitools.model.NotFoundErrorMessageResponse;
import java.time.OffsetDateTime;
import org.openapitools.model.RateLimitErrorMessageResponse;
import org.openapitools.model.RemoveWorkspaceSharingResponse;
import org.openapitools.model.ShareWorkspaceBody;
import org.openapitools.model.ShareWorkspaceResponse;
import org.openapitools.model.Status;
import org.openapitools.model.UpdateAccountRequest;
import org.openapitools.model.UpdateAccountResponse;
import org.openapitools.model.UpdateWorkspaceRequest;
import org.openapitools.model.UpdateWorkspaceResponse;
import org.openapitools.model.UpdateWorkspaceSharingRequest;
import org.openapitools.model.UpdateWorkspaceSharingResponse;
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

@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", date = "2025-02-03T10:37:17.791439-05:00[America/New_York]", comments = "Generator version: 7.7.0")
@Controller
@RequestMapping("${openapi.leadScrapingService.base-path:}")
public class WorkspaceServiceApiController implements WorkspaceServiceApi {

    private final NativeWebRequest request;

    @Autowired
    public WorkspaceServiceApiController(NativeWebRequest request) {
        this.request = request;
    }

    @Override
    public Optional<NativeWebRequest> getRequest() {
        return Optional.ofNullable(request);
    }

}
