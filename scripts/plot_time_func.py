import pandas as pd
import argparse
import seaborn as sns
import matplotlib.pyplot as plt

sns.set_theme(style="whitegrid")
plt.rcParams["text.usetex"] = True
plt.rcParams['figure.figsize'] = 15, 8

parser = argparse.ArgumentParser()
parser.add_argument("-n", "--network", type=str, required=True)
parser.add_argument("-u", "--uniform", type=str, required=True)
parser.add_argument("-o", "--out", type=str, required=True)

args = parser.parse_args()

uniform_data = pd.read_json(args.uniform, lines=True)
network_data = pd.read_json(args.network, lines=True)

uniform_data["setting"] = uniform_data["algo"] + " - Uniform"
network_data["setting"] = network_data["algo"] + " - Network"

data = pd.concat([uniform_data, network_data], ignore_index=True, sort=False)
data = data[(data.algo != "EXT") & (data.algo != "OPT")]

# Group Algorithms by k and l
data["kk"] = (data["k"] * 4) / data["na"]
data["ll"] = (data["l"] * 4) / data["k"]

data[["kk", "ll"]] = data[["kk", "ll"]].astype(int)

data["time"] = data["time"] * 1000000

settings = {
    "AMP - Uniform": r"\textsc{Amp - Uniform}",
    "NAMP - Uniform": r"\textsc{Namp - Uniform}",
    "AMP - Network": r"\textsc{Amp - Network}",
    "NAMP - Network": r"\textsc{Namp - Network}",

}

data.replace({"setting": settings}, inplace=True)

print("Now plotting")

# Plot Data
fig, ax = plt.subplots(3, 4, sharex=True, sharey=True)

palette = sns.color_palette("colorblind")
palette = [palette[0], palette[0], palette[1], palette[1]]
dashes = [(1, 1), (3, 2), (1, 1), (3, 2)]

for k in range(3):
    for l in range(4):
        plot = sns.lineplot(
            ax=ax[k, l],
            data=data[(data.kk == (k + 1)) & (data.ll == (l + 1))],
            x="na",
            y="time",
            hue="setting",
            style="setting",
            palette=palette,
            dashes=dashes
        )
        if k != 1 or l != 3:
            plot.get_legend().remove()
        else:
            plt.setp(plot.get_legend().get_texts(), fontsize=20)
            plot.get_legend().set_title(r"\textsc{Algorithm \& Setting}")
            sns.move_legend(plot, "center left", bbox_to_anchor=(1.0, 0.5))

ax[0][0].set_title(r'$\ell = \frac{1}{4}k$', fontsize=17)
ax[0][1].set_title(r'$\ell = \frac{2}{4}k$', fontsize=17)
ax[0][2].set_title(r'$\ell = \frac{3}{4}k$', fontsize=17)
ax[0][3].set_title(r'$\ell = \frac{4}{4}k$', fontsize=17)

ax[2][0].set_xlabel("")
ax[2][1].set_xlabel("")
ax[2][2].set_xlabel("")
ax[2][3].set_xlabel("")

ax[0][0].set_ylabel(r'$k = \frac{1}{4}n_A$', fontsize=17)
ax[1][0].set_ylabel(r'$k = \frac{2}{4}n_A$', fontsize=17)
ax[2][0].set_ylabel(r'$k = \frac{3}{4}n_A$', fontsize=17)


fig.text(0.5, 0.04, r'$n_{\mathcal{A}}$', ha="center", fontsize=20)
fig.text(0.04, 0.5, r'time in $\mu s$', va="center", rotation="vertical", fontsize=20)

plt.yscale("log")

plt.savefig(f"{args.out}.pdf", format="pdf", bbox_inches="tight")
