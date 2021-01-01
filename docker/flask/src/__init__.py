from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_login import LoginManager
from flask_mobility import Mobility
import toml
import os
from collections import Mapping
from flask_cors import CORS

# init SQLAlchemy so we can use it later in our models
db = SQLAlchemy()
config = toml.load("config.toml")

def create_app():
    """
    Entry point for api server 
    """
    app = Flask(__name__)
    # # enable CORS
    CORS(app, origins="*")
    

    Mobility(app)

    app.config['SECRET_KEY'] = config['flask']['secret_key']
    app.config['CORS_HEADERS'] = 'Content-Type'
    app.config['SQLALCHEMY_DATABASE_URI'] = 'mysql+pymysql://' \
        + config['database']['user'] + ':' \
        + config['database']['pass'] + '@' \
        + config['database']['server'] + ':' \
        + config['database']['port'] + '/' \
        + config['database']['db']

    db.init_app(app)
    from .api import api as api_blueprint
    app.register_blueprint(api_blueprint)

   
    # blueprint for non-auth parts of app
    # from .main import main as main_blueprint
    # app.register_blueprint(main_blueprint)

    # adds all routes from the routes folder to the app
    # from .routes import register_all_blueprints
    # register_all_blueprints(app)
    # CORS(app, origins="*", allow_headers=[
    # "Content-Type", "Authorization", "Access-Control-Allow-Credentials"])

    return app

with create_app().app_context():
    db.create_all()
    db.session.commit()

