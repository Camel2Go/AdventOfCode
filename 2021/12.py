#!/bin/python

# ======== setup ===========
data = open("12.data").read().split('\n')

# ======== code =======

def run(x: str, traversed: list, twice=False) -> int:
    if x == "end": return 1 
    if x.islower() and x in traversed:
        if not twice or x == "start": return 0
        twice = False
    return sum([run(cave, traversed + [x], twice) for cave in caves[x]])


caves = {}
for line in data:
    x, y = line.split('-')
    caves[x] = caves.get(x, []) + [y]
    caves[y] = caves.get(y, []) + [x]

print(run("start", []))
print(run("start", [], True))