import asyncio

async def tcp_echo_client():
    reader, writer = await asyncio.open_connection('127.0.0.1', 8080)

    hearbeat = bytes([165,1,0,16,71,31,32,79,172,254,103,0,247,21])

    data = bytes([
        165, 151, 0, 16, 66, 44, 45, 79, 172, 254, 103, 1, 1, 39, 173, 118, 1, 0, 27, 12, 0, 0,
        228, 17, 81, 100, 1, 0, 187, 1, 0, 0, 83, 70, 52, 69, 83, 48, 48, 51, 77, 52, 67, 48, 53,
        56, 32, 32, 114, 1, 94, 11, 217, 2, 5, 0, 0, 0, 8, 0, 9, 0, 8, 0, 225, 8, 207, 8, 213, 8,
        134, 19, 110, 0, 0, 0, 229, 1, 0, 0, 176, 119, 0, 0, 246, 23, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 86, 50, 56, 48, 86, 49, 48, 48, 21, 0, 55, 24, 79, 11, 202, 2, 60,
        0, 1, 0, 52, 5, 77, 6, 23, 5, 7, 0, 0, 0, 0, 0, 6, 0, 228, 3, 224, 3, 227, 3, 86, 50, 56,
        48, 86, 50, 56, 48, 23, 5, 3, 18, 14, 19, 15, 0, 0, 0, 239, 21,
    ])



    print(f"sending {list(hearbeat)}")
    writer.write(hearbeat)
    await writer.drain()

    print('Waiting for response')
    res = await reader.read(23)
    print(f"Reading: {list(res)}")



    step = 100
    for i in range(0, len(data), step):
        d = data[i:i+step]
        print(f"sending {list(d)}")
        writer.write(d)
        await writer.drain()
        await asyncio.sleep(0.005)

    print('Waiting for response')
    res = await reader.read(23)
    print(f"Reading: {list(res)}")


    writer.close()
    await writer.wait_closed()
    print('Closed connection')

asyncio.run(tcp_echo_client())