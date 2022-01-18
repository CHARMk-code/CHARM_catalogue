from flask import Blueprint, jsonify
from ...models import Shortcut 
from flask_cors import CORS
from flask_api import status

blueprint = Blueprint('shortcut', __name__, url_prefix='/api/shortcut')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("",methods=["GET"])
def shortcut_get():
    shortcuts = Shortcut.query.all()
    return jsonify([shortcut_object.serialize for shortcut_object in shortcuts]), status.HTTP_200_OK
