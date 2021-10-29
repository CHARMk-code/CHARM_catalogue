from . import db,config
from flask_login import UserMixin
import sys
import jwt
from werkzeug.security import generate_password_hash,check_password_hash
import datetime
from .helper_functions import test_and_set


# Crowd:
# 0 - all
# 1 - Only crowd sourced
# 2 - Only manual added

companies_tags = db.Table('companies_tags',
        #  db.Column('id', db.Integer, primary_key=True),
    db.Column('company_id', db.Integer, db.ForeignKey(
        'companies.id'), primary_key=True),
    db.Column('tag_id', db.Integer, db.ForeignKey(
        'tags.id'), primary_key=True),
    db.PrimaryKeyConstraint('company_id', 'tag_id')
    )

class User(UserMixin, db.Model):
    id = db.Column(db.Integer, primary_key=True)
    password = db.Column(db.String(100))

    @staticmethod
    def create(password):
        if len(User.query.all()) == 0:
            new = User(password = generate_password_hash(password,method='sha256'))
            db.session.add(new)
            db.session.commit()
            return new

    def update(self,password):
        self.password = generate_password_hash(password,method='sha256')
        db.session.commit()
        return True

    def delete(self):
        db.session.delete(self)
        db.session.commit()

    def gen_token(self):
        payload = {
            'exp': datetime.datetime.utcnow() + datetime.timedelta(days=1, seconds=0),
            'iat': datetime.datetime.utcnow(),
            'sub': self.id
        }
        return jwt.encode(
            payload,
            config['creds']['secret'],
            algorithm='HS256'
        )

    def authenticate(self, password):
        return check_password_hash(self.password, password)




class Company(db.Model):
    __tablename__ = "companies"
    """
    Reps a company
    """
    id = db.Column(db.Integer, primary_key=True)
    active = db.Column(db.Boolean)
    name = db.Column(db.String(200))
    description = db.Column(db.String(1000))
    business_area = db.Column(db.String(500))
    founded = db.Column(db.Integer)
    contacts = db.Column(db.String(300))
    employees_sweden = db.Column(db.Integer)
    employees_world = db.Column(db.Integer)
    trivia = db.Column(db.String(500))
    website = db.Column(db.String(200))
    logo = db.Column(db.String(100))
    tags = db.relationship(
        'Tag',
        secondary=companies_tags,
        lazy='subquery',
        backref=db.backref('tags', lazy=True, cascade='all, delete')
    )

    @staticmethod
    def create( name, active, description, business_area,
        trivia, founded, contacts, employees_sweden,
        employees_world, website,logo, tags):
        try:
            if Company.query.filter_by(name=name).first():
                return False
            new_company = Company(
                name=name,
                active=active,
                description=description,
                business_area = business_area,
                trivia=trivia,
                founded = founded,
                contacts = contacts,
                employees_sweden = employees_sweden,
                employees_world = employees_world,
                website = website,
                logo = logo,
                tags = tags
            )

            db.session.add(new_company)
            db.session.commit()
        except Exception as e:
            return False
        return True

    def update(self, name, active, description, business_area,
            trivia, founded, contacts, employees_sweden,
            employees_world, website, logo, tags):
        self.name = test_and_set(self.name,name)
        self.active = test_and_set(self.active,active)
        self.description = test_and_set(self.description,description)
        self.business_area = test_and_set(self.business_area,business_area)
        self.trivia = test_and_set(self.trivia,trivia)
        self.founded = test_and_set(self.founded,founded)
        self.contacts = test_and_set(self.contacts,contacts)
        self.employees_sweden = test_and_set(self.employees_sweden,employees_sweden)
        self.employees_world = test_and_set(self.employees_world, employees_world)
        self.website = test_and_set(self.website, website)
        self.logo = test_and_set(self.logo, logo)
        self.tags = test_and_set(self.tags, tags)

        db.session.commit()
        return True

    def delete(self):
        db.session.delete(self)
        db.session.commit()
        return True

    @property
    def serialize(self):
        tags = []
        for tag in self.tags:
            tags.append(tag.id)
        return {
            'id': self.id,
            'name': self.name,
            'active': self.active,
            'description': self.description,
            'business_area': self.business_area,
            'trivia': self.trivia,
            'founded': self.founded,
            'contacts': self.contacts,
            'employees_sweden': self.employees_sweden,
            'employees_world': self.employees_world,
            'website': self.website,
            'logo': self.logo,
            'tags': tags
        }


class Tag(db.Model):
    __tablename__ = "tags"
    """
    Tag represents a buzzword, program, or talent.
    These can be crowd sourced.
    """
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(200))
    parent_tag = db.Column(db.Integer)
    up_votes = db.Column(db.Integer)
    down_votes = db.Column(db.Integer)
    crowd_sourced = db.Column(db.Boolean)

    @staticmethod
    def create(name, parent_tag,up_votes, down_votes, crowd_sourced):
        try:
            if Tag.query.filter_by(name=name).first():
                return False
            new_tag = Tag(
                name=name,
                parent_tag=parent_tag,
                up_votes = up_votes,
                down_votes = down_votes,
                crowd_sourced=crowd_sourced,
            )
            db.session.add(new_tag)
            db.session.commit()
        except:
            return False
        return True

    def update(self,name, parent_tag,up_votes, down_votes, crowd_sourced):
        try:
            self.name = test_and_set(self.name,name)
            self.parent_tag = test_and_set(self.parent_tag,parent_tag)
            self.up_votes = test_and_set(self.up_votes,up_votes)
            self.down_votes = test_and_set(self.down_votes, down_votes)
            self.crowd_sourced = test_and_set(self.crowd_sourced,crowd_sourced)
            db.session.commit()
            return True
        except:
            return False


    def delete(self):
        db.session.delete(self)
        db.session.commit()

        return True

    @property
    def serialize(self):
        return {
            'id': self.id,
            'name': self.name,
            'parent_tag': self.parent_tag,
            'up_votes': self.up_votes,
            'down_votes': self.down_votes,
            'crowd_sourced': self.crowd_sourced,
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
    crowd_sourced = db.Column(db.Boolean)

    @property
    def serialize(self):
        return {
            'id': self.id,
            'tag': self.tag,
            'company': self.company,
            'up_votes': self.up_votes,
            'down_votes': self.down_votes,
            'crowd_sourced': self.crowd_sourced,
        }

    @staticmethod
    def create(tag, company, up_votes, down_votes, crowd_sourced):
        try:
            if Tag_company.query.filter_by(tag=tag,company=company).first():
                return False
            new_tag_company = Tag_company(
                tag=tag,
                company=company,
                up_votes=up_votes,
                down_votes=down_votes,
                crowd_sourced=crowd_sourced
            )

            db.session.add(new_tag_company)
            db.session.commit()
        except:
            return False
        return True

    def update(self, tag, company, up_votes, down_votes, crowd_sourced):
        try:
            self.tag = test_and_set(self.tag, tag)
            self.company = test_and_set(self.company,company)
            self.up_votes= test_and_set(self.up_votes, up_votes)
            self.down_votes= test_and_set(self.down_votes, down_votes)
            self.crowd_sourced = test_and_set(self.crowd_sourced, crowd_sourced)
            db.session.commit()
            return True
        except:
            return False

