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
parser.add_argument("input", help = "Input File to Plot")
args = parser.parse_args()

header = os.path.basename(args.input).split("_")
with open(args.input, "rb") as datafile:
    raw_data = pickle.load(datafile)

inverted_ararys = [
    [
        [
            [
                raw_data["y"][k][l][0][x][1],
                raw_data["y"][k][l][0][x][0],
                raw_data["y"][k][l][1][x][1],
                raw_data["y"][k][l][1][x][0],
                1.0
            ] for x in range(len(raw_data["x"]))
        ] for l in range(4)
    ] for k in range(3)
]

values = [
    [
        np.array(inverted_ararys[k][l]) for l in range(4)   
    ] for k in range(3)
]

data = [
    [
        pd.DataFrame(values[k][l], raw_data["x"], columns = ["AMP - Average", "AMP - Worst", "NAMP - Average", "NAMP - Worst", "OPT"]) for l in range(4)
    ] for k in range(3)
]

fig, ax = plt.subplots(3, 4, sharex = True, sharey = True)

for k in range(3):
    for l in range(4):
        plot = sns.lineplot(ax = ax[k, l], data = data[k][l], palette = "tab10", linewidth = 2.5)
        if k != 2 or l != 3:
            plot.get_legend().remove()
        else:
            plt.setp(plot.get_legend().get_texts(), fontsize = 17)

ax[0][0].set_title(r'$\ell = \frac{1}{4}k$', fontsize = 17)
ax[0][1].set_title(r'$\ell = \frac{2}{4}k$', fontsize = 17)
ax[0][2].set_title(r'$\ell = \frac{3}{4}k$', fontsize = 17)
ax[0][3].set_title(r'$\ell = \frac{4}{4}k$', fontsize = 17)

ax[0][0].set_ylabel(r'$k = \frac{1}{4}n_A$', fontsize = 17)
ax[1][0].set_ylabel(r'$k = \frac{2}{4}n_A$', fontsize = 17)
ax[2][0].set_ylabel(r'$k = \frac{3}{4}n_A$', fontsize = 17)

def goal_function(fun: str) -> str:
    if "MAX" in fun:
        return r'$f_{max}$'
    elif "SUM" in fun:
        return r'$f_{sum}$'
    else:
        return r'$f_{cov}$'

if header[2][0] == "P":
    fig.suptitle(goal_function(header[1]) + r' : $n_A = 16 \cdot z,\, n_B = 400 \cdot z,\, |\mathcal{V}| = 10$', fontsize = 23, y = 0.95)
else:
    fig.suptitle(goal_function(header[1]) + r' : $n_A = n_B = |\mathcal{V}| = 16 \cdot z$', fontsize = 23, y = 0.95)

fig.text(0.5, 0.04, r'$z$', ha = "center", fontsize = 20)
fig.text(0.04, 0.5, r'$val_{ALG} / val_{OPT}$', va = "center", rotation = "vertical", fontsize = 20)

plt.show()