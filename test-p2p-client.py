import asyncio
import websockets
import json

async def client():
    uri = "ws://127.0.0.1:15446"

    async with websockets.connect(uri, ping_interval=5) as websocket:
        print(f"Connected to {uri}")

        while True:
            command = input("Enter command ('text', 'file', or 'quit'): ").lower()

            if command == "text":
                message = input("Enter text to send: ")
                await websocket.send("text")
                await websocket.send(message)
                print(f"Sent: {message}")
                try:
                    response = await asyncio.wait_for(websocket.recv(), timeout=5)
                    if response != "tick":
                        print(f"Received: {response}")
                except asyncio.TimeoutError:
                    print("No response received within timeout.")
                except websockets.exceptions.ConnectionClosedOK:
                    print("Connection closed by server.")
                    break
                except websockets.exceptions.ConnectionClosedError as e:
                    print(f"Connection closed unexpectedly: {e}")
                    break

            elif command == "file":
                file_path = input("Enter the path to the file you want to send: ")
                try:
                    with open(file_path, "rb") as file:
                        await websocket.send("file")
                        file_name = file_path.split("\\")[-1]
                        file_size = len(file.read())
                        await websocket.send(f"{file_name}<|>{file_size}")
                        print(f"Sent file info: Name='{file_name}', Size='{file_size}'")
                        file.seek(0)

                        ready = False
                        ready_response = None
                        try:
                            while not ready:
                                ready_response = await asyncio.wait_for(websocket.recv(), timeout=10)
                                if ready_response == "1":
                                    print("Server is ready for file data.")
                                    ready = True
                                elif ready_response == "tick":
                                    print("Received keepalive tick.")
                                else:
                                    print(f"Unexpected response while waiting for readiness: {ready_response}")
                                    break  # Exit the waiting loop if it's not '1' or 'tick'
                        except asyncio.TimeoutError:
                            print("Timeout waiting for server to be ready for file.")
                        except websockets.exceptions.ConnectionClosedOK:
                            print("Connection closed by server while waiting for file readiness.")
                            break
                        except websockets.exceptions.ConnectionClosedError as e:
                            print(f"Connection closed unexpectedly while waiting for file readiness: {e}")
                            break

                        if ready:
                            chunk_size = 1024
                            while True:
                                chunk = file.read(chunk_size)
                                if not chunk:
                                    break
                                await websocket.send(chunk)
                                print(f"Sent {len(chunk)} bytes")
                            await websocket.send("done")
                            print("File sent successfully.")

                            try:
                                response_after_file = await asyncio.wait_for(websocket.recv(), timeout=10)
                                if response_after_file != "tick":
                                    print(f"Received response after file: {response_after_file}")
                                else:
                                    print("Received keepalive tick after sending file.")
                            except asyncio.TimeoutError:
                                print("No response received within timeout after sending file.")
                            except websockets.exceptions.ConnectionClosedOK:
                                print("Connection closed by server.")
                                break
                            except websockets.exceptions.ConnectionClosedError as e:
                                print(f"Connection closed unexpectedly: {e}")
                                break

                except FileNotFoundError:
                    print(f"Error: File not found at '{file_path}'")
                except Exception as e:
                    print(f"An error occurred while preparing to send the file: {e}")

            elif command == "quit":
                break

            else:
                print("Invalid command. Please enter 'text', 'file', or 'quit'.")

if __name__ == "__main__":
    asyncio.run(client())