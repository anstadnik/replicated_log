import json
import logging
import os
import random
import time
import collections
import json

from flask import Flask, jsonify, request

log = logging.getLogger("werkzeug")
log.setLevel(logging.ERROR)


app = Flask(__name__)

MESSAGES_DICT = {}

@app.route("/", methods=["GET", "POST"])
async def msgs_listener():
    if request.method in ["DELETE", "PUT"]:
        return (
            jsonify(isError=True, message=f"Use GET or POST methods.", statusCode=400),
            400,
        )
    try:
        time.sleep(int(os.environ["SLEEP_TIMEOUT"]))
    except (KeyError, ValueError):
        time.sleep(random.randint(1, 10))

    if request.method == "POST":
        if request.json is None:
            print("Wrong request")
            return jsonify(isError=True, message="Use JSON please")
        data = request.json
        if "msg" in data:
            print(f'added message {data["msg"]} with id {data["id"]}')
            # here can be deduplication, check it
            MESSAGES_DICT[int(data['id'])] = data['msg']
            return (
                jsonify(isError=False, message="Success", statusCode=200, data=data),
                200,
            )
        else:
            return jsonify(isError=True, message='Use json key "msg" for POST request.')

    if request.method == "GET":
        if MESSAGES_DICT == {}:
            return jsonify(isError=False, message=MESSAGES_DICT, statusCode=200), 200
        res = {}
        keys = sorted(MESSAGES_DICT.keys())
        shift = keys[0]
        for i, k in enumerate(keys):
            if k - i != shift:
                break
            res[k] = MESSAGES_DICT[k]
        print(res)
        return jsonify(isError=False, message=res, statusCode=200), 200

@app.route("/health", methods=["GET"])
async def heath():
    if request.method == 'GET':
        print(f"Health checker.")
        return jsonify(isError=False, message='Healthy', statusCode=200), 200

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=4567, debug=False)
