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

from playbookmedia_backend_client_sdk.models.file_version import FileVersion

class TestFileVersion(unittest.TestCase):
    """FileVersion unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> FileVersion:
        """Test FileVersion
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `FileVersion`
        """
        model = FileVersion()
        if include_optional:
            return FileVersion(
                id = '',
                file_id = '',
                version_number = 56,
                s3_key = '',
                size = '',
                commit_message = '',
                author_id = '',
                created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f')
            )
        else:
            return FileVersion(
        )
        """

    def testFileVersion(self):
        """Test FileVersion"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
