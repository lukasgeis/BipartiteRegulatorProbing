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

keys = sorted(raw_data.keys())
inverted_array = [
    [
        raw_data[key][i][0] / raw_data[key][i][1] for i in range(3)
    ] for key in keys
]

values = np.array(inverted_array)

data = pd.DataFrame(values, keys, columns = ["OPT", "AMP", "NAMP"])

plot = sns.lineplot(data = data, palette = "tab10", linewidth = 2.5)

plt.xscale("log")
plt.yscale("log")
plt.title(str(header[1]) + " : Average Running Time", fontsize = 20)
plt.xlabel(r'$|A \times B| = n_A \cdot n_B$', fontsize = 20)
plt.ylabel(r'time in $s$', fontsize = 20)
plt.setp(plot.get_legend().get_texts(), fontsize = 17)

plt.show()