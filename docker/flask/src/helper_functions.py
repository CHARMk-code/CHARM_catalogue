from . import db, config
from .models import *
from werkzeug.security import generate_password_hash
import csv
import datetime
import sys
from flask import render_template
from flask_sqlalchemy import *
from flask_api import status

def send_status(success):
    if success:
        return "Success", status.HTTP_200_OK
    else:
        return "Failed", status.HTTP_500_INTERNAL_SERVER_ERROR

    
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

def tag_update(id, name, parent_tag, crowd_soured):
    tag = Tag.query.get(id)

    if tag:
        tag.name = name
        tag.parent_tag = parent_tag
        tag.crowd_soured = crowd_soured
        db.session.commit()
        return True
    return False

def tag_delete(id):
    tag = Tag.query.get(id)

    if not tag:
        return False

    db.session.delete(tag)
    db.session.commit()

    return True

def tag_company_update(id, tag, company, votes, score, crowd_soured):
    tag_company = Tag_company.query.get(id)

    if tag_company:
        tag_company.tag = tag
        tag_company.company = company
        tag_company.votes = votes
        tag_company.score = score
        tag_company.crowd_soured = crowd_soured
        db.session.commit()
        return True
    return False

def tag_company_delete(id):
    tag_company = Tag_company.query.get(id)

    if not tag_company:
        return False

    db.session.delete(tag_company)
    db.session.commit()

    return True

def company_update(id, name, active, page):
    company = Company.query.get(id)

    if company:
        company.name = name
        company.active = active
        company.page = page
        db.session.commit()
        return True
    return False

def company_delete(id):
    company = Company.query.get(id)

    if not company:
        return False

    db.session.delete(company)
    db.session.commit()

    return True
