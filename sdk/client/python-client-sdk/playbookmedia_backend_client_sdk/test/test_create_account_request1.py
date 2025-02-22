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

from playbookmedia_backend_client_sdk.models.create_account_request1 import CreateAccountRequest1

class TestCreateAccountRequest1(unittest.TestCase):
    """CreateAccountRequest1 unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> CreateAccountRequest1:
        """Test CreateAccountRequest1
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `CreateAccountRequest1`
        """
        model = CreateAccountRequest1()
        if include_optional:
            return CreateAccountRequest1(
                auth0_user_id = '',
                email = '',
                base_directory = '',
                region = '',
                org_id = '',
                tenant_id = '',
                roles = [
                    ''
                    ],
                permissions = [
                    ''
                    ],
                mfa_enabled = True,
                compliance_level = 'COMPLIANCE_LEVEL_UNSPECIFIED',
                preferences = {
                    'key' : ''
                    }
            )
        else:
            return CreateAccountRequest1(
                auth0_user_id = '',
                email = '',
        )
        """

    def testCreateAccountRequest1(self):
        """Test CreateAccountRequest1"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
