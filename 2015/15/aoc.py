#!/bin/python

# ======== setup ===========

from itertools import product
from math import prod
from time import time
import sys
start = time()
data = open(sys.path[0] + "/data").read().split('\n')
# ======== code =======

data = [tuple(int(x[-2:]) for x in line.split(',')) for line in data]

def score(amount):
	cookie = (sum(amount[ingr] * data[ingr][prop] for ingr in range(len(data))) for prop in range(4))
	return prod(x if x > 0 else 0 for x in cookie)

def run(calories = False):
	max = 0
	for amount in (x for x in product(range(100), repeat=len(data)) if sum(x) == 100):
		if not calories or sum(amount[x] * data[x][4] for x in range(len(data))) == 500:
			x = score(amount)
			if x > max:
				max = x
	return max

print(run())
print(run(True))
print(f"\n===== {time() - start} sec =====")
