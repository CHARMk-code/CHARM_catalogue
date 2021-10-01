"""
This file contians all auth api call that handles company datatype
"""
from flask import Blueprint, request, jsonify
from ...models import Company
import sys
from flask_cors import CORS
from flask_api import status
from ...shared_data import last_update_company
from ...helper_functions import *

blueprint = Blueprint('company_admin', __name__, url_prefix='/api/company') 
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])


@blueprint.route("/update", methods=["POST"])
# @login_required
def company_update():
    """
    POST endpoint api/company/update

    Args:
        id - Id of target company, if not provided a new object is created .
        name - New name of the company (if not provided the value will be set to None)
        active - New statue of the company (if not provided the value will be set to None)
        page - New page of the company (if not provided the value will be set to None)
        delete_option - If set the give id will be deleted       
    return:
        200_ok - If ok
        500_internal_server_error - otherwise
    """
    id = try_int(request.form.get("id"))
    name = request.form.get("name")
    active = try_bool(request.form.get("active"))
    page = try_int(request.form.get("page"))
    delete_option = request.form.get("delete")

    if not id:
        return send_status(company_create(name,active,page))

    success = False
    if delete_option:
        success = company_delete(id)
    else:
        success = company_update_helper(id,name,active,page)
    return send_status(success)
