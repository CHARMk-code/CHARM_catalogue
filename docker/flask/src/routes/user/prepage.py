from flask import Blueprint, jsonify
from flask_api import status
from ...models import Prepage

blueprint = Blueprint('prepage', __name__, url_prefix='/api/prepage')

@blueprint.route("", methods=["GET"])
def tags_get():
    """
    Get endpoint /api/prepage
        List Tags - A json list of all prepages.
    """
    prepages = Prepage.query.all()
    return jsonify([prepage.serialize for prepage in prepages]), status.HTTP_200_OK
