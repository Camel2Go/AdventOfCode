#!/bin/python

# ======== setup ===========

from time import time

start = time()
data = open("18.data").read().split('\n')

# ======== code =======

def step(state, part2):
	new_state = []
	for i in range(len(state)):
		new_state.append([])
		if part2 and i in [0, len(state) - 1]: 
			new_state[i].append(True)
			jrange = range(1, len(state[0]) - 1)
		else: jrange = range(len(state[0]))
		for j in jrange:
			neighbors = 0
			for ni, nj in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]:
				neighbors += 0 <= i + ni < len(state) and 0 <= j + nj < len(state[0]) and state[i + ni][j + nj]
			new_state[i].append(state[i][j] and neighbors in [2, 3] or not state[i][j] and neighbors == 3)
		if part2 and i in [0, len(state) - 1]: new_state[i].append(True)
	return new_state


def run(data, part2 = False):
	for _ in range(100):
		data = step(data, part2)
	return sum(sum(line) for line in data)	


data = [[x == '#' for x in line] for line in data]

print(run(data))
data[0][0] = data[0][-1] = data[-1][0] = data[-1][-1] = True
print(run(data, True))
print(f"\n===== {time() - start} sec =====")
