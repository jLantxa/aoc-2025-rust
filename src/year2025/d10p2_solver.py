import sys
import json
from typing import List, Dict, Any
from z3 import Int, Optimize, sat, Sum

MachineData = Dict[str, Any]

def solve_single_machine(data: MachineData) -> int:
    N = data['num_buttons']
    M = data['num_counters']
    masks: List[int] = data['buttons']
    T: List[int] = data['joltages']

    B = []
    for mask in masks:
        button_vector = [1 if (mask >> j) & 1 else 0 for j in range(M)]
        B.append(button_vector)

    opt = Optimize()

    s_vars = [Int(f"s_{i}") for i in range(N)]

    for s_var in s_vars:
        opt.add(s_var >= 0)

    for j in range(M):
        sum_contributions = [B[i][j] * s_vars[i] for i in range(N)]
        opt.add(Sum(sum_contributions) == T[j])

    opt.minimize(Sum(s_vars))

    if opt.check() == sat:
        model = opt.model()

        total_presses = 0
        for s_var in s_vars:
            presses = model.evaluate(s_var).as_long()
            total_presses += presses

        return total_presses
    else:
        raise RuntimeError(f"No optimal solution found. Z3 Status: {opt.check()}")


if __name__ == "__main__":
    try:
        machine_json = sys.stdin.read()
        machine_data = json.loads(machine_json)
        result = solve_single_machine(machine_data)
        print(result)

    except Exception as e:
        sys.stderr.write(f"Python script failed: {e}\n")
        sys.exit(-1)
