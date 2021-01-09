from . import db
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

def try_int(value):
    try:
        return int(value)
    except:
        return None

def try_bool(value):
    return value == "True"
    
def create_user(email,name,password,number,privilege):
    try:
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
    except:
        return False
    return True

def tag_create(name, parent_tag, crowd_soured):
    try:
        if Tag.query.filter_by(name=name).first():
            return False
        new_tag = Tag(
            name=name,
            parent_tag=parent_tag,
            crowd_soured=crowd_soured,
        )
        db.session.add(new_tag)
        db.session.commit()
    except:
        return False
    return True

def tag_update_helper(id, name, parent_tag, crowd_soured):
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

def tag_company_create(tag, company, votes, score, crowd_soured):
    try:
        if Tag_company.query.filter_by(tag=tag,company=company).first():
            return False
        new_tag_company = Tag_company(
            tag=tag,
            company=company,
            votes=votes,
            score=score,
            crowd_soured=crowd_soured
        )

        db.session.add(new_tag_company)
        db.session.commit()
    except:
        return False
    return True

def tag_company_update_helper(id, tag, company, votes, score, crowd_soured):
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


def company_create(name, active, page):
    try:
        if Company.query.filter_by(name=name).first():
            return False
        new_company = Company(
            name=name,
            active=active,
            page=page
        )

        db.session.add(new_company)
        db.session.commit()
    except:
        return False
    return True

def company_update_helper(id, name, active, page):
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
