from z3 import BitVec, Solver, sat
import os
import re

def lmap(func, *iterables):
    return list(map(func, *iterables))
def ints(s):
    return lmap(int, re.findall(r"-?\d+", s))  # thanks mserrano!

data = open(os.path.join("..", "puzzle", "day24", "input")).readlines()
hailstones = lmap(ints, data)

solver = Solver()
x, y, z, vx, vy, vz = (BitVec(name, 64) for name in ('x', 'y', 'z', 'vx', 'vy', 'vz'))
for i, (a, b, c, va, vb, vc) in enumerate(hailstones[:3]):
    t = BitVec(f"t{i}", 64)
    solver.add(t > 0)
    solver.add(x + vx * t == a + va * t)
    solver.add(y + vy * t == b + vb * t)
    solver.add(z + vz * t == c + vc * t)
if solver.check() == sat:
    m = solver.model()
    print(sum(m.eval(var).as_long() for var in (x, y, z)))