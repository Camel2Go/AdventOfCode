#!/bin/python

# ======== setup ===========

from time import time

start = time()
data = open("19.data").read().split('\n')

# ======== code =======

def replace(molecule, replacement, reverse = False):
	molecules = set()
	x, y = replacement.split(" => ")[::-1] if reverse else replacement.split(" => ")
	for i in range(len(molecule) - len(x) + 1):
		if molecule[i:i + len(x)] == x:
			molecules.add(molecule[:i] + y + molecule[i + len(x):])
	return molecules


def calibrate(molecule, replacements):
	molecules = set()
	for replacement in replacements:
		molecules |= replace(molecule, replacement)
	return len(molecules)

# def generate(target, replacements):
# 	molecules = {'e' : 0}
# 	while not target in molecules:
# 		new_molecules = {}
# 		for molecule, steps in molecules.items():
# 			for replacement in replacements:
# 				for result in replace(molecule, replacement):
# 					if len(result) <= len(target): new_molecules[result] = molecules.get(result, steps + 1)
# 					# if len(result) <= len(target) and result not in molecules: molecules[result] = steps + 1
# 		molecules = new_molecules
# 		print(len(molecules))
# 	return molecules[target]

def generate(target, replacements):
	molecules = {target}
	steps = 0
	while not 'e' in molecules:
		steps += 1
		new_molecules = set()
		for molecule in molecules:
			for replacement in replacements:
				new_molecules |= replace(molecule, replacement, True)
		molecules = new_molecules
		print(len(molecules))
	return steps

molecule = data[-1]
replacements = data[:-2]

print(calibrate(molecule, replacements))
print(generate(molecule, replacements))

print(f"\n===== {time() - start} sec =====")
