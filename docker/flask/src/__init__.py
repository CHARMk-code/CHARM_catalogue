from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_mobility import Mobility
import toml, sys
from flask_cors import CORS
from time import sleep
import random


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
    if config['flask']['secret_key'] == "":
        def randomString(): 
            valid_chars = [chr(i+48) for i in range(10)] + [chr(i+65) for i in range(25)] + [chr(i+97) for i in range(25)]
            random_string = ''
            for _ in range(255):
                # Do while loop to avoid reserved char '
                random_string += valid_chars[random.randrange(0,len(valid_chars))]
            return random_string

        # Randomize secrets
        config['flask']['secret_key'] = randomString()
        config['creds']['secret'] = randomString()

        with open("config.toml", "w") as toml_file:
            toml.dump(config,toml_file)


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
