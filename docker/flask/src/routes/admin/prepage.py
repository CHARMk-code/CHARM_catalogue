from flask import Blueprint, request
from flask_cors import CORS
from ...helper_functions import *
from ...models import Prepage
blueprint = Blueprint('prepage_admin', __name__, url_prefix='/api/prepage')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("", methods=["PUT"])
# @login_required
def prepage_put():
    """
    PUT endpoint /api/prepage

    Return:
        200_ok
        500_internal_server_error

    """
    result = auth_token(request)
    if not result[0]:
        return result[1]

    request_data = request.get_json()
    id = try_int(get_if_exist(request_data,"id"))
    active = get_if_exist(request_data, "active")
    image = get_if_exist(request_data, "image")
    order = get_if_exist(request_data, "order")

    prepage = Prepage.query.get(id)
    if not prepage:

        return send_status(Prepage.create(active, image,order))
    return send_status(prepage.update(active, image,order))

@blueprint.route("<id>", methods=["DELETE"])
def prepage_delete(id):
    result = auth_token(request)
    if not result[0]:
        return result[1]

    prepage = Prepage.query.get(id)
    if prepage:
        return send_status(prepage.delete())
    return "Failed", status.HTTP_400_BAD_REQUEST
