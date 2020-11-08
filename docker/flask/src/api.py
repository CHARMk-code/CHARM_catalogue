from flask import Blueprint, request, jsonify
from .models import *
import csv
import sys

api = Blueprint('api', __name__,url_prefix='/api')


@api.route("/tag/add/<company>/<tag>",methods =["POST"])
def tag_add(company,tag):
    new_tag_company = Tag_company(
        tag=tag,
        company=company,
        crowd_soured=True
    )
    db.session.add(new_tag_company)
    db.session.commit()

@api.route("/tag/match")
def tag_match():
    select_tags = request.args.get("tags")
    select_tags =select_tags.translate({ord('['): None})
    select_tags= select_tags.translate({ord(']'): None})
    select_tags =select_tags.split(',')
    select_tags = list(map(int,select_tags))
    companies = db.session.query(Tag_company).filter(Tag_company.tag.in_(select_tags)).with_entities(Tag_company.company).all()
    return jsonify(companies)

@api.route("/tag/get")
def tags_get():
    company_filter = request.args.get("company_filter")
    if company_filter:
        Tag_query = Tag.query.filter_by(company = company_filter)
    else:
        Tag_query = Tag.query
    tags = Tag_query.all()
    return jsonify([tag.serialize for tag in tags])
    
@api.route("/company/get")
def companies_get():
    companies = Company.query.filter_by(active = True).all()
    return jsonify([company.serialize for company in companies])
    

@api.route("/load")
def load():
    with open("tags.csv","r") as csv_file:
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
                    parent_tag = parent_tag
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
            comp_id = Company.query.filter_by(name=reader[i+1][0]).first().id

            for j in range(row_length-1):
                if reader[i+1][j+1] == "TRUE":
                    if not Tag_company.query.filter_by( tag = tags[j],  company = comp_id).first():
                        new_link = Tag_company(
                            tag = tags[j],
                            company = comp_id,
                            crowd_soured = False
                        )
                        db.session.add(new_link)
                        db.session.commit()
