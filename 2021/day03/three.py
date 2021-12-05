from collections import Counter
l = [Counter(x) for x in zip(*puzzle_input.splitlines())]

gamma = ''.join(["1" if item["1"] > item["0"] else "0" for item in l])
epsilon = ''.join(["1" if item["1"] < item["0"] else "0" for item in l])



print(int(gamma, 2) * int(epsilon, 2))

def search(numbers, cmp):
    population = [(v, k) for k, v in Counter(num[0] for num in numbers).items()]
    if len(population) == 1:
        selected = population[0][1]
    else:
        _, selected = cmp(*population)

    candidates = []
    for number in numbers:
        this, *rest = number
        if this == selected and rest:
            candidates.append(rest)

    if not candidates:
        return selected

    return selected + search(candidates, cmp)

oxygen = search(puzzle_input.splitlines(), max)
co2 = search(puzzle_input.splitlines(), min)

print(int(oxygen, 2) * int(co2, 2))
