#!/bin/python

# ======== setup ===========

from time import time
from itertools import product
from functools import reduce
import sys
start = time()
data = open(sys.path[0] + "/data").read().replace('#', '1').replace('.', '0').split('\n\n')

# ======== code =======

def run(input, outside):
    out = []
    tmp = ""
    (x, y) = (-1, -1)
    while y != len(input) + 1:
        n = ""
        for (j, i) in product([-1, 0, 1], [-1, 0, 1]):
            if 0 <= x + i < len(input[0]) and 0 <= y + j < len(input): n += input[y + j][x + i]
            else: n += outside
        tmp += algorithm[int(n, 2)]
        x += 1
        if x == len(input[0]) + 1: 
            out.append(tmp)
            tmp = ""
            x = -1
            y += 1
    return out, algorithm[int(9 * outside, 2)]

algorithm = data[0]
input = data[1].split('\n')
outside = '0'
print(''.join(run(*run(input, outside))[0]).count('1'))
for i in range(50):
    input, outside = run(input, outside)
print(''.join(input).count('1'))

print(f"\n===== {time() - start} sec =====")
