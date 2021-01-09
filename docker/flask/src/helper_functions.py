from . import db, config
from .models import *
from werkzeug.security import generate_password_hash
import csv
import datetime
import sys
from flask import render_template
from flask_sqlalchemy import *

def create_user(email,name,password,number,privilege):
    # create new user with the form data. Hash the password so plaintext version isn't saved.
    new_user = User(
        email=email,
        name=name,
        password=generate_password_hash(password, method='sha256'),
        number=number,
        privilege=privilege,
    )

    # add the new user to the database
    db.session.add(new_user)
    db.session.commit()