#!/bin/python

# ======== setup ===========

from itertools import combinations
from time import time

start = time()
data = open("17.data").read().split('\n')

# ======== code =======

data = list(map(int, data))

def run(data, part2=False):
	ret = 0
	min_containers = float('inf')
	for amount in range(len(data)):
		for containers in combinations(data, r=amount):
			if sum(containers) == 150:
				if part2 and len(containers) < min_containers: 
					min_containers = len(containers)
					ret = 0
				if not part2 or len(containers) == min_containers:
					ret += 1
	return ret

print(run(data))
print(run(data, True))
print(f"\n===== {time() - start} sec =====")
