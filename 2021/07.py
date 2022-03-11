#!/bin/python

# ======== setup ===========
data = open("07.data").read().split(',')
data = [int(x) for x in data]

# ======== code =======

def run(data, constant = True):
    return min([sum([abs(x - y) for x in data]) for y in range(min(data), max(data) + 1)]) if constant else min([sum([abs(x - y) * (abs(x - y) + 1) // 2 for x in data]) for y in range(min(data), max(data) + 1)])

print(run(data))
print(run(data, False))