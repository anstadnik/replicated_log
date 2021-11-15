import requests
import json
import aiohttp
import asyncio
import time


async def get_pokemon(session, url, data):
    async with session.post(url,json=data) as resp:
        msg = await resp.json()
        print(msg)
        return msg


async def main():

    async with aiohttp.ClientSession() as session:

        tasks = []
        for number in range(1, 11):
            host = 'localhost'
            port = '5000'
            url = f'http://{host}:{port}/'
            message = f'Hello, number {number}'
            data = {
                'msg': message
            }
            tasks.append(asyncio.ensure_future(get_pokemon(session, url,json.dumps(data))))

        msg_list = await asyncio.gather(*tasks)

        for msg in msg_list:
            print(msg)

if __name__ == "__main__":
    start_time = time.time()
    asyncio.run(main())
    print("--- %s seconds ---" % (time.time() - start_time))