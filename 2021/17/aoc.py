#!/bin/python

# ======== setup ===========

from time import time
import sys
start = time()
data = open(sys.path[0] + "/data").readline()

# ======== code =======

def run(xs, ys, high=False):
    x = y = 0
    if high: ym = 0
    while True:
        x += xs
        y += ys
        if high and y > ym: ym = y
        xs += -1 if xs > 0 else xs != 0
        ys -= 1
        if x0 <= x <= x1 and y0 <= y <= y1: return ym if high else True
        if y < y0 or x > x1: return False

def find(xm, ym, high=False):
    velocitys = 0
    for ys in range(-ym, ym)[::-1]:
        for xs in range(-xm, xm):
            if run(xs, ys, high): 
                if high: return run(xs, ys, high) 
                else: velocitys += 1
    return velocitys

(x0, x1), (y0, y1) = ((int(x[x.index('=') + 1:x.index("..")]), int(x[x.index("..") + 2:])) for x in data.split(', '))

print(find(18, 150, True))
print(find(200, 200))

print(f"\n===== {time() - start} sec =====")
