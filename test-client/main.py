import socket
import time

if __name__ == '__main__':
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect(("127.0.0.1", 7978))

        while True:
            s.sendall(b'jump')
    #     data = s.recv(1024)
    #
    # print('Received', repr(data))
