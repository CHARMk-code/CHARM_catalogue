from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_mobility import Mobility
import toml
from flask_cors import CORS
from time import sleep

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

    # adds all routes from the routes folder to the app
    from .routes import register_all_blueprints
    register_all_blueprints(app)



    return app

with create_app().app_context():
    while True:
        try:
            db.create_all()
            break
        except:
            sleep(1)
            pass
    db.session.commit()
