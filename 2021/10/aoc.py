#!/bin/python

# ======== setup ===========
import sys
from functools import reduce
data = open(sys.path[0] + "/data").read().split('\n')

# ======== code =======

def repair(line, complete=False):
    check = []
    for x in line:
        if x in "([{<": check.append(oracle[x])
        elif x != check.pop() and not complete: return x
    if complete: return check[::-1]

oracle = {'(': ')', '[': ']', '{': '}', '<': '>'}
score_cor = {None: 0, ')': 3, ']':57 , '}':1197 , '>':25137}
score_rep = {')': 1, ']':2 , '}':3 , '>':4}

print(sum([score_cor[repair(line)] for line in data]))
data = [reduce(lambda s, x: 5 * s + score_rep[x], repair(line, True), 0) for line in [line for line in data if not repair(line)]]
print(sorted(data)[len(data) // 2])