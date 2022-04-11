from . import db,config
from flask_login import UserMixin
import jwt
from werkzeug.security import generate_password_hash,check_password_hash
import datetime
from .helper_functions import test_and_set
import sys,urllib

def serializeGeneric(table,obj):
    result = {}
    for attr in table.__table__.columns.keys():
        if attr == "last_updated":
            tags = []
            for tag in obj.tags:
                tags.append(tag.id)

            result[attr] = (getattr(obj,attr) + datetime.timedelta(hours=1, seconds=0)).strftime("%Y-%m-%mT%H:%M:%S")
            result["tags"] = tags
        else:
            result[attr] = getattr(obj,attr)
    return result


def createGeneric(table,args):
    try:
        if type(args) == list:
            if table.query.filter_by(name=args[0]).first():
                return False
            
            new_obj = table()
            attrs =  table.__table__.columns.keys()

            if table == Company:
                attrs.append("tags")

            index = 0
            for attr in table.__table__.columns.keys()[1:]:
                if attr == "last_updated":
                    continue
                elif attr in ["attr", "map_image","icon"]:
                    setattr(new_obj,attr,urllib.parse.quote(args[index]))
                else:
                    setattr(new_obj,attr,args[index])

                index += 1

        elif type(args) == dict:
            primary_key = table.__table__.columns.keys()[1]
            if table.query.filter_by(name=args[primary_key]).first():
                return False

            new_obj = table()
            for attr in table.__table__.columns.keys()[1:]:
                setattr(new_obj,attr,args[attr])
        
        else:
            raise Exception(f"{type(args)} is not a valid input" )
        
        # Handle company edge case
        if table is Company:
            new_obj.last_updated=datetime.datetime.now(),
            new_obj.tags = args[-1] 

        db.session.add(new_obj)
        db.session.commit()
    except Exception as e:
        return False
    return True

def updateGeneric(table,obj,args):
    try:

        if type(args) == list:
            for index, attr in enumerate(table.__table__.columns.keys()[1:]):
                setattr(obj,attr,test_and_set(getattr(obj,attr),args[index]))

        elif type(args) == dict:
            for attr in table.__table__.columns.keys()[1:]:
                setattr(obj,attr,test_and_set(getattr(obj,attr),args[attr]))
        else:
            raise Exception(f"{type(args)} is not a valid input" )
        
        # Handle company edge case
        if table is Company:
            obj.last_updated=datetime.datetime.now(),
        db.session.commit()
    except:
        return False
    return True

def deleteGeneric(obj):
    try:
        db.session.delete(obj)
        db.session.commit()
    except:
        return False
    return True


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
    name = db.Column(db.String(200))
    active = db.Column(db.Boolean)
    charmtalk = db.Column(db.Boolean)
    description = db.Column(db.String(1000))
    summer_job_description = db.Column(db.String(1000))
    summer_job_link = db.Column(db.String(1000))
    contacts = db.Column(db.String(100))
    contact_email = db.Column(db.String(320))
    employees_world = db.Column(db.Integer)
    website = db.Column(db.String(200))
    talk_to_us_about = db.Column(db.String(1000))
    logo = db.Column(db.String(100))
    map_image = db.Column(db.String(100))
    booth_number = db.Column(db.Integer)
    last_updated = db.Column(db.DateTime)
    tags = db.relationship(
        'Tag',
        secondary=companies_tags,
        lazy='subquery',
        backref=db.backref('tags', lazy=True, cascade='all, delete')
    )


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
    language = db.Column(db.Boolean)
class Map(db.Model):
    __tablename__ = "maps"
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(100))
    image = db.Column(db.String(100))
    ref = db.Column(db.Integer)


class Prepage(db.Model):
    __tablename__ = "prepages"
    """
    Reps a prepages
    """
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(100))
    active = db.Column(db.Boolean)
    image = db.Column(db.String(100))
    order = db.Column(db.Integer)

class Layout(db.Model):
    __tablename__ = "layout"
    id = db.Column(db.Integer, primary_key=True)
    active = db.Column(db.Boolean)
    image = db.Column(db.String(100))
    placement = db.Column(db.Integer)

class Shortcut(db.Model):
    __tablename__ = "shortcuts"
    id = db.Column(db.Integer, primary_key=True)

    name = db.Column(db.String(100))
    desc = db.Column(db.String(100))
    link = db.Column(db.String(100))
    icon = db.Column(db.String(100))

class Company_card(db.Model):
    __tablename__ = "company_cards"
    id = db.Column(db.Integer, primary_key=True)

    text = db.Column(db.String(100))
    name = db.Column(db.String(100))
    active = db.Column(db.Boolean)
