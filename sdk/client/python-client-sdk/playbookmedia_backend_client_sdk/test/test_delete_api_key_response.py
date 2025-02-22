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

from playbookmedia_backend_client_sdk.models.delete_api_key_response import DeleteAPIKeyResponse

class TestDeleteAPIKeyResponse(unittest.TestCase):
    """DeleteAPIKeyResponse unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> DeleteAPIKeyResponse:
        """Test DeleteAPIKeyResponse
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `DeleteAPIKeyResponse`
        """
        model = DeleteAPIKeyResponse()
        if include_optional:
            return DeleteAPIKeyResponse(
                success = True
            )
        else:
            return DeleteAPIKeyResponse(
        )
        """

    def testDeleteAPIKeyResponse(self):
        """Test DeleteAPIKeyResponse"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
