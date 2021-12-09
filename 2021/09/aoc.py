#!/bin/python

# ======== setup ===========

import sys
import math
data = open(sys.path[0] + "/data").read()

# ======== code =======

def adjacent(i, n) -> set:
    x = set()
    if i % n: x.add(i - 1)
    if (i + 1) % n: x.add(i + 1)
    if i >= n: x.add(i - n)
    if i < len(data) - n: x.add(i + n)
    return x

def run(i, low=False):
    if low: return i if data[i] < min([data[x] for x in adjacents[i]]) else None
    visited.add(i)
    return 0 if data[i] == 9 else sum([run(i) for i in adjacents[i] if i not in visited]) + 1


n = data.index('\n')
data = [int(x) for x in data if x != '\n']
adjacents = {i:adjacent(i, n) for i in range(len(data))}
low = [x for x in [run(i, True) for i in range(len(data))] if x]
print(sum([data[x] + 1 for x in low]))
visited = set()
print(math.prod(sorted([run(x) for x in low], reverse=True)[:3]))