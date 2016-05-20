#!/usr/bin/python

import socket

print "binding to 1234"
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind(("127.0.0.1", 1234))
s.listen(128)

while 1:
    conn, addr = s.accept()
    conn.send("hello")
    conn.close()
