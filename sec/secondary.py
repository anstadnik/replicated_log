import json
import logging
import os
import random
import time

from flask import Flask, jsonify, request

log = logging.getLogger("werkzeug")
log.setLevel(logging.ERROR)


app = Flask(__name__)

MESSAGES_LIST = []


@app.route("/", methods=["GET", "POST"])
async def msgs_listener():
    if request.method in ["DELETE", "PUT"]:
        return (
            jsonify(isError=True, message=f"Use GET or POST methods.", statusCode=400),
            400,
        )
    # time.sleep(random.randint(1, 10))
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
            MESSAGES_LIST.append(data["msg"])
            return (
                jsonify(isError=False, message="Success", statusCode=200, data=data),
                200,
            )
        else:
            return jsonify(isError=True, message='Use json key "msg" for POST request.')

    if request.method == "GET":
        print(f"Messages: {MESSAGES_LIST}")
        return jsonify(isError=False, message=MESSAGES_LIST, statusCode=200), 200


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=4567, debug=False)
