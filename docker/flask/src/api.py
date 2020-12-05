from flask import Blueprint, request, jsonify
from .models import *
import csv
import sys
from flask_cors import CORS

api = Blueprint('api', __name__,url_prefix='/api')
CORS(api,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

# Tag
@api.route("/tag/create")
def tag_add():
    """
    POST endpoint /api/tag/create

    Args:
        tag - the name of the new tag
        parent_tag - id of the parent 
    """
    tag = request.args.get("tag")
    parent_tag = request.args.get("parent_tag")
    if not parent_tag:
        # if there is no parent_tag, we will create a new top level tag
        parent_tag = None
    if not Tag.query.filter_by(name = tag).first():
        new_tag = Tag(
            name=tag,
            parent_tag=parent_tag,
            crowd_soured=True
        )
        db.session.add(new_tag)
        db.session.commit()
    return "success"

@api.route("/tag/add")
def tag_company_add():
    """
    POST endpoint /api/tag/add

    If relation doesn't exist, it's creates. If the relation exist it's handle as a vote, if no vote args is supplied then it's ignored.

    Args:
        tag - id of the tag
        company -  id of the company
        optional vote - [up, down] used to cast vote, base if the user agree with the relation.
    """
    tag = int(request.args.get("tag"))
    company = int(request.args.get("company"))
    tag_company = Tag_company.query.filter_by(tag=tag, company =company).first()
    if not tag_company: # Create new 
        new_tag_company = Tag_company(
            tag=tag,
            company=company,
            crowd_soured=True,
            score = 1,
            votes = 1
        )
        db.session.add(new_tag_company)
    else:
        vote = request.args.get("vote")
        if vote == "up":
            tag_company.score +=1
            tag_company.votes +=1
        elif vote == "down":
            tag_company.votes +=1

    db.session.commit()
    return "success"


@api.route("/tag/match")
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
    select_tags = request.args.get("tags")
    crowd = 0
    if request.args.get("crowd"):
        crowd = int(request.args.get("crowd"))
        if crowd > 2:
            return
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
    return jsonify(result)

@api.route("/tag/get")
def tags_get():
    """
    Get endpoint /api/tag/get

    args:
        optional company_filter - id of a company, will only return tag relation to said company
        optional crowd - int 0 - 2 specifing crowd sourcing option. Key:
        0 - all tags
        1 - Only crowd sourced tags
        2 - Only non crowd sourced tags

    return:
        List Tags - A json list of all tags that match the optional args.
    """
    company_filter = request.args.get("company_filter")
    crowd = 0
    if request.args.get("crowd"):
        crowd = int(request.args.get("crowd"))
        if crowd > 2:
            return
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
    return jsonify([tag.serialize for tag in tags])

# Company
@api.route("/company/get")
def companies_get():
    """
    Get endpoint api/company/get
 
    Return:
        List Companys - A json list of all active companies.
    """
    companies = Company.query.filter_by(active = True).all()
    return jsonify([company.serialize for company in companies])
    
# Misc
@api.route("/load")
def load():
    """
    GET endpoint /api/load

    This such be moved to behind authentication.

    When called fills company, tag, tag_company from docker/flask/src/tags.csv & docker/flask/src/data.csv.
    """
    with open("tags.csv","r") as csv_file:
        Company.query.update({Company.active:False})
        db.session.commit()
        reader = list(csv.reader(csv_file, delimiter=';', quotechar='|'))
        row_length = len(reader[0])
        next_col = 0
        parent_tag = None
        for i in range(len(reader)):
            row = reader[i]
            tag = Tag.query.filter_by(name=row[next_col]).first()
            if not tag:
                new_tag = Tag(
                    name = row[next_col],
                    parent_tag = parent_tag,
                    crowd_soured = False
                )
                db.session.add(new_tag)
                db.session.commit()
                parent_tag = Tag.query.filter_by(name=row[next_col]).first().id
            else:
                parent_tag = tag.id

            if (i+1 >= len(reader)):
                break
            if (reader[i+1][next_col]!=''):
                if (next_col==0):
                    parent_tag = None
                continue
            elif (next_col+1 < row_length):
                if (reader[i+1][next_col+1]!=''):
                    next_col += 1
                    continue
        
            for j in range(row_length):
                if reader[i+1][j] != '':
                    if (j==0):
                        parent_tag = None
                    next_col = j 
                    break
                
                
            

    with open("data.csv","r") as csv_file:
        reader = list(csv.reader(csv_file, delimiter=';', quotechar='|'))
        tags = []


        # Generats tags
        tag_row = reader[0]
        row_length = len(tag_row)
        with db.session.no_autoflush:
            for i in range (row_length -1):
                tags.append(Tag.query.filter_by(name = tag_row[i+1]).first().id)
        
        for i in range(len(reader) -1):
            if not Company.query.filter_by(name=reader[i+1][0]).first():
                new_comp = Company(
                    name = reader[i+1][0],
                    active = True
                )
                db.session.add(new_comp)
                db.session.commit()
            comp = Company.query.filter_by(name=reader[i+1][0]).first()
            comp.active = True
            db.session.commit()
            comp_id = comp.id
            for j in range(row_length-1):
                if reader[i+1][j+1] == "TRUE":
                    if not Tag_company.query.filter_by( tag = tags[j],  company = comp_id).first():
                        new_link = Tag_company(
                            tag = tags[j],
                            company = comp_id,
                            crowd_soured = False,
                            score = 1,
                            votes = 1
                        )
                        db.session.add(new_link)
                        db.session.commit()
