from flask import Blueprint,jsonify
from flask_cors import CORS
from flask_api import status
import os

blueprint = Blueprint('site_settings', __name__, url_prefix='/api/site_settings')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("",methods=["GET"])
def companies_get():
    json = dict()
    json["server_mode"] = os.environ["SERVER_MODE"]
    return jsonify(json), status.HTTP_200_OK
