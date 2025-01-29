import unittest

from flask import json

from backend_server.models.authentication_error_message_response import AuthenticationErrorMessageResponse  # noqa: E501
from backend_server.models.bad_gateway_error_message_response import BadGatewayErrorMessageResponse  # noqa: E501
from backend_server.models.conflict_error_message_response import ConflictErrorMessageResponse  # noqa: E501
from backend_server.models.create_account_request import CreateAccountRequest  # noqa: E501
from backend_server.models.create_account_response import CreateAccountResponse  # noqa: E501
from backend_server.models.create_scraping_job_request import CreateScrapingJobRequest  # noqa: E501
from backend_server.models.create_scraping_job_response import CreateScrapingJobResponse  # noqa: E501
from backend_server.models.delete_account_response import DeleteAccountResponse  # noqa: E501
from backend_server.models.delete_scraping_job_response import DeleteScrapingJobResponse  # noqa: E501
from backend_server.models.download_scraping_results_response import DownloadScrapingResultsResponse  # noqa: E501
from backend_server.models.forbidden_error_message_response import ForbiddenErrorMessageResponse  # noqa: E501
from backend_server.models.gateway_timeout_error_message_response import GatewayTimeoutErrorMessageResponse  # noqa: E501
from backend_server.models.get_account_response import GetAccountResponse  # noqa: E501
from backend_server.models.get_scraping_job_response import GetScrapingJobResponse  # noqa: E501
from backend_server.models.gone_error_message_response import GoneErrorMessageResponse  # noqa: E501
from backend_server.models.internal_error_message_response import InternalErrorMessageResponse  # noqa: E501
from backend_server.models.list_accounts_response import ListAccountsResponse  # noqa: E501
from backend_server.models.list_scraping_jobs_response import ListScrapingJobsResponse  # noqa: E501
from backend_server.models.method_not_allowed_error_message_response import MethodNotAllowedErrorMessageResponse  # noqa: E501
from backend_server.models.not_found_error_message_response import NotFoundErrorMessageResponse  # noqa: E501
from backend_server.models.not_implemented_error_message_response import NotImplementedErrorMessageResponse  # noqa: E501
from backend_server.models.payment_required_error_message_response import PaymentRequiredErrorMessageResponse  # noqa: E501
from backend_server.models.precondition_failed_error_message_response import PreconditionFailedErrorMessageResponse  # noqa: E501
from backend_server.models.rate_limit_error_message_response import RateLimitErrorMessageResponse  # noqa: E501
from backend_server.models.service_unavailable_error_message_response import ServiceUnavailableErrorMessageResponse  # noqa: E501
from backend_server.models.status import Status  # noqa: E501
from backend_server.models.too_early_error_message_response import TooEarlyErrorMessageResponse  # noqa: E501
from backend_server.models.unprocessable_entity_error_message_response import UnprocessableEntityErrorMessageResponse  # noqa: E501
from backend_server.models.update_account_request import UpdateAccountRequest  # noqa: E501
from backend_server.models.update_account_response import UpdateAccountResponse  # noqa: E501
from backend_server.models.validation_error_message_response import ValidationErrorMessageResponse  # noqa: E501
from backend_server.test import BaseTestCase


class TestLeadScraperServiceController(BaseTestCase):
    """LeadScraperServiceController integration test stubs"""

    @unittest.skip("Connexion does not support multiple consumes. See https://github.com/zalando/connexion/pull/760")
    def test_create_account(self):
        """Test case for create_account

        Create a new account
        """
        create_account_request = {"authPlatformUserId":"authPlatformUserId","tenantId":"tenantId","orgId":"orgId","email":"email"}
        headers = { 
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/lead-scraper-microservice/api/v1/accounts',
            method='POST',
            headers=headers,
            data=json.dumps(create_account_request),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    @unittest.skip("Connexion does not support multiple consumes. See https://github.com/zalando/connexion/pull/760")
    def test_create_scraping_job(self):
        """Test case for create_scraping_job

        Create a new job scraping task
        """
        create_scraping_job_request = {"maxTime":5,"keywords":["keywords","keywords"],"fastMode":True,"zoom":0,"lon":"lon","userId":"userId","orgId":"orgId","depth":1,"tenantId":"tenantId","name":"name","proxies":["proxies","proxies"],"lang":"lang","radius":6,"lat":"lat","email":True}
        headers = { 
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/lead-scraper-microservice/api/v1/jobs',
            method='POST',
            headers=headers,
            data=json.dumps(create_scraping_job_request),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_delete_account(self):
        """Test case for delete_account

        Delete account
        """
        query_string = [('orgId', 'org_id_example'),
                        ('tenantId', 'tenant_id_example')]
        headers = { 
            'Accept': 'application/json',
        }
        response = self.client.open(
            '/lead-scraper-microservice/api/v1/accounts/{account_id}'.format(account_id='account_id_example'),
            method='DELETE',
            headers=headers,
            query_string=query_string)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_delete_scraping_job(self):
        """Test case for delete_scraping_job

        Delete a specific job
        """
        query_string = [('userId', 'user_id_example'),
                        ('orgId', 'org_id_example'),
                        ('tenantId', 'tenant_id_example')]
        headers = { 
            'Accept': 'application/json',
        }
        response = self.client.open(
            '/lead-scraper-microservice/api/v1/jobs/{job_id}'.format(job_id='job_id_example'),
            method='DELETE',
            headers=headers,
            query_string=query_string)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_download_scraping_results(self):
        """Test case for download_scraping_results

        Download job results as CSV
        """
        query_string = [('userId', 'user_id_example'),
                        ('orgId', 'org_id_example'),
                        ('tenantId', 'tenant_id_example')]
        headers = { 
            'Accept': 'application/json',
        }
        response = self.client.open(
            '/lead-scraper-microservice/api/v1/jobs/{job_id}/download'.format(job_id='job_id_example'),
            method='GET',
            headers=headers,
            query_string=query_string)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_get_account(self):
        """Test case for get_account

        Get account details
        """
        query_string = [('orgId', 'org_id_example'),
                        ('tenantId', 'tenant_id_example')]
        headers = { 
            'Accept': 'application/json',
        }
        response = self.client.open(
            '/lead-scraper-microservice/api/v1/accounts/{account_id}'.format(account_id='account_id_example'),
            method='GET',
            headers=headers,
            query_string=query_string)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_get_scraping_job(self):
        """Test case for get_scraping_job

        Get a specific job
        """
        query_string = [('userId', 'user_id_example'),
                        ('orgId', 'org_id_example'),
                        ('tenantId', 'tenant_id_example')]
        headers = { 
            'Accept': 'application/json',
        }
        response = self.client.open(
            '/lead-scraper-microservice/api/v1/jobs/{job_id}'.format(job_id='job_id_example'),
            method='GET',
            headers=headers,
            query_string=query_string)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_list_accounts(self):
        """Test case for list_accounts

        List accounts
        """
        query_string = [('orgId', 'org_id_example'),
                        ('tenantId', 'tenant_id_example'),
                        ('offset', 56),
                        ('limit', 56)]
        headers = { 
            'Accept': 'application/json',
        }
        response = self.client.open(
            '/lead-scraper-microservice/api/v1/accounts',
            method='GET',
            headers=headers,
            query_string=query_string)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_list_scraping_jobs(self):
        """Test case for list_scraping_jobs

        Get all jobs
        """
        query_string = [('userId', 'user_id_example'),
                        ('orgId', 'org_id_example'),
                        ('tenantId', 'tenant_id_example')]
        headers = { 
            'Accept': 'application/json',
        }
        response = self.client.open(
            '/lead-scraper-microservice/api/v1/jobs',
            method='GET',
            headers=headers,
            query_string=query_string)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    @unittest.skip("Connexion does not support multiple consumes. See https://github.com/zalando/connexion/pull/760")
    def test_update_account(self):
        """Test case for update_account

        Update account details
        """
        update_account_request = {"account":{"createdAt":"2000-01-23T04:56:07.000+00:00","scrapingJobs":[{"maxTime":5,"keywords":["keywords","keywords"],"fastMode":True,"zoom":0,"lon":"lon","priority":5,"createdAt":"2000-01-23T04:56:07.000+00:00","deletedAt":"2000-01-23T04:56:07.000+00:00","depth":1,"payloadType":"payloadType","payload":"payload","name":"name","proxies":["proxies","proxies"],"id":"id","lang":"lang","radius":6,"lat":"lat","email":True,"status":"BACKGROUND_JOB_STATUS_UNSPECIFIED","updatedAt":"2000-01-23T04:56:07.000+00:00"},{"maxTime":5,"keywords":["keywords","keywords"],"fastMode":True,"zoom":0,"lon":"lon","priority":5,"createdAt":"2000-01-23T04:56:07.000+00:00","deletedAt":"2000-01-23T04:56:07.000+00:00","depth":1,"payloadType":"payloadType","payload":"payload","name":"name","proxies":["proxies","proxies"],"id":"id","lang":"lang","radius":6,"lat":"lat","email":True,"status":"BACKGROUND_JOB_STATUS_UNSPECIFIED","updatedAt":"2000-01-23T04:56:07.000+00:00"}],"deletedAt":"2000-01-23T04:56:07.000+00:00","lastModifiedAt":"2000-01-23T04:56:07.000+00:00","authPlatformUserId":"authPlatformUserId","tenantId":"tenantId","id":"id","orgId":"orgId","email":"email"}}
        headers = { 
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/lead-scraper-microservice/api/v1/accounts',
            method='PATCH',
            headers=headers,
            data=json.dumps(update_account_request),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))


if __name__ == '__main__':
    unittest.main()
