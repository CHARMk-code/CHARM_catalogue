from flask import Blueprint, request
from ...models import Shortcut 
from flask_cors import CORS
from ...helper_functions import *

blueprint = Blueprint('shortcut_admin', __name__, url_prefix='/api/shortcut')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])


@blueprint.route("", methods=["PUT"])
# @login_required
def shortcut_put():
    result = auth_token(request)
    if not result[0]:
        return result[1]

    request_data = request.get_json()

    id = get_if_exist(request_data,"id")
    name = get_if_exist(request_data,"name")
    desc = get_if_exist(request_data,"desc")
    link = get_if_exist(request_data,"link")
    icon = get_if_exist(request_data,"icon")

    if not id:
        return send_status(Shortcut.create(name,desc,link,icon))

    shortcut_object = Shortcut.query.get(id)

    return send_status(shortcut_object.update(name,desc,link,icon))


@blueprint.route("<id>",methods=["DELETE"])
def shortcut_delete(id):
    result = auth_token(request)
    if not result[0]:
        return result[1]


    shortcut_object = Shortcut.query.get(id)
    if shortcut_object:
        return send_status(shortcut_object.delete())
    else:
        return "Failed", status.HTTP_400_BAD_REQUEST
