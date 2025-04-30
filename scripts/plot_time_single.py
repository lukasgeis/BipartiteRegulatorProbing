import pandas as pd
import argparse
import seaborn as sns
import matplotlib.pyplot as plt

sns.set_theme(style="whitegrid")
plt.rcParams["text.usetex"] = True
plt.rcParams['figure.figsize'] = 15, 8

parser = argparse.ArgumentParser()
parser.add_argument("-d", "--data", type=str, required=True)
parser.add_argument("-o", "--out", type=str, required=True)

args = parser.parse_args()

data = pd.read_json(args.data, lines=True)

data = data[(data.algo != "EXT") & (data.algo != "OPT")]

# Group Algorithms by k and l
data["kk"] = (data["k"] * 4) / data["na"]
data["ll"] = (data["l"] * 4) / data["k"]

data[["kk", "ll"]] = data[["kk", "ll"]].astype(int)

data["time"] = data["time"] * 1000000

algo = {
    "AMP": r"\textsc{Amp}",
    "NAMP": r"\textsc{Namp}",
}

data.replace({"algo": algo}, inplace=True)

print("Now plotting")

# Plot Data
fig, ax = plt.subplots(3, 4, sharex=True, sharey=True)

palette = sns.color_palette("colorblind")[:2]
dashes = [(1, 1), (3, 2)]

for k in range(3):
    for l in range(4):
        plot = sns.lineplot(
            ax=ax[k, l],
            data=data[(data.kk == (k + 1)) & (data.ll == (l + 1))],
            x="na",
            y="time",
            hue="algo",
            style="algo",
            palette=palette,
            dashes=dashes
        )
        if k != 2 or l != 3:
            plot.get_legend().remove()
        else:
            plt.setp(plot.get_legend().get_texts(), fontsize=20)
            plot.get_legend().set_title(r"\textsc{Algorithm}")
            sns.move_legend(plot, "lower right")

ax[0][0].set_title(r'$\ell = \frac{1}{4}k$', fontsize=17)
ax[0][1].set_title(r'$\ell = \frac{2}{4}k$', fontsize=17)
ax[0][2].set_title(r'$\ell = \frac{3}{4}k$', fontsize=17)
ax[0][3].set_title(r'$\ell = \frac{4}{4}k$', fontsize=17)

ax[2][0].set_xlabel("")
ax[2][1].set_xlabel("")
ax[2][2].set_xlabel("")

ax[0][0].set_ylabel(r'$k = \frac{1}{4}n_A$', fontsize=17)
ax[1][0].set_ylabel(r'$k = \frac{2}{4}n_A$', fontsize=17)
ax[2][0].set_ylabel(r'$k = \frac{3}{4}n_A$', fontsize=17)


fig.text(0.5, 0.04, r'$n_{\mathcal{A}}$', ha="center", fontsize=20)
fig.text(0.04, 0.5, r'time in $\mu s$', va="center", rotation="vertical", fontsize=20)

plt.yscale("log")

plt.savefig(f"{args.out}.pdf", format="pdf", bbox_inches="tight")
