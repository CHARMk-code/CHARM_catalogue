from flask import Blueprint,jsonify
from ...models import Company_card
from flask_cors import CORS
from flask_api import status
from ...helper_functions import *
import os

blueprint = Blueprint('settings', __name__, url_prefix='/api/settings')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("company_view",methods=["GET"])
def company_view():
    cards = Company_card.query.all()
    return jsonify([card_object.serialize for card_object in cards]), status.HTTP_200_OK
