from flask import Blueprint, request
from ..models import Company, Tag, Layout, Prepage, Shortcut, Company_card, Map, createGeneric, deleteGeneric, updateGeneric
from flask_cors import CORS
from ..helper_functions import *

blueprint = Blueprint('admin', __name__, url_prefix='/api/')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])



path_LUT = {
"company": Company,
"layout": Layout,
"map": Map,
"prepage": Prepage,
"shortcut": Shortcut,
"settings": Company_card,
"tag": Tag
}

@blueprint.route("<path>", methods=["PUT"])
def generic_put(path):
    result = auth_token(request)
    if not result[0]:
        return result[1]


    table = path_LUT[path]
    
    request_data = request.get_json()
    values = []
    for attr in table.__table__.columns.keys():
        values.append(get_if_exist(request_data,attr))

    if path == "company":
        tags = get_if_exist(request_data, "tags")
        tag_objs=[]
        if tags:
            for tag in tags:
                tag_objs.append(Tag.query.get(tag))
        values.append(tag_objs)

    id = values.pop(0)
    if not id:
        return send_status(createGeneric(table, values))
    else:
        row = table.query.get(int(id))
        return send_status(updateGeneric(table, row, values))

@blueprint.route("<path>/<id>",methods=["DELETE"])
def company_delete(path,id):
    result = auth_token(request)
    if not result[0]:
        return result[1]

    table = path_LUT[path]
    row = table.query.get(id)
    if row:
        return send_status(deleteGeneric(row))
    else:
        return "Failed", status.HTTP_400_BAD_REQUEST