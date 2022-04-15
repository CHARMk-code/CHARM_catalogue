from flask import Blueprint,jsonify
from ..models import Company, Layout, Map, Prepage, Shortcut, Company_card, Tag, serializeGeneric
from flask_cors import CORS
from flask_api import status

blueprint = Blueprint('user', __name__, url_prefix='/api')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

path_LUT = {
"company": Company,
"layout": Layout,
"map": Map,
"prepage": Prepage,
"shortcut": Shortcut,
"settings": Company_card,
"tag": Tag
}

@blueprint.route("/<path>",methods=["GET"])
def getAll(path):
    table = path_LUT[path]
    rows = table.query.all()
    return jsonify([serializeGeneric(table,row) for row in rows]), status.HTTP_200_OK


@blueprint.route("/<path>/<year>",methods=["GET"])
def getSpecfic(path,year):
    table = path_LUT[path]
    rows = table.query.filter_by(year = year).all()
    return jsonify([serializeGeneric(table,row) for row in rows]), status.HTTP_200_OK