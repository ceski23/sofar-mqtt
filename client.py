import asyncio

async def tcp_echo_client():
    reader, writer = await asyncio.open_connection('127.0.0.1', 8080)

    hearbeat = bytes([165,1,0,16,71,31,32,79,172,254,103,0,247,21])

    data = bytes([165,151,0,16,66,4,5,79,172,254,103,1,1,39,72,125,14,0,128,0,0,0,69,170,88,100,1,0,40,13,0,0,83,70,52,69,83,48,48,51,77,52,67,48,53,56,32,32,104,1,122,11,213,2,12,0,0,0,9,0,10,0,9,0,195,8,216,8,201,8,135,19,54,1,0,0,69,0,0,0,174,126,0,0,220,24,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,86,50,56,48,86,49,48,48,21,0,4,24,100,11,193,2,60,0,1,0,40,5,87,6,33,5,7,0,0,0,0,0,6,0,226,3,227,3,227,3,86,50,56,48,86,50,56,48,23,5,19,9,36,49,37,0,0,0,96,21])

    hello = bytes([165,86,0,16,65,3,4,79,172,254,103,2,71,125,14,0,127,0,0,0,0,0,0,0,5,60,120,2,25,1,76,83,87,51,95,49,52,95,70,70,70,70,95,49,46,48,46,51,52,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,52,234,231,44,60,22,49,48,46,48,46,48,46,54,52,0,0,0,0,0,0,0,1,0,1,1,39,127,21])

    hello_cd = bytes([165, 28, 0, 16, 72, 3, 4, 79, 172, 254, 103, 1, 92, 133, 14, 0, 37, 0, 0, 0, 110, 170, 88, 100, 1, 1, 12, 59, 133, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 21])
    hello_end = bytes([165,60,0,16,72,9,13,79,172,254,103,1,194,133,14,0,139,0,0,0,110,170,88,100,1,5,44,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,197,21])

    unknown_44 = bytes([165, 47, 0, 16, 67, 5, 6, 79, 172, 254, 103, 129, 165, 11, 15, 0, 30, 0, 0, 0, 0, 0, 0, 0, 8, 0, 66, 111, 98, 101, 114, 46, 78, 69, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 1, 106, 21])

    print(f"sending {list(unknown_44)}")
    writer.write(unknown_44)
    await writer.drain()


    # print(f"sending {list(hearbeat)}")
    # writer.write(hearbeat)
    # await writer.drain()

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