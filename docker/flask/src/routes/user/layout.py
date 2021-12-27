from flask import Blueprint, jsonify
from ...models import Layout
from flask_cors import CORS
from flask_api import status

blueprint = Blueprint('layout', __name__, url_prefix='/api/layout')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("",methods=["GET"])
def layouts_get():
    layouts = Layout.query.all()
    return jsonify([map_obejct.serialize for map_obejct in layouts]), status.HTTP_200_OK
