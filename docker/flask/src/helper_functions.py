from . import config
from flask_api import status
import jwt

def send_status(success):
    if success:
        return "Success", status.HTTP_200_OK
    else:
        return "Failed", status.HTTP_500_INTERNAL_SERVER_ERROR

def try_int(value):
    try:
        return int(value)
    except:
        return None

def try_bool(value):
    return (type(value) == str and (value.lower() == "true")) or \
           (type(value) == bool and value) or \
           (type(value) == int and value != 0)

def get_if_exist(data,key):
    try:
        return data[key]
    except:
        return None

def test_and_set(field, data):
    if not data and (data != 0 or data != False):
        return field
    else:
        if data == "None":
            return None
        return data

def auth_token(request):
    auth_header = request.headers.get('Authorization')
    if auth_header:
        auth_token = auth_header.split(" ")[1]
    else:
        return (False, ('None, token supplied.', status.HTTP_401_UNAUTHORIZED))
    try:
        payload = jwt.decode(auth_token, config['creds']['secret'],'HS256')
        return (True, '')
    except jwt.ExpiredSignatureError:
        return (False,('Signature expired. Please log in again.', status.HTTP_401_UNAUTHORIZED))
    except jwt.InvalidTokenError:
        return (False, ('Invalid token. Please log in again.', status.HTTP_401_UNAUTHORIZED))
