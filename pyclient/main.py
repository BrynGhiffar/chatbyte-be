# pip install websocket-client
from websocket import create_connection
import json

WS_ENDPOINT = "ws://localhost:8080/message/ws?token=eyJhbGciOiJIUzI1NiJ9.eyJ1aWQiOjJ9.Z6PvjgUoR6rx7gBEkddOlg7DpE5e_-B-3jpzLlSWEIA"

def login():
    pass

def message(receiver_uid, content):
    return json.dumps({
        "receiverUid": receiver_uid,
        "content": content
    })

def main():
    ws = create_connection(WS_ENDPOINT)
    for _ in range(1_000_000):
        ws.send(message(3, "Spamming"))
        res = ws.recv()
    ws.close()
    pass


if __name__ == '__main__':
    main()