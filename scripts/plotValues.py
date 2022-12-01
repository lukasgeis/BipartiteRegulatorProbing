import os
import pickle
import argparse
import matplotlib.pyplot as plt
plt.rcParams['text.usetex'] = True
plt.rcParams['axes.titlepad'] = 10

def main(inputFile: str):
    header = os.path.basename(inputFile).split("_")
    with open(inputFile, "rb") as infile:
        data = pickle.load(infile)

    plt.plot([-1] + data["x"] + [data["x"][len(data["x"]) - 1] + 10], [1.0, 1.0] + [1.0 for x in data["x"]], "-", color = "black", label = "OPT")
    plt.plot(data["x"], data["y"][0][0], "-", color = "blue", label = "AMP - Average")
    plt.plot(data["x"], data["y"][0][1], "-", color = "green", label = "AMP - Worst")
    plt.plot(data["x"], data["y"][1][0], "--",color = "darkorange", label = "NAMP - Average")
    plt.plot(data["x"], data["y"][1][1], "--", color = "red", label = "NAMP - Worst")

    plt.xlim(0.5, data["x"][len(data["x"]) - 1] + 0.5)
    emax = max(data["x"])
    plt.xticks([1] + [i for i in range(5, emax, 5)] + [emax])

    if "POISSON" in header:
        plt.title(str(header[0]) + r' : $n_A = 16 \cdot z,\, n_B = 400 \cdot z,\, |\mathcal{V}| = 10$', fontsize = 23)
    else:
        plt.title(str(header[0]) + r' : $n_A = n_B = |\mathcal{V}| = 16 \cdot z$', fontsize = 23)
    plt.xlabel(r'$z$', fontsize = 20)
    plt.ylabel(r'$val_{ALG} / val_{OPT}$', fontsize = 20)
    plt.legend(prop = {"size": 17})

    plt.show()



if __name__ == "__main__":
    parser = argparse.ArgumentParser(description = "Plot Values Data")
    parser.add_argument("input", help = "Input File")

    args = parser.parse_args()

    main(args.input)