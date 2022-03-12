#!/bin/python

# ======== setup ===========

from time import time

start = time()
data = open("02.data").read().split('\n')

# ======== code =======

def step(instr, button):
	for x in instr:
		if x == 'U' and button[1] > 0: button[1] -= 1
		elif x == 'R' and button[0] < 2: button[0] += 1
		elif x == 'D' and button[1] < 2: button[1] += 1
		elif x == 'L' and button[0] > 0: button[0] -= 1
	return button

def run(data, part2 = False):
	code = ''
	button = [1, 1]
	for instr in data:
		button = step(instr, button)
		code += str(button[0] + 1 + button[1] * 3)
	return code

print(run(data))
# print(run(data, True))
print(f"\n===== {time() - start} sec =====")
