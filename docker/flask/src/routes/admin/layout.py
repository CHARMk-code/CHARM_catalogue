from flask import Blueprint, request
from ...models import Layout
from flask_cors import CORS
from ...helper_functions import *

blueprint = Blueprint('layout_admin', __name__, url_prefix='/api/layout')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])


@blueprint.route("", methods=["PUT"])
# @login_required
def layout_put():
    result = auth_token(request)
    if not result[0]:
        return result[1]

    request_data = request.get_json()

    id = get_if_exist(request_data,"id")
    placement = get_if_exist(request_data,"placement")
    image = get_if_exist(request_data,"image")
    active = get_if_exist(request_data,"active")

    ref = Layout.query.filter_by(image=image).first()
    if ref != None:
        ref = ref.id

    if not id:
        return send_status(Layout.create(active,image,placement))

    layout_object = Layout.query.get(id)

    return send_status(layout_object.update(active,image,placement))


@blueprint.route("<id>",methods=["DELETE"])
def layout_delete(id):
    result = auth_token(request)
    if not result[0]:
        return result[1]

    layout_object = Layout.query.get(id)
    if layout_object:
        return send_status(layout_object.delete())
    else:
        return "Failed", status.HTTP_400_BAD_REQUEST
