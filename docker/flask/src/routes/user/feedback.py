from flask import Blueprint, request
from ...models import Feedback
from flask_cors import CORS
from flask_api import status
from ...helper_functions import *

blueprint = Blueprint('feedback', __name__, url_prefix='/api/feedback')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

@blueprint.route("",methods=["PUT"])
def feedback_put():
    request_data = request.get_json()

    title = get_if_exist(request_data, "title")
    text = get_if_exist(request_data, "text")
    meta = get_if_exist(request_data, "meta")

    return send_status(Feedback.create(title, text, meta))
