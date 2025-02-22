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

from playbookmedia_backend_client_sdk.models.activity_metrics import ActivityMetrics

class TestActivityMetrics(unittest.TestCase):
    """ActivityMetrics unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> ActivityMetrics:
        """Test ActivityMetrics
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `ActivityMetrics`
        """
        model = ActivityMetrics()
        if include_optional:
            return ActivityMetrics(
                total_files = 56,
                total_folders = 56,
                active_users = 56,
                storage_used = '',
                storage_usage_percentage = 1.337
            )
        else:
            return ActivityMetrics(
        )
        """

    def testActivityMetrics(self):
        """Test ActivityMetrics"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
