import flask

app = flask.Flask(__name__)
app.config["DEBUG"] = True


@app.route('/', methods=['GET'])
def home():
    return "<h1>Petite initialisation</h1><p>Voilà Nicolas amuse toi avec ça! </p>"

app.run()
