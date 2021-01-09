from . import db
from flask_login import UserMixin

# Crowd:
# 0 - all
# 1 - Only crowd sourced
# 2 - Only manual added


class User(UserMixin, db.Model):
    id = db.Column(db.Integer, primary_key=True)
    email = db.Column(db.String(100), unique=True)
    password = db.Column(db.String(100))
    name = db.Column(db.String(1000))
    number = db.Column(db.String(20))
    privilege = db.Column(db.Integer)

    @property
    def serialize(self):
        return {
            'id': self.id,
            'email': self.email,
            'password': self.password,
            'name': self.name,
            'number': self.number,
            'privilege': self.privilege,
        }

class Company(db.Model):
    """
    Reps a company
    """ 
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(200))
    active = db.Column(db.Boolean)
    page = db.Column(db.Integer)

    @property
    def serialize(self):
        return {
            'id': self.id,
            'name': self.name,
            'active': self.active,
            'page': self.page
        }


class Tag(db.Model):
    """
    Tag represents a buzzword, program, or talent.
    These can be crowd sourced.
    """
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(200))
    parent_tag = db.Column(db.Integer)
    crowd_soured = db.Column(db.Boolean)

    @property
    def serialize(self):
        return {
            'id': self.id,
            'name': self.name,
            'parent_tag': self.parent_tag,
            'crowd_soured': self.crowd_soured,
        }

class Tag_company(db.Model):
    """
    These are the reletion between a company and tag.
    These can be crowd source, it supports voting to improve the accuracy of the relation.
    """
    id = db.Column(db.Integer, primary_key=True)
    tag = db.Column(db.Integer)
    company = db.Column(db.Integer)
    votes = db.Column(db.Integer)
    score = db.Column(db.Integer)
    crowd_soured = db.Column(db.Boolean)

    @property
    def serialize(self):
        return {
            'id': self.id,
            'tag': self.tag,
            'company': self.company,
            'votes': self.votes,
            'score': self.score,
            'crowd_soured': self.crowd_soured,
        }
