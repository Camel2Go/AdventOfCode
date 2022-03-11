#!/bin/python

# ======== setup ===========

from math import prod
from time import time
start = time()
data = open("16.data").read().split('\n')
# ======== code =======

output = """children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1"""

def part1(data):
	for property in output.split('\n'):
		data = [sue for sue in data if not property.split(':')[0] in sue or property in sue]
	return data

def part2(data):
	for property, value in (x.split(':') for x in output.split('\n')):
		if property == "cats" or property == "trees":
			data = [sue for sue in data if not property in sue or int(sue[sue.index(property):].split(',')[0][-2:]) > int(value)]
		elif property == "pomeranians" or property == "goldfish":
			data = [sue for sue in data if not property in sue or int(sue[sue.index(property):].split(',')[0][-2:]) < int(value)]
		else:
			data = [sue for sue in data if not property in sue or property + ":" + value in sue]
	return data

print(part1(data))
print(part2(data))
print(f"\n===== {time() - start} sec =====")
