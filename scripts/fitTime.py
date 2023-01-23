import pickle
from math import log
import matplotlib.pyplot as plt

PATH = "logs/time/T_COV_G"

def fun(na: int, nb: int) -> int:
    return na * na * na * na * nb

with open(PATH, "rb") as f:
    data = pickle.load(f)

keys = data["x"]

values = [
    data["y"][key][0] for key in range(len(keys))
]

factors = [
    values[key] / fun(keys[key] * 16, keys[key] * 16) for key in range(len(keys))    
]

factor = sum(factors) / len(keys)

fitting = [
    factor * fun(keys[key] * 16, keys[key] * 16) for key in range(len(keys))
]

plt.plot(keys, values, label = "Values")
plt.plot(keys, fitting, label = "Fitting")
plt.yscale("log")
plt.legend()

plt.show()