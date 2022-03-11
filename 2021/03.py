#!/bin/python

# ======== setup ===========

data = open("03.data").read().split('\n')

# ======== code =======

for i in range(len(data[0])):
    if len(data) == 1: break
    x = sum([int(data[j][i]) for j in range(len(data))], 0)
    data = [d for d in data if int(d[i]) == (x // (len(data)/2))]

o2 = int(data[0], 2)

data = open("03.data").read().split('\n')
for i in range(len(data[0])):
    if len(data) == 1: break
    x = sum([int(data[j][i]) for j in range(len(data))], 0)
    data = [d for d in data if int(d[i]) == ((x // (len(data)/2)) + 1) % 2]

co2 = int(data[0], 2)

print(o2 * co2)