import sys
from setuptools import setup, find_packages

NAME = "playbookmedia_backend_server_sdk"
VERSION = "1.7.0"

# To install the library, run the following
#
# python setup.py install
#
# prerequisite: setuptools
# http://pypi.python.org/pypi/setuptools

REQUIRES = [
    "connexion>=2.0.2",
    "swagger-ui-bundle>=0.0.2",
    "python_dateutil>=2.6.0"
]

setup(
    name=NAME,
    version=VERSION,
    description="Lead Scraping Service API",
    author_email="yoanyomba@vector.ai",
    url="",
    keywords=["OpenAPI", "Lead Scraping Service API"],
    install_requires=REQUIRES,
    packages=find_packages(),
    package_data={'': ['openapi/openapi.yaml']},
    include_package_data=True,
    entry_points={
        'console_scripts': ['playbookmedia_backend_server_sdk=playbookmedia_backend_server_sdk.__main__:main']},
    long_description="""\
    Vector Lead Scraping Service API - Manages Lead Scraping Jobs
    """
)

