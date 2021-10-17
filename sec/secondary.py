import json
import time
import random
from flask import Flask, request, jsonify
import sys
import argparse

app = Flask('Secondary')

MESSAGES_LIST = []


@app.route("/", methods=['GET'])
def hello():
    return 'Welcome on server!'


@app.route("/msgs", methods=['GET', 'POST'])
def msgs_listener():
    if request.method in ['DELETE', 'PUT']:
        return jsonify(isError=True,
                       message=f'Use GET or POST methods.',
                       statusCode=400), 400
    time.sleep(random.randint(1, 5))
    if request.method == 'POST':
        data = json.loads(request.json)
        if 'msg' in data:
            MESSAGES_LIST.append(data['msg'])
            return jsonify(isError=False,
                           message="Success",
                           statusCode=200,
                           data=data), 200
        else:
            return jsonify(isError=True,
                           message='Use json key "msg" for POST request.')

    if request.method == 'GET':
        return jsonify(isError=False,
                       message=MESSAGES_LIST,
                       statusCode=200), 200


def main(host='0.0.0.0', port=8080, debug=True, threaded=False):
    app.run(host=host, port=port, debug=debug, threaded=threaded)


if __name__ == '__main__':
    # main(debug=False)
    parser = argparse.ArgumentParser(description='Params for server.')
    parser.add_argument('-host', '--host', type=str, default=None)
    parser.add_argument('-p', '--port', type=int, default=None)
    parser.add_argument('-d', '--debug', type=bool, default=None)
    parser.add_argument('-th', '--threaded', type=bool, default=None)
    args = parser.parse_args()
    main(host=args.host, port=args.port, debug=args.debug, threaded=args.threaded)
