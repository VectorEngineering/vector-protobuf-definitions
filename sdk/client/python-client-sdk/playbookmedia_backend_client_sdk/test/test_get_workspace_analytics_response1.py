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

from playbookmedia_backend_client_sdk.models.get_workspace_analytics_response1 import GetWorkspaceAnalyticsResponse1

class TestGetWorkspaceAnalyticsResponse1(unittest.TestCase):
    """GetWorkspaceAnalyticsResponse1 unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> GetWorkspaceAnalyticsResponse1:
        """Test GetWorkspaceAnalyticsResponse1
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `GetWorkspaceAnalyticsResponse1`
        """
        model = GetWorkspaceAnalyticsResponse1()
        if include_optional:
            return GetWorkspaceAnalyticsResponse1(
                activity = playbookmedia_backend_client_sdk.models.activity_metrics.ActivityMetrics(
                    total_files = 56, 
                    total_folders = 56, 
                    active_users = 56, 
                    storage_used = '', 
                    storage_usage_percentage = 1.337, ),
                user_activities = [
                    playbookmedia_backend_client_sdk.models.user_activity.UserActivity(
                        user_id = '', 
                        email = '', 
                        file_operations = 56, 
                        comments_made = 56, 
                        documents_processed = 56, )
                    ],
                compliance = playbookmedia_backend_client_sdk.models.compliance_metrics.ComplianceMetrics(
                    gdpr_compliance_score = 1.337, 
                    hipaa_compliance_score = 1.337, 
                    pending_approvals = 56, 
                    compliance_violations = 56, ),
                recent_activities = [
                    playbookmedia_backend_client_sdk.models.workspace_activity.WorkspaceActivity(
                        id = '', 
                        activity_type = '', 
                        user_id = '', 
                        description = '', 
                        metadata = {
                            'key' : ''
                            }, 
                        created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), )
                    ]
            )
        else:
            return GetWorkspaceAnalyticsResponse1(
        )
        """

    def testGetWorkspaceAnalyticsResponse1(self):
        """Test GetWorkspaceAnalyticsResponse1"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
