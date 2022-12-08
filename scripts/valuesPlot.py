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

inverted_array = [
    [
        raw_data["y"][0][0][i],
        raw_data["y"][0][1][i],
        raw_data["y"][1][0][i],
        raw_data["y"][1][1][i],
        1.0
    ] for i in range(len(raw_data["x"]))
]

values = np.array(inverted_array)

data = pd.DataFrame(values, raw_data["x"], columns = ["AMP - Avergae", "AMP - Worst", "NAMP - Average", "NAMP - Worst", "OPT"])

plot = sns.lineplot(data = data, palette = "tab10", linewidth = 2.5)

if "POISSON" in header:
    plt.title(str(header[0]) + r' : $n_A = 16 \cdot z,\, n_B = 400 \cdot z,\, |\mathcal{V}| = 10$', fontsize = 23)
else:
    plt.title(str(header[0]) + r' : $n_A = n_B = |\mathcal{V}| = 16 \cdot z$', fontsize = 23)
plt.xlabel(r'$z$', fontsize = 20)
plt.ylabel(r'$val_{ALG} / val_{OPT}$', fontsize = 20)
plt.setp(plot.get_legend().get_texts(), fontsize = 17)

plt.show()