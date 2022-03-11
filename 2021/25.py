#!/bin/python

# ======== setup ===========

from time import time
start = time()
data = open("25.data").read().split('\n')

# ======== code =======

def step():
    global data
    count = 0
    for c in ['>', 'v']:
        move = []
        for y in range(len(data)):
            for x in range(len(data[0])):
                if data[y][x] == c and data[(y + (c == 'v')) % len(data)][(x + (c == '>')) % len(data[0])] == '.':
                    move.append((x, y))
        count += len(move)
        for (x, y) in move:
            data[y][x] = '.'
            data[(y + (c == 'v')) % len(data)][(x + (c == '>')) % len(data[0])] = c
    return count

def run():
    count = 1
    while step():
        count += 1
    return count

data = [list(line) for line in data]
print(run())

print(f"\n===== {time() - start} sec =====")
