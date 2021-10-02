from sqlalchemy.sql.schema import Column
from toml.decoder import TIME_RE
from . import db
from flask_login import UserMixin
import sys

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
    __tablename__ = "companies"
    """
    Reps a company
    """
    id = db.Column(db.Integer, primary_key=True)
    active = db.Column(db.Boolean)
    name = db.Column(db.String(200))
    description = db.Column(db.String(1000))
    founded = db.Column(db.Integer)
    contacts = db.Column(db.String(300))
    employs_sweden = db.Column(db.Integer)
    employs_world = db.Column(db.Integer)
    trivia = db.Column(db.String(500))
    website = db.Column(db.String(200))
    tags = db.relationship(
        'Tag',
        secondary=companies_tags,
        lazy='subquery',
        backref=db.backref('tags', lazy=True, cascade='all, delete')
    )

    @staticmethod
    def create( name, active, description,
        trivia, founded, contacts, employs_sweden,
        employs_world, website, tags):
        try:
            if Company.query.filter_by(name=name).first():
                return False
            new_company = Company(
                name=name,
                active=active,
                description=description,
                trivia=trivia,
                founded = founded,
                contacts = contacts,
                employs_sweden = employs_sweden,
                employs_world = employs_world,
                website = website,
                tags = tags
            )

            db.session.add(new_company)
            db.session.commit()
        except Exception as e:
            return False
        return True

    def update(self, name, active, description,
            trivia, founded, contacts, employs_sweden,
            employs_world, website, tags):
        self.name = name
        self.active = active
        self.description = description
        self.trivia = trivia
        self.founded = founded
        self.contacts = contacts
        self.employs_sweden = employs_sweden
        self.employs_world = employs_world
        self.website = website
        self.tags = tags

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
            'trivia': self.trivia,
            'founded': self.founded,
            'contacts': self.contacts,
            'employs_sweden': self.employs_sweden,
            'employs_world': self.employs_world,
            'website': self.website,
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
            self.name = name
            self.parent_tag = parent_tag
            self.up_votes = up_votes
            self.down_votes = down_votes
            self.crowd_sourced = crowd_sourced
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
            'up_votes': self.up_votes,
            'down_votes': self.down_votes,
            'crowd_soured': self.crowd_soured,
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

    def update_helper(self, tag, company, up_votes, down_votes, crowd_sourced):
        try:
            self.tag = tag
            self.company = company
            self.up_votes=up_votes
            self.down_votes=down_votes
            self.crowd_sourced = crowd_sourced
            db.session.commit()
            return True
        except:
            return False

