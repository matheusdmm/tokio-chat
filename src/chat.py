import telnetlib

HOST = "localhost"
PORT = '8080'

username = input("Enter your username: ")
tn = telnetlib.Telnet(HOST, PORT)

def send(msg):
    temp = username + ': ' + msg  
    tn.write(temp.encode('ascii') + b"\n")

def recv():
    return tn.read_all().decode('ascii')

def disconnect():
    tn.close()

while True:
    command = input("Enter command: ")
    if command == "send":
        send(input("Enter message: "))
    elif command == "recv":
        print(recv())
    elif command == "disconnect":
        disconnect()
        break