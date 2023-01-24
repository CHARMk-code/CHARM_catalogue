from flask import Blueprint, request, jsonify
from ...models import Feedback
from flask_cors import CORS
from ...helper_functions import *

blueprint = Blueprint('feedback_admin', __name__, url_prefix='/api/feedback')
CORS(blueprint,origins="*", resources=r'*', allow_headers=[
    "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])


@blueprint.route("", methods=["GET"])
def feedback_get():
    result = auth_token(request)
    if not result[0]:
        return result[1]

    feedbacks = Feedback.query.all()
    return jsonify([feedback.serialize for feedback in feedbacks]), status.HTTP_200_OK


@blueprint.route("", methods=["POST"])
def feedback_post():
    result = auth_token(request)
    if not result[0]:
        return result[1]

    request_data = request.get_json()
    id = try_int(get_if_exist(request_data, "id"))

    important = try_bool(get_if_exist(request_data, "important"))
    archived = try_bool(get_if_exist(request_data,"archived"))

    feedback = Feedback.query.get(id)

    return send_status(feedback.update(None,None,None,important, archived))

@blueprint.route("<id>",methods=["DELETE"])
def feedback_delete(id):
    result = auth_token(request)
    if not result[0]:
        return result[1]


    feedback_object = Feedback.query.get(id)
    if feedback_object:
        return send_status(feedback_object.delete())
    else:
        return "Failed", status.HTTP_400_BAD_REQUEST
