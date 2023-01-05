from flask import Blueprint,jsonify, request
from ...models import Company_card
from flask_cors import CORS
from flask_api import status
from ...helper_functions import *
import os

blueprint = Blueprint('settings_admin', __name__, url_prefix='/api/settings')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("company_view",methods=["PUT"])
def company_view_put():
    result = auth_token(request)
    if not result[0]:
        return result[1]

    def create_or_update(one_request_data):
        id = get_if_exist(one_request_data,"id")
        name = get_if_exist(one_request_data,"name")
        text = get_if_exist(one_request_data,"text")
        active = get_if_exist(one_request_data,"active")
        
        if not id:
            return send_status(Company_card.create(name,text,active))

        company_card_object = Company_card.query.get(id)

        return company_card_object.update(name,text,active)

    request_data = request.get_json()
    if isinstance(request_data, list):
        status = all(map(create_or_update, request_data))
    else:
        status = create_or_update(request_data)


    return send_status(status)

    
    

@blueprint.route("company_view/reset",methods=["GET"])
def reset_company_view():
    result = auth_token(request)
    if not result[0]:
        return result[1]

    default_cards = [
        { 'text': "Logo", 'name': "logo", 'active': True },
        { 'text': "Name", 'name': "name", 'active': True },
        { 'text': "Description", 'name': "desc", 'active': True },
        { 'text': "Did you know", 'name': "didyouknow", 'active': True },
        { 'text': "Divisions", 'name': "tag_divisions", 'active': True },
        { 'text': "Business Areas", 'name': "tag_business_areas", 'active': True },
        { 'text': "Offering", 'name': "tag_offering", 'active': True },
        { 'text': "Looking for", 'name': "tag_looking_for", 'active': True },
        { 'text': "Website", 'name': "website", 'active': True },
        { 'text': "Map", 'name': "map", 'active': True },
        { 'text': "Summer job", 'name': "summerjob", 'active': True },
        { 'text': "Notes", 'name': "notes", 'active': True },
        { 'text': "CHARMtalks", 'name': "CHARMtalks", 'active': True },
        { 'text': "Language", 'name': "language", 'active': True },
        { 'text': "Fair Areas", 'name': "fair_area", 'active': True },
    ]
    cards = Company_card.query.all()
    for card in cards:
        card.delete()
    status = map(lambda card: Company_card.create(**card), default_cards)
    return send_status(all(status))


