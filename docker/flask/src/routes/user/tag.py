from flask import Blueprint, request, jsonify
from flask_api import status
from flask_cors import CORS
from ... import db
from ...models import Tag, Tag_company
from ...helper_functions import try_int, get_if_exist
import sys

blueprint = Blueprint('tag', __name__, url_prefix='/api/tag')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("", methods=["POST"])
def tag_add():

    """
    POST endpoint /api/tag/create

    Args:
        tag - the name of the new tag
        parent_tag - id of the parent
    """
    request_data = request.get_json()
    tag = get_if_exist(request_data, "tag")
    tag_obj = Tag.query.filter_by(name = tag).first()
    parent_tag = get_if_exist(request_data,"parent_tag")
    if not tag_obj:
        Tag.create(tag, parent_tag, 1, 1,True)
    else:
        vote = request.form.get("vote")
        if vote == "up":
            tag_obj.up_votes +=1
        elif vote == "down":
            tag_obj.down_votes +=1
        db.session.commit()
    return "success", status.HTTP_200_OK

# NO LONGER USED
#  @blueprint.route("/company", methods=["POST"])
#  def tag_company_add():
#      """
#      POST endpoint /api/tag/add
#
#      If relation doesn't exist, it's creates. If the relation exist it's handle as a vote, if no vote args is supplied then it's ignored.
#
#      Args:
#          tag - id of the tag
#          company -  id of the company
#          optional vote - [up, down] used to cast vote, base if the user agree with the relation.
#      """
#      tag = try_int(request.form.get("tag"))
#      company = try_int(request.form.get("company"))
#      tag_company = Tag_company.query.filter_by(tag=tag, company =company).first()
#      if not tag_company: # Create new
#          tag_company_create(tag,company,1,1,True)
#      else:
#          vote = request.form.get("vote")
#          if vote == "up":
#              tag_company.score +=1
#              tag_company.votes +=1
#          elif vote == "down":
#              tag_company.votes +=1
#
#      db.session.commit()
    #  return "success", status.HTTP_200_OK


#  @blueprint.route("/match", methods=["GET"])
#  def tag_match():
#      """
#      Get endpoint /api/tag/match
#
#      Args:
#          select_tags - list of id of all selected tags
#          optional crowd - int 0 - 2 specifing crowd sourcing option. Key:
#          0 - all tags
#          1 - Only crowd sourced tags
#          2 - Only non crowd sourced tags
#
#      Return:
#          List (company id, votes, score)
#
#      """
#      select_tags = request.form.get("tags")
#
#      crowd = 0
#      if request.form.get("crowd"):
#          crowd = try_int(request.form.get("crowd"))
#          if crowd > 2:
#              return status.HTTP_400_BAD_REQUEST
#      select_tags =select_tags.translate({ord('['): None})
#      select_tags= select_tags.translate({ord(']'): None})
#      select_tags =select_tags.split(',')
#      select_tags = filter(lambda a: a != '', select_tags)
#      select_tags = list(map(int,select_tags))
#      company_query = db.session.query(Tag_company).filter(Tag_company.tag.in_(select_tags))
#      if crowd == 0:
#          companies = company_query.with_entities(Tag_company.company, Tag_company.votes, Tag_company.score).all()
#      else:
#          crowd = (1==crowd)
#          companies = company_query.filter_by(crowd_soured = crowd).with_entities(Tag_company.company, Tag_company.votes, Tag_company.score).all()
#      result = []
#      with db.session.no_autoflush:
#          for company in companies:
#              if Company.query.filter_by(id = company[0], active = True).first():
#                  result.append(company)
#      return jsonify(result), status.HTTP_200_OK

@blueprint.route("", methods=["GET"])
def tags_get():
    """
    Get endpoint /api/tag

    args:
        optional company_filter(int) - id of a company, will only return tag relation to said company
        optional crowd(int) - 0 - 2 specifing crowd sourcing option. Key:
        0 - all tags
        1 - Only crowd sourced tags
        2 - Only non crowd sourced tags
        optional only_ids - if set only returns ids of tags
    return:
        List Tags - A json list of all tags that match the optional args.
    """
    request_data = request.get_json()
    company_filter = get_if_exist(request_data, "company_filter")
    only_ids = get_if_exist(request_data,"only_ids")
    crowd = get_if_exist(request_data, "crowd")
    if crowd:
        if crowd > 2:
            return status.HTTP_400_BAD_REQUEST
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
