from flask import Blueprint, request, jsonify
from flask_api import status
from flask_cors import CORS
from ...helper_functions import *
blueprint = Blueprint('tag_admin', __name__, url_prefix='/api/tag') 
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("/update", methods=["POST"])
# @login_required
def tag_update():
    id = try_int(request.form.get("id"))
    name = request.form.get("name")
    parent_tag = try_int(request.form.get("parent_tag"))
    crowd_soured = try_bool(request.form.get("crowd_soured"))
    delete_option = request.form.get("delete")

    if not id:
        return send_status(tag_create(name,parent_tag,crowd_soured))

    success = False
    if delete_option:
        success = tag_delete(id)
    else:
        success = tag_update_helper(id, name, parent_tag, crowd_soured)
    return send_status(success)

@blueprint.route("/company/update", methods=["POST"])
# @login_required
def tag_company_update():
    id = try_int(request.form.get("id"))
    tag = try_int(request.form.get("tag"))
    company = try_int(request.form.get("company"))
    votes = try_int(request.form.get("votes"))
    score = try_int(request.form.get("score"))
    crowd_soured = try_bool(request.form.get("crowd_soured"))
    delete_option = request.form.get("delete")

    if not id:
        return send_status(tag_company_create(tag,company,votes,score,crowd_soured))


    success = False
    if delete_option:
        success = tag_company_delete(id)
    else:
        success = tag_company_update_helper(id, tag, company,votes, score, crowd_soured)
    return send_status(success)
