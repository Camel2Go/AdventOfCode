#!/bin/python

# ======== setup ===========

from time import time

start = time()
data = open("01.data").read().split(', ')

# ======== code =======

def run(data, part2 = False):
	direct = (0, 1)
	(x, y) = (0, 0)
	visited = []
	for instr in data:
		direct = (direct[1], -direct[0]) if instr[0] == 'R' else (-direct[1], direct[0]) 
		if part2:
			if direct[0]: 
				for nx in range(int(instr[1:])):
					nxy = (x + direct[0] * nx, y)
					if nxy in visited: return abs(nxy[0]) + abs(nxy[1])
					visited.append(nxy)
			else: 
				for ny in range(int(instr[1:])):
					nxy = (x, y + direct[1] * ny)
					if nxy in visited: return abs(nxy[0]) + abs(nxy[1])
					visited.append(nxy)
		(x, y) = (x + direct[0] * int(instr[1:]), y + direct[1] * int(instr[1:]))
	return abs(x) + abs(y)

print(run(data))
print(run(data, True))
print(f"\n===== {time() - start} sec =====")
