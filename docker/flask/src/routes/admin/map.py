from flask import Blueprint, request
from ...models import Map
from flask_cors import CORS
from ...helper_functions import *

blueprint = Blueprint('map_admin', __name__, url_prefix='/api/map')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])


@blueprint.route("", methods=["PUT"])
# @login_required
def map_put():
    result = auth_token(request)
    if not result[0]:
        return result[1]

    request_data = request.get_json()

    id = get_if_exist(request_data,"id")
    name = get_if_exist(request_data,"name")
    image = get_if_exist(request_data,"image")
    ref = get_if_exist(request_data,"ref")

    ref = Map.query.filter_by(name=ref).first().id if ref != "" else None

    if not id:
        return send_status(Map.create(name,image,ref))

    map_object = Map.query.get(id)

    return send_status(map_object.update(name,image,ref))


@blueprint.route("<id>",methods=["DELETE"])
def map_delete(id):
    result = auth_token(request)
    if not result[0]:
        return result[1]


    map_object = Map.query.get(id)
    if map_object:
        return send_status(map_object.delete())
    else:
        return "Failed", status.HTTP_400_BAD_REQUEST
