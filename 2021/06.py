#!/bin/python

# ======== setup ===========
data = [int(x) for x in open("06.data").read().split(',')]

# ======== code =======

def run(fish, days):
    for _ in range(days):
        new = fish[0]
        fish = fish[1:]
        fish[6] += new
        fish.append(new)
    return sum(fish)

fish = [data.count(i) for i in range(9)]

print(run(fish, 80))
print(run(fish, 256))