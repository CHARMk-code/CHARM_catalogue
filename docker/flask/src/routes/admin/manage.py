from flask import Blueprint, send_from_directory, request
from werkzeug.utils import secure_filename
from flask_cors import CORS
import xlrd,os
from ...models import Company,  Tag
from flask_api import status
from ... import db, config
from ...helper_functions import *
import shutil


ACCEPT_IMAGE_EXTENDS = ["jpg","png","svg"]
blueprint = Blueprint('manage', __name__, url_prefix='/api/manage')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("/image/<filename>", methods = ["GET"])
def imageSend(filename):
    return send_from_directory(config['flask']['upload_folder'], secure_filename(filename))


def imageLoad(request):
    file = request.files["file"]
    filename = file.filename
    if not filename[-3:] in ACCEPT_IMAGE_EXTENDS:
        return f'{filename} is not accept file type', status.HTTP_400_BAD_REQUEST
    file.save(os.path.join(config['flask']['upload_folder'], secure_filename(filename)))

    return "All files uploaded", status.HTTP_200_OK

def parseXlsx():
    NUMBER_OF_METADATA_COLS = 11

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
        for i in range(NUMBER_OF_METADATA_COLS,companies_sheet.ncols):
            tags.append(Tag.query.filter_by(name = tag_row[i].value).first())

        for i in range(1,companies_sheet.nrows):
            if not Company.query.filter_by(name=companies_sheet.cell_value(i,0)).first():
                tags_temp = []
                for j in range(NUMBER_OF_METADATA_COLS,companies_sheet.ncols):
                    if companies_sheet.cell_value(i,j):
                        tags_temp.append(tags[j-NUMBER_OF_METADATA_COLS])

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
                        companies_sheet.cell_value(i,10), # logo
                        tags_temp
                        )
    os.remove("/catalogue/CHARM_CATALOGUE_DATA.xlsx")

def unpackAndParse(request):
    file = request.files["file"]
    filename = file.filename
    file_extension = filename[filename.index("."):]
    packedPath = os.path.join(config['flask']['upload_folder'], f"tmp{file_extension}" )
    file.save(packedPath)

    unpackedPath = os.path.join(config['flask']['upload_folder'], "tmp")
    shutil.unpack_archive(packedPath,unpackedPath)
    os.remove(packedPath)

    for root, dirs, files in os.walk(unpackedPath):
        for file in files:
            path = os.path.join(root,file)
            if ".xlsx" in file:
                os.rename(path, "/catalogue/CHARM_CATALOGUE_DATA.xlsx")
                parseXlsx()
            elif any(map(lambda x: x in file, ACCEPT_IMAGE_EXTENDS)):
                os.rename(path,os.path.join(config['flask']['upload_folder'],file))
    shutil.rmtree(unpackedPath)
    return "", status.HTTP_200_OK

@blueprint.route("/upload", methods=["POST"])
def load():
    """
    POST endpoint api/manage/load

    Allow for for uploading of images and data, see README
    """
    result = auth_token(request)
    if not result[0]:
        return result[1]

    if "file" not in request.files:
        return "No file found",status.HTTP_400_BAD_REQUEST

    file = request.files["file"]

    if ".xlsx" in file.filename: # load data
        request.files[file.filename].save("/catalogue/CHARM_CATALOGUE_DATA.xlsx")
        parseXlsx()
    elif any(map(lambda x: x in file.filename, ACCEPT_IMAGE_EXTENDS)): # Load single image
        imageLoad(request)
    elif any(map(lambda x:x in file.filename, [".zip", ".tar.gz"])):
            unpackAndParse(request)
    else:
        return f"{file.filename} unaccepted file", status.HTTP_400_BAD_REQUEST
    return "Success", status.HTTP_201_CREATED


