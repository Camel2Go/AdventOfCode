#!/bin/python

# ======== setup ===========
data = open("14.data").read().split('\n')

# ======== code =======

def run(iterations, polymer, rules, last):
    for _ in range(iterations):
        new = {}
        for key in polymer.keys():
            new[rules[key][0]] = new.get(rules[key][0], 0) + polymer[key]
            new[rules[key][1]] = new.get(rules[key][1], 0) + polymer[key]
        polymer = new
        last = rules[last][1]

    occurs = [0] * 26
    for key in polymer.keys():
        occurs[ord(key[0]) - 65] += polymer[key]
    occurs[ord(last[1]) - 65] += 1

    return max(occurs) - min([x for x in occurs if x != 0])


polymer = {}
for i in range(len(data[0]) - 1):
    polymer[data[0][i] + data[0][i + 1]] = polymer.get(data[0][i] + data[0][i + 1], 0) + 1
last = data[0][-2:len(data[0])]
rules = {line[:2]: (line[0] + line[6], line[6] + line[1]) for line in data[2:]}

print(run(10, polymer, rules, last))
print(run(40, polymer, rules, last))

