from flask import Blueprint, jsonify
from flask_api import status
from ...models import Prepage

blueprint = Blueprint('prepage', __name__, url_prefix='/api/prepage')

@blueprint.route("", methods=["GET"])
def prepages_get():
    """
    Get endpoint /api/prepage
        List Prepages - A json list of all prepages.
    """
    prepages = Prepage.query.all()
    return jsonify([prepage.serialize for prepage in prepages]), status.HTTP_200_OK
