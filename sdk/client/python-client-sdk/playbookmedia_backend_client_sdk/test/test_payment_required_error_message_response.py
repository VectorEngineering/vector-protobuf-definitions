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

from playbookmedia_backend_client_sdk.models.payment_required_error_message_response import PaymentRequiredErrorMessageResponse

class TestPaymentRequiredErrorMessageResponse(unittest.TestCase):
    """PaymentRequiredErrorMessageResponse unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> PaymentRequiredErrorMessageResponse:
        """Test PaymentRequiredErrorMessageResponse
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `PaymentRequiredErrorMessageResponse`
        """
        model = PaymentRequiredErrorMessageResponse()
        if include_optional:
            return PaymentRequiredErrorMessageResponse(
                code = 56,
                message = '',
                payment_info = playbookmedia_backend_client_sdk.models.payment_info.PaymentInfo(
                    amount_due = 1.337, 
                    currency = '', 
                    due_date = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                    payment_url = '', ),
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
            return PaymentRequiredErrorMessageResponse(
        )
        """

    def testPaymentRequiredErrorMessageResponse(self):
        """Test PaymentRequiredErrorMessageResponse"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
