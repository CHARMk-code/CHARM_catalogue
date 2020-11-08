import csv
from flask import Flask
from flask_sqlalchemy import SQLAlchemy
import toml

db = SQLAlchemy()
config = toml.load("config.toml")

app = Flask(__name__)



app.config['SECRET_KEY'] = config['flask']['secret_key']
app.config['UPLOAD_FOLDER'] = config['flask']['upload_folder']
app.config['SQLALCHEMY_DATABASE_URI'] = 'mysql+pymysql://' \
    + config['database']['user'] + ':' \
    + config['database']['pass'] + '@' \
    + config['database']['server'] + ':' \
    + config['database']['port'] + '/' \
    + config['database']['db']

db.init_app(app)

with open("data.csv","r") as csv_file:
    reader = list(csv.reader(csv_file, delimiter=';', quotechar='|'))
    tags = []
    companies = []
    tag_comapany = []

    # Generats tags
    tag_row = reader[0]
    row_length = len(tag_row)
    for i in range (row_length -1):
        tags.append(tag_row[i+1])
    
    tag_comapany = [ [] for _ in range(len(tags))]
    for i in range(len(reader) -1):
        companies.append(reader[i+1][0])
        for j in range(row_length-1):
            if reader[i+1][j+1] == "TRUE":
                tag_comapany[j].append(i)


    toml_dict={}
    toml_dict["tags"] = tags
    toml_dict["companies"] = companies
    toml_dict["tag_comapany"] = tag_comapany
    with open("config.toml","w")  as cfg:
        cfg.write(toml.dumps(toml_dict))