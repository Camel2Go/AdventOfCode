#!/bin/python

# ======== setup ===========

data = [int(x) for x in open("data").read().split(',')]

# ======== code =======

def read(data):
    state = []
    for i in range(9):
        state.append(data.count(i))
    return state

def simulate(state, days):
    for _ in range(days):
        new = state[0]
        state = state[1:]
        state[6] += new
        state.append(new)
    return state


state = read(data)
print(sum(simulate(state, 80)))
print(sum(simulate(state, 256)))