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

from backend_client.models.download_scraping_results_response import DownloadScrapingResultsResponse

class TestDownloadScrapingResultsResponse(unittest.TestCase):
    """DownloadScrapingResultsResponse unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> DownloadScrapingResultsResponse:
        """Test DownloadScrapingResultsResponse
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `DownloadScrapingResultsResponse`
        """
        model = DownloadScrapingResultsResponse()
        if include_optional:
            return DownloadScrapingResultsResponse(
                content = 'YQ==',
                filename = '',
                content_type = ''
            )
        else:
            return DownloadScrapingResultsResponse(
        )
        """

    def testDownloadScrapingResultsResponse(self):
        """Test DownloadScrapingResultsResponse"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
