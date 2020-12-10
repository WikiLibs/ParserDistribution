import flask
import os
from waitress import serve
import app1
from flask import request, send_file

app = flask.Flask(__name__)
app.config["DEBUG"] = False


@app.route('/', methods=['GET'])

def home():
    pathToBin = "./patch-tool"
    osName = request.args.get("os")
    apiKey = request.args.get("apiKey")
    userID = request.args.get("userId")
    contentType = {}

    if osName and apiKey and userID:
        os.system(pathToBin + ' ' + str(osName) + ' ' + str(apiKey) + ' ' + str(userID) + ' > tmp')

        f = open("tmp", "r")
        content = f.read()
        f.close()
        os.system('rm tmp')

        if len(content) == 0:
            return {'error': "invalid args"}
        
        if osName == 'Linux':
            contentType = 'application/x-executable'
        elif osName == 'Win64':
            contentType = 'application/vnd.microsoft.portable-executable'
        elif osName == 'OSX':
            contentType = 'application/x-mach-binary'

        try:
            return send_file(content.replace('\n', ''), attachment_filename='wikilibs_parser', as_attachment=True, mimetype=contentType), 200
        except Exception as e:
            return {'error': str(e)}
        
    else:
        return {'error': "missing args"}

if __name__ == "__main__":
    serve(app1.app, host='127.0.0.1', port=4242)
