from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class GetAccountUsageResponse(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, total_jobs_run=None, monthly_job_limit=None, remaining_jobs=None, reset_time=None):  # noqa: E501
        """GetAccountUsageResponse - a model defined in OpenAPI

        :param total_jobs_run: The total_jobs_run of this GetAccountUsageResponse.  # noqa: E501
        :type total_jobs_run: int
        :param monthly_job_limit: The monthly_job_limit of this GetAccountUsageResponse.  # noqa: E501
        :type monthly_job_limit: int
        :param remaining_jobs: The remaining_jobs of this GetAccountUsageResponse.  # noqa: E501
        :type remaining_jobs: int
        :param reset_time: The reset_time of this GetAccountUsageResponse.  # noqa: E501
        :type reset_time: datetime
        """
        self.openapi_types = {
            'total_jobs_run': int,
            'monthly_job_limit': int,
            'remaining_jobs': int,
            'reset_time': datetime
        }

        self.attribute_map = {
            'total_jobs_run': 'totalJobsRun',
            'monthly_job_limit': 'monthlyJobLimit',
            'remaining_jobs': 'remainingJobs',
            'reset_time': 'resetTime'
        }

        self._total_jobs_run = total_jobs_run
        self._monthly_job_limit = monthly_job_limit
        self._remaining_jobs = remaining_jobs
        self._reset_time = reset_time

    @classmethod
    def from_dict(cls, dikt) -> 'GetAccountUsageResponse':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The GetAccountUsageResponse of this GetAccountUsageResponse.  # noqa: E501
        :rtype: GetAccountUsageResponse
        """
        return util.deserialize_model(dikt, cls)

    @property
    def total_jobs_run(self) -> int:
        """Gets the total_jobs_run of this GetAccountUsageResponse.


        :return: The total_jobs_run of this GetAccountUsageResponse.
        :rtype: int
        """
        return self._total_jobs_run

    @total_jobs_run.setter
    def total_jobs_run(self, total_jobs_run: int):
        """Sets the total_jobs_run of this GetAccountUsageResponse.


        :param total_jobs_run: The total_jobs_run of this GetAccountUsageResponse.
        :type total_jobs_run: int
        """

        self._total_jobs_run = total_jobs_run

    @property
    def monthly_job_limit(self) -> int:
        """Gets the monthly_job_limit of this GetAccountUsageResponse.


        :return: The monthly_job_limit of this GetAccountUsageResponse.
        :rtype: int
        """
        return self._monthly_job_limit

    @monthly_job_limit.setter
    def monthly_job_limit(self, monthly_job_limit: int):
        """Sets the monthly_job_limit of this GetAccountUsageResponse.


        :param monthly_job_limit: The monthly_job_limit of this GetAccountUsageResponse.
        :type monthly_job_limit: int
        """

        self._monthly_job_limit = monthly_job_limit

    @property
    def remaining_jobs(self) -> int:
        """Gets the remaining_jobs of this GetAccountUsageResponse.


        :return: The remaining_jobs of this GetAccountUsageResponse.
        :rtype: int
        """
        return self._remaining_jobs

    @remaining_jobs.setter
    def remaining_jobs(self, remaining_jobs: int):
        """Sets the remaining_jobs of this GetAccountUsageResponse.


        :param remaining_jobs: The remaining_jobs of this GetAccountUsageResponse.
        :type remaining_jobs: int
        """

        self._remaining_jobs = remaining_jobs

    @property
    def reset_time(self) -> datetime:
        """Gets the reset_time of this GetAccountUsageResponse.


        :return: The reset_time of this GetAccountUsageResponse.
        :rtype: datetime
        """
        return self._reset_time

    @reset_time.setter
    def reset_time(self, reset_time: datetime):
        """Sets the reset_time of this GetAccountUsageResponse.


        :param reset_time: The reset_time of this GetAccountUsageResponse.
        :type reset_time: datetime
        """

        self._reset_time = reset_time
