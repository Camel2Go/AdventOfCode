#!/bin/python

# ======== setup ===========

data = open("data").read().split('\n')

# ======== code =======

field = {}
for line in data:
    (x0, y0), (x1, y1) = [(int(x.split(',')[0]), int(x.split(',')[1])) for x in line.split(" -> ")]
    diffx, diffy = abs(x1-x0), abs(y1-y0)
    xs = 0 if x0 == x1 else (x1 - x0) // diffx 
    ys = 0 if y0 == y1 else (y1 - y0) // diffy
    (x, y) = (x0, y0)
    for _ in range(max(diffx, diffy) + 1):
        field[(x, y)] = field.get((x, y), 0) + 1
        x += xs
        y += ys

print(len([x for x in field.values() if x > 1]))