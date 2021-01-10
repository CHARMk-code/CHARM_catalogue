from flask import Blueprint, request, jsonify
from flask_api import status
from flask_cors import CORS
from math import ceil
from time import time
from ... import db
from ...models import Tag, Tag_company, Company
import sys
from ...shared_data import last_update_company, last_update_tag
from ...helper_functions import tag_create, tag_company_create, try_int

blueprint = Blueprint('tag', __name__, url_prefix='/api/tag') 
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("/create", methods=["POST"])
def tag_add():
    global last_update_tag
    last_update_tag = ceil(time())
    """
    POST endpoint /api/tag/create

    Args:
        tag - the name of the new tag
        parent_tag - id of the parent 
    """
    tag = request.form.get("tag")
    tag_obj = Tag.query.filter_by(name = tag).first()
    parent_tag = try_int(request.form.get("parent_tag"))
    if not tag_obj:
        tag_create(tag, parent_tag, 1, 1,True)
    else:
        vote = request.form.get("vote")
        if vote == "up":
            tag_obj.score +=1
            tag_obj.votes +=1
        elif vote == "down":
            tag_obj.votes +=1
        db.session.commit()
    return "success", status.HTTP_200_OK

@blueprint.route("/add", methods=["POST"])
def tag_company_add():
    global last_update_tag
    last_update_tag = ceil(time())
    """
    POST endpoint /api/tag/add

    If relation doesn't exist, it's creates. If the relation exist it's handle as a vote, if no vote args is supplied then it's ignored.

    Args:
        tag - id of the tag
        company -  id of the company
        optional vote - [up, down] used to cast vote, base if the user agree with the relation.
    """
    tag = try_int(request.form.get("tag"))
    company = try_int(request.form.get("company"))
    tag_company = Tag_company.query.filter_by(tag=tag, company =company).first()
    if not tag_company: # Create new 
        tag_company_create(tag,company,1,1,True)
    else:
        vote = request.form.get("vote")
        if vote == "up":
            tag_company.score +=1
            tag_company.votes +=1
        elif vote == "down":
            tag_company.votes +=1

    db.session.commit()
    return "success", status.HTTP_200_OK


@blueprint.route("/match", methods=["GET"])
def tag_match():
    """
    Get endpoint /api/tag/match
    
    Args:
        select_tags - list of id of all selected tags
        optional crowd - int 0 - 2 specifing crowd sourcing option. Key:
        0 - all tags
        1 - Only crowd sourced tags
        2 - Only non crowd sourced tags

    Return:
        List (company id, votes, score) 

    """
    select_tags = request.form.get("tags")
    
    crowd = 0
    if request.form.get("crowd"):
        crowd = try_int(request.form.get("crowd"))
        if crowd > 2:
            return status.HTTP_400_BAD_REQUEST
    select_tags =select_tags.translate({ord('['): None})
    select_tags= select_tags.translate({ord(']'): None})
    select_tags =select_tags.split(',')
    select_tags = filter(lambda a: a != '', select_tags)
    select_tags = list(map(int,select_tags))
    company_query = db.session.query(Tag_company).filter(Tag_company.tag.in_(select_tags))
    if crowd == 0:
        companies = company_query.with_entities(Tag_company.company, Tag_company.votes, Tag_company.score).all()
    else:
        crowd = (1==crowd)
        companies = company_query.filter_by(crowd_soured = crowd).with_entities(Tag_company.company, Tag_company.votes, Tag_company.score).all()
    result = []
    with db.session.no_autoflush:
        for company in companies:
            if Company.query.filter_by(id = company[0], active = True).first():
                result.append(company)
    return jsonify(result), status.HTTP_200_OK

@blueprint.route("/get", methods=["GET"])
def tags_get():
    global last_update_tag
    """
    Get endpoint /api/tag/get

    args:
        optional company_filter - id of a company, will only return tag relation to said company
        optional crowd - int 0 - 2 specifing crowd sourcing option. Key:
        0 - all tags
        1 - Only crowd sourced tags
        2 - Only non crowd sourced tags
        optional timestamp - if not None will return the timestamp when the data was updated

    return:
        List Tags - A json list of all tags that match the optional args.
    """
    company_filter = request.form.get("company_filter")
    only_ids = request.form.get("only_ids")
    timestamp = request.form.get("timestamp")
    if timestamp:
        return jsonify(last_update_tag)
    if request.form.get("crowd"):
        crowd = try_int(request.form.get("crowd"))
        if crowd > 2:
            return status.HTTP_401_UNAUTHORIZED
    crowd = 0
    if company_filter:
        t = db.session.query(
            Tag_company.tag,
        ).filter(Tag_company.company == int(company_filter)).group_by(Tag_company.tag).subquery('t')

        Tag_query = Tag.query.filter(
            Tag.id == t.c.tag
        )
    else:
        Tag_query = Tag.query
    if crowd != 0:        
        crowd = (1==crowd)
        Tag_query = Tag_query.filter_by(crowd_soured = crowd)
    tags = Tag_query.all()
    if only_ids:
        return jsonify([tag.id for tag in tags]), status.HTTP_200_OK
    else:
        return jsonify([tag.serialize for tag in tags]), status.HTTP_200_OK
