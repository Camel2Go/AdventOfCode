#!/bin/python

# ======== setup ===========

from time import time
from math import sqrt
from itertools import combinations, product, permutations
start = time()
data = open("19.data").read().split('\n\n')

# ======== code =======

# {dist between vec1 and vec2: (vec1, vec2)}
def distances(vectors: set) -> dict:
    return {sqrt(sum(pow(y[i] - x[i], 2) for i in range(3))): (x, y) for (x, y) in combinations(vectors, 2)}

# direction-vector from x to y
def vector(x, y):
    return tuple(y[i] - x[i] for i in range(3))

# vec x + vec y
def add(x, y):
    return tuple(x[i] + y[i] for i in range(3))

# direction = e.g. ((0, -1), (1, -1), (2, 1)) = (index of axis, invert axis) * 3
def transform(vector: tuple, direction: tuple) -> tuple:
    return tuple(vector[direction[i][0]] * direction[i][1] for i in range(3))

def check(data, dist_beacons, dist_data):
    dist_same = list(set(dist_beacons) & set(dist_data))        # intersection of distances
    if len(dist_same) >= 66:                                    # if 12 beacons are intersecting (66 = 12*11/2)
        vec_beacons = vector(*dist_beacons[dist_same[0]])       # vector between the two known beacons of the first intersecting distance
        
        for direction in directions:
            
            # check if vector between known beacons is same as transformed vector between unknown beacons 
            if vec_beacons == transform((vector(*dist_data[dist_same[0]])), direction) or vec_beacons == transform((vector(*dist_data[dist_same[0]])), tuple((x, -y) for (x, y) in direction)):
                
                # calc the four possible vectors between one known + one unknown beacon
                candidates = [vector(transform(dist_data[dist_same[0]][x], direction), dist_beacons[dist_same[0]][y]) for (x, y) in product([0, 1], [0, 1])]
                
                scanner = max(set(candidates), key=candidates.count)                            # the vector that occured twice in candiates
                new_beacons = {add(scanner, transform(vec, direction)) for vec in data}         # calculate new beacons
                return new_beacons, scanner

def run(data: dict):
    # all relative distances the beacons have to each other
    dist_data = [distances(x) for x in data]
    beacons, dist_beacons= data[0], dist_data[0]
    scanners = {i: None for i in range(len(data))}
    scanners[0] = (0, 0, 0)
    i = 1
    while len([x for x in scanners.values() if not x]):
        if not scanners[i]:
            result = check(data[i], dist_beacons, dist_data[i])
            if result:
                beacons |= result[0]
                dist_beacons = distances(beacons)
                scanners[i] = result[1]
                i = 0
        i += 1
    return beacons, scanners

directions = {((0, 1), (1, 1), (2, 1)), ((0, 1), (2,-1), (1, 1)), ((0, 1), (1,-1), (2,-1)), ((0, 1), (2, 1), (1,-1)), ((1, 1), (0,-1), (2, 1)), ((1, 1), (2,-1), (0,-1)), ((1, 1), (0, 1), (2,-1)), ((1, 1), (2, 1), (0, 1)), ((2, 1), (1, 1), (0,-1)), ((2, 1), (0, 1), (1, 1)), ((2, 1), (0,-1), (1,-1)), ((2, 1), (1,-1), (0, 1)), ((0,-1), (1, 1), (2,-1)), ((0,-1), (2, 1), (1, 1)), ((0,-1), (1,-1), (2, 1)), ((0,-1), (2,-1), (1,-1)), ((1,-1), (2,-1), (0, 1)), ((1,-1), (0,-1), (2,-1)), ((1,-1), (0, 1), (2, 1)), ((1,-1), (2, 1), (0,-1)), ((2,-1), (1, 1), (0, 1)), ((2,-1), (0,-1), (1, 1)), ((2,-1), (1,-1), (0,-1)), ((2,-1), (0, 1), (1,-1))}
data = [{tuple(map(int, line.split(','))) for line in block.split('\n')[1:]} for block in data]
beacons, scanners = run(data)
print(len(beacons))
print(max([sum(y[i] - x[i] for i in range(3)) for (x, y) in permutations(scanners.values(), 2)]))

print(f"\n===== {time() - start} sec =====")