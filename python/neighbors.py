import time
from numba import njit

@njit
def bench(n, p0):
    s1 = set()
    s1.add(p0)
    s2 = set()
    s0 = set()

    for i in range(n):
        for p in s1:
            pair = (p[0] - 1, p[1])
            if pair not in s1 and pair not in s2:
                s0.add(pair)
            pair = (p[0] + 1, p[1])
            if pair not in s1 and pair not in s2:
                s0.add(pair)
            pair = (p[0], p[1] - 1)
            if pair not in s1 and pair not in s2:
                s0.add(pair)
            pair = (p[0], p[1] + 1)
            if pair not in s1 and pair not in s2:
                s0.add(pair)
        s2.clear()
        s1, s2, s0 = s0, s1, s2

    return s1

t = time.time(); print("Time including JIT compilation: ", len(bench(2000, (0, 0)))); print(time.time() - t)
t = time.time(); print("Time without JIT compilation: ", len(bench(2000, (0, 0)))); print(time.time() - t)
