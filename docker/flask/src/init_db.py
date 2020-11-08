from . import db, create_app
from .models import User
from .helper_functions import create_user
db.create_all(app=create_app())