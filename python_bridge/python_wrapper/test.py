# Make sure to:
# - First install maturin in your virtual env
# - Run `maturin develop --release` in the parent folder (`python_bridge/`) with your virtual env
import json
import python_bridge
print("Loaded python bridge")

a = 2.0
b = 4.0
res = python_bridge.add_two_floats(a, b)
print(f"{a} + {b} = {res}")


p = {"name": "Aaron", "age": 23, "children": None}

greeting = python_bridge.greet_person(json.dumps(p))
print(greeting)
