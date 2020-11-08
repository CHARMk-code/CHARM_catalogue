from . import db
from flask_login import UserMixin

class User(UserMixin, db.Model):
    id = db.Column(db.Integer, primary_key=True)
    privilege = db.Column(db.Integer)

    @property
    def serialize(self):
        return {
            'id': self.id,
            'privilege': self.privilege,
        }

class Company(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(200))
    active = db.Column(db.Boolean)

    @property
    def serialize(self):
        return {
            'id': self.id,
            'name': self.name,
            'active': self.active,
        }


class Tag(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(200))
    parent_tag = db.Column(db.Integer)

    @property
    def serialize(self):
        return {
            'id': self.id,
            'name': self.name,
            'parent_tag': self.parent_tag,
        }

class Tag_company(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    tag = db.Column(db.Integer)
    company = db.Column(db.Integer)
    crowd_soured = db.Column(db.Boolean)

    @property
    def serialize(self):
        return {
            'id': self.id,
            'tag': self.tag,
            'company': self.company,
            'crowd_soured': self.crowd_soured,
        }
