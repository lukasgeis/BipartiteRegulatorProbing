import os
import pickle
import argparse
import numpy as np
import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

sns.set_theme(style = "darkgrid")
plt.rcParams["text.usetex"] = True

parser = argparse.ArgumentParser(description = "Plot Values using Seaborn")
parser.add_argument("--general", required = True, help = "General Input File to Plot")
parser.add_argument("--poisson", required = True, help = "Poisson Input File to Plot")
args = parser.parse_args()

goal = os.path.basename(args.general).split("_")[1]
with open(args.general, "rb") as datafile:
    general_data = pickle.load(datafile)
with open(args.poisson, "rb") as datafile:
    poisson_data = pickle.load(datafile)

if general_data["x"] != poisson_data["x"]:
    quit()

keys = general_data["x"]
inverted_array = [
    [
        general_data["y"][key][2],
        poisson_data["y"][key][2],
        general_data["y"][key][0],
        poisson_data["y"][key][0],
        general_data["y"][key][1],
        poisson_data["y"][key][1]
    ] for key in range(len(keys))
]


values = np.array(inverted_array)

data = pd.DataFrame(values, keys, columns = ["OPT - General", "OPT - Poisson", "AMP - General", "AMP - Poisson", "NAMP - General", "NAMP - Poisson"])

plot = sns.lineplot(data = data, palette = "Paired", dashes = False, linewidth = 2.5)

def goal_function(fun: str) -> str:
    if "MAX" in fun:
        return r'$f_{max}$'
    elif "SUM" in fun:
        return r'$f_{sum}$'
    else:
        return r'$f_{cov}$'

plt.yscale("log")
plt.title(goal_function(goal) + " : Average Running Time", fontsize = 20)
plt.xlabel(r'$z$', fontsize = 20)
plt.ylabel(r'time in $s$', fontsize = 20)
plt.setp(plot.get_legend().get_texts(), fontsize = 17)

plt.show()