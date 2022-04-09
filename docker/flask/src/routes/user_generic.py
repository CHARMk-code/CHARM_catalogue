from flask import Blueprint,jsonify
from ..models import Company, Layout, Map, Prepage, Shortcut, Company_card, Tag
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
def company_view(path):
    table = path_LUT[path]
    rows = table.query.all()
    return jsonify([row.serialize for row in rows]), status.HTTP_200_OK
