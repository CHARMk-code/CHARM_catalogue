from flask import Blueprint, request, jsonify
from ...models import Company
import sys
from flask_cors import CORS
from flask_api import status
from ...shared_data import last_update_company

blueprint = Blueprint('company', __name__, url_prefix='/api/company') 
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("/get")
def companies_get():
    global last_update_company
    """
    Get endpoint api/company/get
 
    Return:
        List Companys - A json list of all active companies.
    """
    timestamp = request.args.get("timestamp")
    if timestamp:
        return jsonify(last_update_company), status.HTTP_200_OK
    companies = Company.query.filter_by(active = True).all()
    return jsonify([company.serialize for company in companies]), status.HTTP_200_OK
