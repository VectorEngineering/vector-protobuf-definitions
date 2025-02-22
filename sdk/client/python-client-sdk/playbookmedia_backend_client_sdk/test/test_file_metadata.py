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

from playbookmedia_backend_client_sdk.models.file_metadata import FileMetadata

class TestFileMetadata(unittest.TestCase):
    """FileMetadata unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> FileMetadata:
        """Test FileMetadata
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `FileMetadata`
        """
        model = FileMetadata()
        if include_optional:
            return FileMetadata(
                id = '',
                name = '',
                size = '',
                s3_key = '',
                s3_bucket_name = '',
                is_deleted = True,
                version = 56,
                created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'),
                updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'),
                deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'),
                embeddings = playbookmedia_backend_client_sdk.models.file_embeddings.FileEmbeddings(
                    id = '', 
                    model_version = '', 
                    created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                    updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), ),
                versions = [
                    playbookmedia_backend_client_sdk.models.file_version.FileVersion(
                        id = '', 
                        file_id = '', 
                        version_number = 56, 
                        s3_key = '', 
                        size = '', 
                        commit_message = '', 
                        author_id = '', 
                        created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), )
                    ],
                comments = [
                    playbookmedia_backend_client_sdk.models.comment_thread.CommentThread(
                        id = '', 
                        author_id = '', 
                        content = '', 
                        start_position = 56, 
                        end_position = 56, 
                        resolved = True, 
                        created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                        updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), )
                    ],
                sharing = [
                    playbookmedia_backend_client_sdk.models.file_sharing.FileSharing(
                        id = '', 
                        shared_with_email = '', 
                        permission_level = '', 
                        expires_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                        created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                        updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), )
                    ],
                snapshots = [
                    playbookmedia_backend_client_sdk.models.document_snapshot.DocumentSnapshot(
                        id = '', 
                        file_id = '', 
                        snapshot_hash = '', 
                        content = 'YQ==', 
                        author_id = '', 
                        reason = '', 
                        created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), )
                    ]
            )
        else:
            return FileMetadata(
        )
        """

    def testFileMetadata(self):
        """Test FileMetadata"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
