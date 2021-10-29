from flask import Blueprint, jsonify
from ...models import Company
from flask_cors import CORS
from flask_api import status
from ...shared_data import last_update_company

blueprint = Blueprint('company', __name__, url_prefix='/api/company')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("",methods=["GET"])
def companies_get():
    global last_update_company
    """
    Get endpoint api/company

    Return:
        List Companies - A json list of all active companies.
    """
    companies = Company.query.all()
    return jsonify([company.serialize for company in companies]), status.HTTP_200_OK
