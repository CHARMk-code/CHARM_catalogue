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
    charmtalk = get_if_exist(request_data,"charmtalk")
    description = get_if_exist(request_data,"description")
    summer_job_description = get_if_exist(request_data, "summer_job_description")
    summer_job_link = get_if_exist(request_data, "summer_job_link")
    contacts = get_if_exist(request_data,"contacts")
    contact_email = get_if_exist(request_data,"contact_email")
    employees_world = get_if_exist(request_data,"employees_world")
    website = get_if_exist(request_data, "website")
    talk_to_us_about = get_if_exist(request_data, "talk_to_us_about")
    logo = get_if_exist(request_data, "logo")
    in_sweden = get_if_exist(request_data, "in_sweden")
    map_image = get_if_exist(request_data, "map_image")
    booth_number = get_if_exist(request_data,"booth_number")
    tags = get_if_exist(request_data, "tags")

    tag_objs=[]
    if tags:
        for tag in tags:
            tag_objs.append(Tag.query.get(tag))


    if not id:

        return send_status(Company.create(name,active,charmtalk,description, summer_job_description, summer_job_link, 
            contacts, contact_email, employees_world, website, talk_to_us_about,logo,map_image, booth_number, tag_objs))

    company = Company.query.get(id)

    return send_status(company.update(name,active, charmtalk, description, summer_job_description, summer_job_link,
                contacts, contact_email, employees_world, website, talk_to_us_about, logo, map_image, booth_number, tag_objs))


@blueprint.route("<id>",methods=["DELETE"])
def company_delete(id):
    result = auth_token(request)
    if not result[0]:
        return result[1]


    company = Company.query.get(id)
    if company:
        return send_status(company.delete())
    else:
        return "Failed", status.HTTP_400_BAD_REQUEST
