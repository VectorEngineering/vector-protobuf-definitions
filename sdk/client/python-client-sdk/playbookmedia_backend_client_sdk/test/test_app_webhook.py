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

from playbookmedia_backend_client_sdk.models.app_webhook import AppWebhook

class TestAppWebhook(unittest.TestCase):
    """AppWebhook unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> AppWebhook:
        """Test AppWebhook
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `AppWebhook`
        """
        model = AppWebhook()
        if include_optional:
            return AppWebhook(
                id = '',
                app_id = '',
                url = '',
                subscribed_events = [
                    ''
                    ],
                secret_key = '',
                retry_count = 56,
                timeout_seconds = 56,
                is_active = True,
                headers = {
                    'key' : ''
                    },
                last_triggered = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'),
                created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f')
            )
        else:
            return AppWebhook(
        )
        """

    def testAppWebhook(self):
        """Test AppWebhook"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
