from flask import Blueprint, jsonify
from ...models import Map
from flask_cors import CORS
from flask_api import status

blueprint = Blueprint('map', __name__, url_prefix='/api/map')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("",methods=["GET"])
def maps_get():
    """
    Get endpoint api/company

    Return:
        List Companies - A json list of all active maps.
    """
    maps = Map.query.all()
    return jsonify([map_obejct.serialize for map_obejct in maps]), status.HTTP_200_OK
