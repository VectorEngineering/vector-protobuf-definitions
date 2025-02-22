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

from backend_client.models.resource_validation import ResourceValidation

class TestResourceValidation(unittest.TestCase):
    """ResourceValidation unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> ResourceValidation:
        """Test ResourceValidation
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `ResourceValidation`
        """
        model = ResourceValidation()
        if include_optional:
            return ResourceValidation(
                missing_resources = [
                    ''
                    ],
                invalid_references = [
                    ''
                    ],
                id_violations = {
                    'key' : ''
                    }
            )
        else:
            return ResourceValidation(
        )
        """

    def testResourceValidation(self):
        """Test ResourceValidation"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
