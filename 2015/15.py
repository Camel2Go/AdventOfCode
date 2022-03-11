#!/bin/python

# ======== setup ===========

from itertools import product
from math import prod
from time import time
start = time()
data = open("15.data").read().split('\n')
# ======== code =======

def score(amount):
	cookie = (sum(amount[ingr] * data[ingr][prop] for ingr in range(len(data))) for prop in range(4))
	return prod(x if x > 0 else 0 for x in cookie)

def run(calories = False):
	max = 0
	for amount in amounts:
		if not calories or sum(amount[x] * data[x][4] for x in range(len(data))) == 500:
			x = score(amount)
			if x > max:
				max = x
	return max

data = [tuple(int(x[-2:]) for x in line.split(',')) for line in data]
amounts = [x for x in product(range(100), repeat=len(data)) if sum(x) == 100]

print(run())
print(run(True))
print(f"\n===== {time() - start} sec =====")
