from flask import Blueprint, request, url_for
from flask_cors import CORS
from flask_login import login_required, current_user
import csv
import os
from datetime import datetime
from string import punctuation
from ...models import Company, Tag_company, Tag
from flask_api import status
from math import ceil
from time import time
from ... import db
from ...shared_data import last_update_company,last_update_tag
from ...helper_functions import *


blueprint = Blueprint('manage', __name__, url_prefix='/api/manage')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("/load", methods=["POST", "GET"])
# @login_required
def load():
    global last_update_tag, last_update_company
    # last_update_company = ceil(time())
    # last_update_tag = ceil(time())
    
    """
    GET endpoint /management/load

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
                tag_create(row[next_col],parent_tag,1,1,False)
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
            for i in range(2,row_length):
                tags.append(Tag.query.filter_by(name = tag_row[i]).first().id)

        for i in range(1,len(reader)):
            if not Company.query.filter_by(name=reader[i][0]).first():
                new_comp = Company(
                    name = reader[i][0],
                    active = True,
                    page = int(reader[i][1])
                )
                db.session.add(new_comp)
                db.session.commit()
            comp = Company.query.filter_by(name=reader[i][0]).first()
            comp.active = True
            db.session.commit()
            comp_id = comp.id
            for j in range(2,row_length):
                if reader[i][j] == "TRUE":
                    if not Tag_company.query.filter_by( tag = tags[j-2],  company = comp_id).first():
                        new_link = Tag_company(
                            tag = tags[j-2],
                            company = comp_id,
                            crowd_soured = False,
                            score = 1,
                            votes = 1
                        )
                        db.session.add(new_link)
                        db.session.commit()
    return "Success", status.HTTP_200_OK