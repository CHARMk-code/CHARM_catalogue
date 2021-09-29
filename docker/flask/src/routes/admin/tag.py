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
    """
    POST endpoint /api/tag/update

    Arg:
        id(int) - Id of the tag, if no id provided creates new object
        name(string) - Name of the tag (if not provided the value will be set to None)
        parent_tag(int) - Id of parent tag (if not provided the value will be set to None)
        votes(int) - Total number of votes cast on this tag. Must be greater than 1.
        score(int) - Total number of upvotes.
        crowd_sourced(bool) - Set if the tag, was created by an non authed user.
        delete_option(bool) - If set, deletes tag_company with the given id
    Return:
        200_ok
        500_internal_server_error

    Design choose, I choose to combine create/update/delete
    to one endpoint to make it more lean and I'm lazy
    """
    id = try_int(request.form.get("id"))
    name = request.form.get("name")
    parent_tag = try_int(request.form.get("parent_tag"))
    votes = try_int(request.form.get("votes"))
    score = try_int(request.form.get("score"))
    crowd_soured = try_bool(request.form.get("crowd_soured"))
    delete_option = request.form.get("delete")

    if not id:
        return send_status(tag_create(name,parent_tag,votes,score,crowd_soured))

    success = False
    if delete_option:
        success = tag_delete(id)
    else:
        success = tag_update_helper(id, name, parent_tag, votes, score, crowd_soured)
    return send_status(success)

@blueprint.route("/company/update", methods=["POST"])
def tag_company_update():
    """
    POST endpoint /api/tag/company/update

    Arg:
        id(int) - Id of the tag_company, if no id provided creates new object
        tag(int) - Id of tag
        company(int) - Id of company
        votes(int) - Total number of votes cast on this tag. Must be greater than 1.
        score(int) - Total number of upvotes.
        crowd_sourced(bool) - Set if the tag, was created by an non authed user.
        delete_option(bool) - If set, deletes tag_company with the given id
    Return:
        200_ok
        500_internal_server_error

    Design choose, I choose to combine create/update/delete
    to one endpoint to make it more lean and I'm lazy
    """
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
