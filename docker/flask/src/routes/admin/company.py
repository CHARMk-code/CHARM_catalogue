"""
This file contians all auth api call that handles company datatype
"""
from flask import Blueprint, request, jsonify
from ...models import Company
import sys
from flask_cors import CORS
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
    request_data = request.get_json()
    id = try_int(get_if_exist(request_data,"id"))
    delete_option = try_bool(get_if_exist(request_data,"delete"))

    name = get_if_exist(request_data,"name")
    active = try_bool(get_if_exist(request_data,"active"))
    description = get_if_exist(request_data,"description")
    trivia = get_if_exist(request_data,"trivia")
    founded = get_if_exist(request_data,"founded")
    contacts = get_if_exist(request_data,"contacts")
    employs_sweden = get_if_exist(request_data,"employs_sweden")
    employs_world = get_if_exist(request_data,"employs_world")
    website = get_if_exist(request_data, "website")
    tags = get_if_exist(request_data, "tags")

    print(active, delete_option,file=sys.stderr)

    tag_objs=[]
    if tags:
        for tag in tags:
            tag_objs.append(Tag.query.get(tag))


    if not id:

        return send_status(Company.create(name,active,description, trivia,
            founded, contacts, employs_sweden, employs_world, website,tag_objs))

    company = Company.query.get(id)

    success = False
    if delete_option:
        success = company.delete()
    else:
        success = company.update(name,active, description, trivia, founded,
                contacts, employs_sweden, employs_world, website, tag_objs)
    return send_status(success)
