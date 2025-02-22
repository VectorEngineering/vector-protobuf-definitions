from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from playbookmedia_backend_server_sdk.models.base_model import Model
from playbookmedia_backend_server_sdk import util


class SignatureBlock(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, id=None, request_id=None, block_type=None, page_number=None, x_position=None, y_position=None, width=None, height=None, is_required=None, signature_data=None, signed_at=None):  # noqa: E501
        """SignatureBlock - a model defined in OpenAPI

        :param id: The id of this SignatureBlock.  # noqa: E501
        :type id: str
        :param request_id: The request_id of this SignatureBlock.  # noqa: E501
        :type request_id: str
        :param block_type: The block_type of this SignatureBlock.  # noqa: E501
        :type block_type: str
        :param page_number: The page_number of this SignatureBlock.  # noqa: E501
        :type page_number: int
        :param x_position: The x_position of this SignatureBlock.  # noqa: E501
        :type x_position: float
        :param y_position: The y_position of this SignatureBlock.  # noqa: E501
        :type y_position: float
        :param width: The width of this SignatureBlock.  # noqa: E501
        :type width: float
        :param height: The height of this SignatureBlock.  # noqa: E501
        :type height: float
        :param is_required: The is_required of this SignatureBlock.  # noqa: E501
        :type is_required: bool
        :param signature_data: The signature_data of this SignatureBlock.  # noqa: E501
        :type signature_data: str
        :param signed_at: The signed_at of this SignatureBlock.  # noqa: E501
        :type signed_at: datetime
        """
        self.openapi_types = {
            'id': str,
            'request_id': str,
            'block_type': str,
            'page_number': int,
            'x_position': float,
            'y_position': float,
            'width': float,
            'height': float,
            'is_required': bool,
            'signature_data': str,
            'signed_at': datetime
        }

        self.attribute_map = {
            'id': 'id',
            'request_id': 'requestId',
            'block_type': 'blockType',
            'page_number': 'pageNumber',
            'x_position': 'xPosition',
            'y_position': 'yPosition',
            'width': 'width',
            'height': 'height',
            'is_required': 'isRequired',
            'signature_data': 'signatureData',
            'signed_at': 'signedAt'
        }

        self._id = id
        self._request_id = request_id
        self._block_type = block_type
        self._page_number = page_number
        self._x_position = x_position
        self._y_position = y_position
        self._width = width
        self._height = height
        self._is_required = is_required
        self._signature_data = signature_data
        self._signed_at = signed_at

    @classmethod
    def from_dict(cls, dikt) -> 'SignatureBlock':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The SignatureBlock of this SignatureBlock.  # noqa: E501
        :rtype: SignatureBlock
        """
        return util.deserialize_model(dikt, cls)

    @property
    def id(self) -> str:
        """Gets the id of this SignatureBlock.


        :return: The id of this SignatureBlock.
        :rtype: str
        """
        return self._id

    @id.setter
    def id(self, id: str):
        """Sets the id of this SignatureBlock.


        :param id: The id of this SignatureBlock.
        :type id: str
        """

        self._id = id

    @property
    def request_id(self) -> str:
        """Gets the request_id of this SignatureBlock.


        :return: The request_id of this SignatureBlock.
        :rtype: str
        """
        return self._request_id

    @request_id.setter
    def request_id(self, request_id: str):
        """Sets the request_id of this SignatureBlock.


        :param request_id: The request_id of this SignatureBlock.
        :type request_id: str
        """

        self._request_id = request_id

    @property
    def block_type(self) -> str:
        """Gets the block_type of this SignatureBlock.


        :return: The block_type of this SignatureBlock.
        :rtype: str
        """
        return self._block_type

    @block_type.setter
    def block_type(self, block_type: str):
        """Sets the block_type of this SignatureBlock.


        :param block_type: The block_type of this SignatureBlock.
        :type block_type: str
        """

        self._block_type = block_type

    @property
    def page_number(self) -> int:
        """Gets the page_number of this SignatureBlock.


        :return: The page_number of this SignatureBlock.
        :rtype: int
        """
        return self._page_number

    @page_number.setter
    def page_number(self, page_number: int):
        """Sets the page_number of this SignatureBlock.


        :param page_number: The page_number of this SignatureBlock.
        :type page_number: int
        """

        self._page_number = page_number

    @property
    def x_position(self) -> float:
        """Gets the x_position of this SignatureBlock.


        :return: The x_position of this SignatureBlock.
        :rtype: float
        """
        return self._x_position

    @x_position.setter
    def x_position(self, x_position: float):
        """Sets the x_position of this SignatureBlock.


        :param x_position: The x_position of this SignatureBlock.
        :type x_position: float
        """

        self._x_position = x_position

    @property
    def y_position(self) -> float:
        """Gets the y_position of this SignatureBlock.


        :return: The y_position of this SignatureBlock.
        :rtype: float
        """
        return self._y_position

    @y_position.setter
    def y_position(self, y_position: float):
        """Sets the y_position of this SignatureBlock.


        :param y_position: The y_position of this SignatureBlock.
        :type y_position: float
        """

        self._y_position = y_position

    @property
    def width(self) -> float:
        """Gets the width of this SignatureBlock.


        :return: The width of this SignatureBlock.
        :rtype: float
        """
        return self._width

    @width.setter
    def width(self, width: float):
        """Sets the width of this SignatureBlock.


        :param width: The width of this SignatureBlock.
        :type width: float
        """

        self._width = width

    @property
    def height(self) -> float:
        """Gets the height of this SignatureBlock.


        :return: The height of this SignatureBlock.
        :rtype: float
        """
        return self._height

    @height.setter
    def height(self, height: float):
        """Sets the height of this SignatureBlock.


        :param height: The height of this SignatureBlock.
        :type height: float
        """

        self._height = height

    @property
    def is_required(self) -> bool:
        """Gets the is_required of this SignatureBlock.


        :return: The is_required of this SignatureBlock.
        :rtype: bool
        """
        return self._is_required

    @is_required.setter
    def is_required(self, is_required: bool):
        """Sets the is_required of this SignatureBlock.


        :param is_required: The is_required of this SignatureBlock.
        :type is_required: bool
        """

        self._is_required = is_required

    @property
    def signature_data(self) -> str:
        """Gets the signature_data of this SignatureBlock.


        :return: The signature_data of this SignatureBlock.
        :rtype: str
        """
        return self._signature_data

    @signature_data.setter
    def signature_data(self, signature_data: str):
        """Sets the signature_data of this SignatureBlock.


        :param signature_data: The signature_data of this SignatureBlock.
        :type signature_data: str
        """

        self._signature_data = signature_data

    @property
    def signed_at(self) -> datetime:
        """Gets the signed_at of this SignatureBlock.


        :return: The signed_at of this SignatureBlock.
        :rtype: datetime
        """
        return self._signed_at

    @signed_at.setter
    def signed_at(self, signed_at: datetime):
        """Sets the signed_at of this SignatureBlock.


        :param signed_at: The signed_at of this SignatureBlock.
        :type signed_at: datetime
        """

        self._signed_at = signed_at
