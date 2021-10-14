import requests
import json

if __name__ == '__main__':
    host = '192.168.89.91'
    port = '8080'
    URL = f'http://{host}:{port}/msgs'
    data = {
        'msg': 'Hello'
    }
    print(json.dumps(data))
    res = requests.post(URL, json=json.dumps(data))
    print(res.json())