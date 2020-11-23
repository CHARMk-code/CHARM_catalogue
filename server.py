from flask import Flask, escape, request
import toml
import sys
config = toml.load("config.toml")

tags = config["tags"]
companies = config["companies"]
tag_comapany = config["tag_comapany"]
app = Flask(__name__)


# input:
# user_tags int[]

def match(user_tags):
    temp_companies = {}
    for tag in user_tags:
        for company in tag_comapany[tag]:
            if company in temp_companies:
                temp_companies[company] += 1
            else:
                temp_companies[company] = 1

    temp_companies = list(temp_companies.items())
    temp_companies.sort(key = lambda x: x[1])
    temp_companies = list (map((lambda x: x[0]), temp_companies))
    return temp_companies


@app.route('/tags')
def get_tags():
    return {'tags':tags}

@app.route('/get',methods = ['GET'])
def get():
    user_tags = request.args.get("user_tags")
    user_tags =user_tags.translate({ord('['): None})
    user_tags= user_tags.translate({ord(']'): None})
    user_tags =user_tags.split(',')
    user_tags = list(map(int,user_tags))
    print(user_tags,file=sys.stderr)
    return {'result':list(map((lambda x: companies[x]),match(user_tags)))}

if __name__ == '__main__':
    app.run(debug=True, port=5001)