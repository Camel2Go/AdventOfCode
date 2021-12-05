#!/bin/python

# ======== setup ===========

data = open("data").read().split('\n')

# ======== code =======

field = {}
for line in data:
    (x0, y0), (x1, y1) = [(int(x.split(',')[0]), int(x.split(',')[1])) for x in line.split(" -> ")]
    if x0 == x1 or y0 == y1:
        rangex = range(x0, x1 + 1) if x1 >= x0 else range(x1, x0 + 1)
        for x in rangex:
            rangey = range(y0, y1 + 1) if y1 >= y0 else range(y1, y0 + 1)
            for y in rangey:
                field[(x, y)] = field.get((x, y), 0) + 1

print(len([x for x in field.values() if x > 1]))