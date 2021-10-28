from flask import Blueprint, request
from ...models import User
from ... import config
from flask_api import status
from ...helper_functions import get_if_exist, send_status, auth_token

blueprint = Blueprint('auth', __name__, url_prefix='/api/auth')

@blueprint.route('/login', methods=['POST'])
def login_post():
    """
    POST endpoint /api/auth/login

    Auths user

    Arguments:
        password - supplied in form
    Return:
        sesssion cookie

    """
    User.create(config["creds"]["password"])
    request_data = request.get_json()
    password = get_if_exist(request_data,'password')

    users = User.query.all()
    user = None
    if len(users) > 0:
        user = users[0]
    else:
        return 'No user exists', status.HTTP_500_INTERNAL_SERVER_ERROR

    if not user.authenticate(password):
        return 'Please check your login details and try again.', status.HTTP_401_UNAUTHORIZED

    return user.gen_token()

@blueprint.route('/update', methods=['POST'])
def update_post():
    result = auth_token(request)
    if not result[0]:
        return result[1]
    request_data = request.get_json()
    delete_option = get_if_exist(request_data,"delete")
    password = get_if_exist(request_data, "password")

    users = User.query.all()
    user = None
    if len(users) > 0:
        user = users[0]
    else:
        return 'No user exists', status.HTTP_500_INTERNAL_SERVER_ERROR

    success = False
    if delete_option:
        success = user.delete()
    else:
        success = user.update(password)
    return send_status(success)
