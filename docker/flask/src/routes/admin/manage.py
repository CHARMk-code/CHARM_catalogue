from flask import Blueprint, request, url_for
from flask_cors import CORS
from flask_login import login_required, current_user
import csv
import xlrd
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
    # Inactives company
    Company.query.update({Company.active:False})
    db.session.commit()

    workbook = xlrd.open_workbook("CHARM_CATALOGUE_DATA.xlsx")


    # Adds tags
    tags_sheet = workbook.sheet_by_name("Tags")

    next_col = 0
    parent_tag = None
    for i in range(tags_sheet.nrows):
        row = tags_sheet.row(i)
        tag = Tag.query.filter_by(name=row[next_col].value).first()
        if not tag:
            Tag.create(row[next_col].value,parent_tag,1,1,False)
            parent_tag = Tag.query.filter_by(name=row[next_col].value).first().id
        else:
            parent_tag = tag.id

        if (i+1 >= tags_sheet.nrows):
            break
        if (tags_sheet.cell_value(i+1,next_col)!=''):
            if (next_col==0):
                parent_tag = None
            continue
        elif (next_col+1 < tags_sheet.ncols):
            if (tags_sheet.cell_value(i+1,next_col+1)!=''):
                next_col += 1
                continue

        for j in range(tags_sheet.ncols):
            if tags_sheet.cell_value(i+1,j) != '':
                if (j==0):
                    parent_tag = None
                next_col = j
                break

    companies_sheet = workbook.sheet_by_name("Companies")
    tags = []

    # Generats tags
    tag_row = companies_sheet.row(0)
    with db.session.no_autoflush:
        for i in range(10,companies_sheet.ncols):
            tags.append(Tag.query.filter_by(name = tag_row[i].value).first())

        for i in range(1,companies_sheet.nrows):
            if not Company.query.filter_by(name=companies_sheet.cell_value(i,0)).first():
                tags_temp = []
                for j in range(10,companies_sheet.ncols):
                    if companies_sheet.cell_value(i,j):
                        tags_temp.append(tags[j-10])

                        # Tempary removed user supplied tag company connection and ratings
                        #  if not Tag_company.query.filter_by( tag = tags[j-2],  company = comp_id).first():
                        #      new_link = Tag_company(
                        #          tag = tags[j-2],
                        #          company = comp_id,
                        #          crowd_soured = False,
                        #          score = 1,
                        #          votes = 1
                        #      )
                        #      db.session.add(new_link)
                        #      db.session.commit()

                Company.create(
                        companies_sheet.cell_value(i,0), # name
                        try_bool(companies_sheet.cell_value(i,1)), # Active
                        companies_sheet.cell_value(i,2), # Description
                        companies_sheet.cell_value(i,3), # Busniess area
                        companies_sheet.cell_value(i,4), # Trivia
                        try_int(companies_sheet.cell_value(i,5)), # Founded
                        companies_sheet.cell_value(i,6), # Contacts
                        try_int(companies_sheet.cell_value(i,7)), # Employs Sweden
                        try_int(companies_sheet.cell_value(i,8)), # Employs world
                        companies_sheet.cell_value(i,9), # Website
                        tags_temp
                        )
    return "Success", status.HTTP_200_OK
