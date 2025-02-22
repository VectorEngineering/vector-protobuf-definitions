# coding: utf-8

"""
    Lead Scraping Service API

    Vector Lead Scraping Service API - Manages Lead Scraping Jobs

    The version of the OpenAPI document: 1.0
    Contact: yoanyomba@vector.ai
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from playbookmedia_backend_client_sdk.models.service_unavailable_error_message_response import ServiceUnavailableErrorMessageResponse

class TestServiceUnavailableErrorMessageResponse(unittest.TestCase):
    """ServiceUnavailableErrorMessageResponse unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> ServiceUnavailableErrorMessageResponse:
        """Test ServiceUnavailableErrorMessageResponse
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `ServiceUnavailableErrorMessageResponse`
        """
        model = ServiceUnavailableErrorMessageResponse()
        if include_optional:
            return ServiceUnavailableErrorMessageResponse(
                code = 'NO_INTERNAL_ERROR',
                message = '',
                availability_info = playbookmedia_backend_client_sdk.models.availability_info.AvailabilityInfo(
                    retry_after = 56, 
                    maintenance_window = '', 
                    affected_services = [
                        ''
                        ], 
                    status_page_url = '', ),
                error_response = playbookmedia_backend_client_sdk.models.base_error_message_response,_extending_google/rpc/status.Base error message response, extending google.rpc.Status(
                    status = playbookmedia_backend_client_sdk.models.status.Status(
                        code = 56, 
                        message = '', 
                        details = [
                            {
                                'key' : null
                                }
                            ], ), )
            )
        else:
            return ServiceUnavailableErrorMessageResponse(
        )
        """

    def testServiceUnavailableErrorMessageResponse(self):
        """Test ServiceUnavailableErrorMessageResponse"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
