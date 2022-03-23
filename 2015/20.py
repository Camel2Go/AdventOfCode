#!/bin/python

# ======== setup ===========

from time import time
from math import sqrt, ceil
from itertools import chain

start = time()
data = open("20.data").read()

# ======== code =======

def part1(data):
	house = score = 0

	while score < data:
		house += 1
		factors = {y for x in range(1, ceil(sqrt(house)) + 1) for y in (x, house // x) if house % x == 0}
		score = sum(factors) * 10
	return house

def part2(data):
	house = score = 0

	while score < data:
		house += 1
		factors = {y for x in range(1, ceil(sqrt(house)) + 1) for y in (x, house // x) if house % x == 0 and house / y <= 50}
		score = sum(factors) * 11
	return house

data = int(data)

print(part1(data))
print(part2(data))
print(f"\n===== {time() - start} sec =====")
