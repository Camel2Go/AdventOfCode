#!/bin/python

# ======== setup ===========

from math import ceil
from time import time

start = time()
data = open("21.data").read()

# ======== code =======
weapons = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)]
armor = [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)]
rings = [(25, 1, 0), (50, 2, 0), (100, 3, 0), (20, 0, 1), (40, 0, 2), (80, 0, 3)]

def game(boss, player):
	boss_damage = boss[1] - player[2]
	player_damage = player[1] - boss[2]
	return ceil(boss[0] / player_damage) <= ceil(player[0] / boss_damage)

def run(boss):
	pass

data = tuple(map(int, [x.split(': ')[-1] for x in data]))

print(run(data))

print(f"\n===== {time() - start} sec =====")
