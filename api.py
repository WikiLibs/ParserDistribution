import flask
from flask import request

app = flask.Flask(__name__)
app.config["DEBUG"] = True


@app.route('/', methods=['GET'])

def home():
    os = request.args.get("os")
    apiKey = request.args.get("apiKey")
    userID = request.args.get("userId")

    if os and apiKey and userID:
        print("[SUCCESS] OS: " + str(os) + ", apiKey: " + str(apiKey) + ", userID: " + str(userID))
    else:
        print("[WARNING] ARGS MISSING")

    return "<h1>Petite initialisation</h1><p>Voilà Nicolas amuse toi avec ça! Ca m'a effectivement pris 5 min X) </p>"

app.run()
