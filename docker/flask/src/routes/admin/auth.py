from sys import stderr
from flask import Blueprint, request, jsonify
from werkzeug.security import check_password_hash
from flask_login import login_user,  logout_user, login_required, current_user
import datetime
import jwt
from ...models import User
from werkzeug.security import check_password_hash
from ... import config
from flask_api import status
from ...helper_functions import get_if_exist

blueprint = Blueprint('auth', __name__, url_prefix='/api/auth')

@blueprint.route('/login', methods=['POST'])
def login_post():
    """
    POST endpoint /api/auth/login

    Auths user

    Arguments:
        email - supplied in form
        password - supplied in form
        remember - supplied in form

    Return:
        sesssion cookie

    """
    request_data = request.get_json()
    password = get_if_exist(request_data,'password')
    # take the user supplied password, hash it, and compare it to the hashed password in database
    # Should use hashed password if scaling, this is only acceptable for now.
    if config["creds"]["password"] != password:
        return 'Please check your login details and try again.', status.HTTP_401_UNAUTHORIZED
    # if the above check passes, then we know the user has the right credentials
    payload = {
        'exp': datetime.datetime.utcnow() + datetime.timedelta(days=1, seconds=0),
        'iat': datetime.datetime.utcnow(),
        'sub': 1
    }

    return jsonify({"token": jwt.encode(
            payload,
            config["creds"]['secret'],
            algorithm='HS256'
        )
})


@blueprint.route('/logout')
@login_required
def logout():
    """ Logout current user. """
    logout_user()
    return "Success", status.HTTP_200_OK
