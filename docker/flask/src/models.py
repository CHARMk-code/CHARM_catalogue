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
    last_updated = db.Column(db.DateTime)
    active = db.Column(db.Boolean)
    charmtalk = db.Column(db.Boolean)
    name = db.Column(db.String(200))
    description = db.Column(db.String(1000))
    founded = db.Column(db.Integer)
    contacts = db.Column(db.String(300))
    employees_sweden = db.Column(db.Integer)
    employees_world = db.Column(db.Integer)
    trivia = db.Column(db.String(500))
    website = db.Column(db.String(200))
    talk_to_us_about = db.Column(db.String(500))
    logo = db.Column(db.String(100))
    tags = db.relationship(
        'Tag',
        secondary=companies_tags,
        lazy='subquery',
        backref=db.backref('tags', lazy=True, cascade='all, delete')
    )

    @staticmethod
    def create( name, active, charmtalk, description,
        trivia, founded, contacts, employees_sweden,
        employees_world, website, talk_to_us_about, logo, tags):
        try:
            if Company.query.filter_by(name=name).first():
                return False
            new_company = Company(
                name=name,
                last_updated=datetime.datetime.now(),
                active=active,
                charmtalk=charmtalk,
                description=description,
                trivia=trivia,
                founded = founded if founded != "" else -1,
                contacts = contacts,
                employees_sweden = employees_sweden if employees_sweden != "" else -1,
                employees_world = employees_world if employees_world != "" else -1,
                website = website,
                talk_to_us_about = talk_to_us_about,
                logo = logo,
                tags = tags
            )

            db.session.add(new_company)
            db.session.commit()
        except Exception as e:
            return False
        return True

    def update(self, name, active, charmtalk, description,
            trivia, founded, contacts, employees_sweden,
            employees_world, website,  talk_to_us_about,logo, tags):
        
        print("updating", flush=True)
        self.name = test_and_set(self.name,name)
        self.last_updated = datetime.datetime.now()
        self.active = test_and_set(self.active,active)
        self.charmtalk = test_and_set(self.charmtalk,charmtalk)
        self.description = test_and_set(self.description,description)
        self.trivia = test_and_set(self.trivia,trivia)
        self.founded = test_and_set(self.founded,founded)
        self.founded = self.founded if self.founded != "" else -1,
        self.contacts = test_and_set(self.contacts,contacts)
        self.employees_sweden = test_and_set(self.employees_sweden,employees_sweden)
        self.employees_sweden = self.employees_sweden if self.employees_sweden != "" else -1,
        self.employees_world = test_and_set(self.employees_world, employees_world)
        self.employees_world = self.employees_world if self.employees_world != "" else -1,
        self.website = test_and_set(self.website, website)
        self.logo = test_and_set(self.logo, logo)
        self.talk_to_us_about = test_and_set(self.talk_to_us_about, talk_to_us_about)
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
            'last_updated': (self.last_updated + datetime.timedelta(hours=1, seconds=0)).strftime("%Y-%m-%mT%H:%M:%S"),
            'active': self.active,
            'charmtalk': self.charmtalk,
            'description': self.description,
            'trivia': self.trivia,
            'founded': self.founded,
            'contacts': self.contacts,
            'employees_sweden': self.employees_sweden,
            'employees_world': self.employees_world,
            'website': self.website,
            'logo': self.logo,
            'talk_to_us_about': self.talk_to_us_about,
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
    icon = db.Column(db.String(100))
    division = db.Column(db.Boolean)
    business_area = db.Column(db.Boolean)
    looking_for = db.Column(db.Boolean)
    offering = db.Column(db.Boolean)

    @staticmethod
    def create(name, parent_tag,up_votes, down_votes, crowd_sourced, icon, division, business_area, looking_for, offering):
        try:
            if Tag.query.filter_by(name=name).first():
                return False
            new_tag = Tag(
                name=name,
                parent_tag=parent_tag,
                up_votes = up_votes,
                down_votes = down_votes,
                crowd_sourced=crowd_sourced,
                icon = icon,
                division = division,
                business_area = business_area,
                looking_for = looking_for,
                offering = offering
            )
            db.session.add(new_tag)
            db.session.commit()
        except:
            return False
        return True

    def update(self,name, parent_tag,up_votes, down_votes, crowd_sourced, icon, division, business_area, looking_for, offering):
        try:
            self.name = test_and_set(self.name,name)
            self.parent_tag = test_and_set(self.parent_tag,parent_tag)
            self.up_votes = test_and_set(self.up_votes,up_votes)
            self.down_votes = test_and_set(self.down_votes, down_votes)
            self.crowd_sourced = test_and_set(self.crowd_sourced,crowd_sourced)
            self.icon = test_and_set(self.icon, icon)
            self.division = test_and_set(self.division, division)
            self.business_area = test_and_set(self.business_area, business_area)
            self.looking_for = test_and_set(self.looking_for, looking_for)
            self.offering = test_and_set(self.offering, offering)
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
            'icon': self.icon,
            'division': self.division,
            'business_area': self.business_area,
            'looking_for': self.looking_for,
            'offering': self.offering
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


class Prepage(db.Model):
    __tablename__ = "prepages"
    """
    Reps a prepages
    """
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(100))
    image = db.Column(db.String(100))
    order = db.Column(db.Integer)
    active = db.Column(db.Boolean)

    @staticmethod
    def create( name,active,image, order ):
        try:
            if Prepage.query.filter_by(image=image).first():
                return False
            new_prepage = Prepage(
                name = name,
                image = image,
                order = order,
                active = active
            )

            db.session.add(new_prepage)
            db.session.commit()
        except Exception as e:
            return False
        return True

    def update(self, name, active,image, order,):
        self.name = test_and_set(self.name,name)
        self.active = test_and_set(self.active, active)
        self.image = test_and_set(self.image, image)
        self.order = test_and_set(self.order, order)
        db.session.commit()
        return True

    def delete(self):
        db.session.delete(self)
        db.session.commit()
        return True

    @property
    def serialize(self):
        return {
            'id': self.id,
            'name': self.name,
            'image': self.image,
            'order': self.order,
            'active': self.active
        }
