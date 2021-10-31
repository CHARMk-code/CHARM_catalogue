"""
This file contians all auth api call that handles company datatype
"""
from flask import Blueprint, request
from ...models import Company, Tag
from flask_cors import CORS
from ...helper_functions import *

blueprint = Blueprint('company_admin', __name__, url_prefix='/api/company')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])


@blueprint.route("", methods=["PUT"])
# @login_required
def company_put():
    """

    Args:
        id - Id of target company, if not provided a new object is created .

        delete_option - If set the give id will be deleted
    return:
        200_ok - If ok
        500_internal_server_error - otherwise
    """
    result = auth_token(request)
    if not result[0]:
        return result[1]

    request_data = request.get_json()

    id = get_if_exist(request_data,"id")
    name = get_if_exist(request_data,"name")
    active = get_if_exist(request_data,"active")
    description = get_if_exist(request_data,"description")
    trivia = get_if_exist(request_data,"trivia")
    founded = get_if_exist(request_data,"founded")
    contacts = get_if_exist(request_data,"contacts")
    employs_sweden = get_if_exist(request_data,"employs_sweden")
    employs_world = get_if_exist(request_data,"employs_world")
    website = get_if_exist(request_data, "website")
    logo = get_if_exist(request_data, "logo")
    tags = get_if_exist(request_data, "tags")

    tag_objs=[]
    if tags:
        for tag in tags:
            tag_objs.append(Tag.query.get(tag))


    if not id:

        return send_status(Company.create(name,active,description, trivia,
            founded, contacts, employs_sweden, employs_world, website,logo, tag_objs))

    company = Company.query.get(id)

    return send_status(company.update(name,active, description,  trivia, founded,
                contacts, employs_sweden, employs_world, website, logo, tag_objs))


@blueprint.route("",methods=["DELETE"])
def company_delete():
    result = auth_token(request)
    if not result[0]:
        return result[1]

    request_data = request.get_json()

    id = get_if_exist(request_data,"id")
    company = Company.query.get(id)
    return send_status(company.delete())
