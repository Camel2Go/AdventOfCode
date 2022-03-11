#!/bin/python

# ======== setup ===========

from time import time
import sys
start = time()
data = open(sys.path[0] + "/data").read().split('\n')

# ======== code =======

def step(z, w, a, b, c):
    x = z % 26 + b
    z //= a
    if x != w: 
        z *= 26
        z += w + c
    return z


def run(data):
    zmin = zmax = {0: 0}
    for (a, b, c) in data:
        print(len(zmin))
        nzmin = nzmax = {}
        for z in zmin:
            for w in range(1, 10):
                x = step(z, w, a, b, c)
                nzmax[x] = max(nzmax.get(x, 0), zmax[z] * 10 + w)
                nzmin[x] = min(nzmin.get(x, float('inf')), zmin[z] * 10 + w)
        zmin, zmax = nzmin, nzmax
    return (zmax.get(0, None), zmin.get(0, None))

data = [(int(data[i + 4][6:]), int(data[i + 5][6:]), int(data[i + 15][6:])) for i in range(0, len(data), 18)]
print("{}\n{}".format(*run(data)))

print(f"\n===== {time() - start} sec =====")
