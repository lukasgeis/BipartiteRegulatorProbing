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


def getGoalAlgo(row):
    if row["algo"] == "AMP":
        algo = r'\textsc{AmpCov}'
    elif row["algo"] == "NAMP":
        algo = r'\textsc{NampCov}'
    else:
        algo = r'None'

    return algo


def loadAndPrepData(path: str) -> pd.DataFrame:
    # Read Data
    data = pd.read_json(path, lines=True)

    print(f"Now loading {path}")

    # Extract OPT-Values for every Line
    val_na = []
    val_ll = []
    val_is = []
    val_it = []
    val_op = []

    for _, row in data.iterrows():
        if row["algo"] == "OPT":
            val_na.append(row["na"])
            val_ll.append(row["l"])
            val_is.append(row["ins_id"])
            val_it.append(row["iter_id"])
            val_op.append(row["val"])

    opt_data = pd.DataFrame.from_dict({
        "na": val_na,
        "l": val_ll,
        "ins_id": val_is,
        "iter_id": val_it,
        "opt": val_op
    })

    data = data.merge(opt_data, on=["na", "l", "ins_id", "iter_id"])

    # Group Algorithms by k and l
    data["kk"] = (data["k"] * 4) / data["na"]
    data["ll"] = (data["l"] * 4) / data["k"]

    data[["kk", "ll"]] = data[["kk", "ll"]].astype(int)

    # Evaluate Algorithms
    data["approx"] = data["val"] / data["opt"]

    data["goalalgo"] = data.apply(getGoalAlgo, axis=1)

    return data


data = loadAndPrepData(args.data) 

print("Now plotting")

# Plot Data
fig, ax = plt.subplots(3, 4, sharex=True, sharey=True)

palette = sns.color_palette("colorblind")[:2]
dashes = [(1, 1), (3, 2)]

for k in range(3):
    for l in range(4):
        plot = sns.lineplot(
            ax=ax[k, l],
            data=data[
                (data.kk == (k + 1)) &
                (data.ll == (l + 1)) &
                (data.algo != "OPT") &
                (data.algo != "EXT")
            ],
            x="na",
            y="approx",
            hue="goalalgo",
            style="goalalgo",
            palette=palette,
            dashes=dashes
        )
        if k != 2 or l != 3:
            plot.get_legend().remove()
        else:
            ax[2][3].legend(
                title=r'\textsc{Algorithm}',
                fontsize="15",
                title_fontsize="17",
                loc="lower right",
                borderaxespad=0
            )

ax[0][0].set_title(r'$\ell = \frac{1}{4}k$', fontsize=22)
ax[0][1].set_title(r'$\ell = \frac{2}{4}k$', fontsize=22)
ax[0][2].set_title(r'$\ell = \frac{3}{4}k$', fontsize=22)
ax[0][3].set_title(r'$\ell = \frac{4}{4}k$', fontsize=22)

ax[2][0].set_xlabel("")
ax[2][1].set_xlabel("")
ax[2][2].set_xlabel("")
ax[2][3].set_xlabel("")

ax[0][0].set_ylabel(r'$k = \frac{1}{4}n_A$', fontsize=22)
ax[1][0].set_ylabel(r'$k = \frac{2}{4}n_A$', fontsize=22)
ax[2][0].set_ylabel(r'$k = \frac{3}{4}n_A$', fontsize=22)

ax[2][0].tick_params(axis="x", labelsize=18)
ax[2][1].tick_params(axis="x", labelsize=18)
ax[2][2].tick_params(axis="x", labelsize=18)
ax[2][3].tick_params(axis="x", labelsize=18)

ax[0][0].tick_params(axis="y", labelsize=18)
ax[1][0].tick_params(axis="y", labelsize=18)
ax[2][0].tick_params(axis="y", labelsize=18)

fig.text(0.52, 0.06, r'$n_{\mathcal{A}}$', ha="center", fontsize=25)
fig.text(
    0.025,
    0.5,
    r'$val_{ALG} / val_{Off}$',
    va="center",
    rotation="vertical",
    fontsize=25
)

plt.savefig(f"{args.out}.pdf", format="pdf", bbox_inches="tight")
