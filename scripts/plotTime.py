import os
import pickle
import argparse
import matplotlib.pyplot as plt
plt.rcParams['text.usetex'] = True
plt.rcParams['axes.titlepad'] = 10

def main(inputDir: str):
    fetchedData = {}
    for file in os.listdir(inputDir):
        with open(os.path.join(inputDir, file), "rb") as infile:
            if "MAX" in file:
                fetchedData["max"] = pickle.load(infile)
            elif "SUM" in file:
                fetchedData["sum"] = pickle.load(infile)
            else:
                fetchedData["cov"] = pickle.load(infile)
    data = {}
    for goal in fetchedData:
        data[goal] = {
            "x": [],
            "y": []
        }
        for entry in fetchedData[goal]:
            if fetchedData[goal][entry][1] > 0:
                data[goal]["x"].append(entry)
                data[goal]["y"].append(fetchedData[goal][entry][0] / fetchedData[goal][entry][1])

        data[goal]["y"] = [t for _,t in sorted(zip(data[goal]["x"], data[goal]["y"]), key = lambda pair: pair[0])]
        data[goal]["x"] = sorted(data[goal]["x"])

    colors = {
        "max": "red",
        "sum": "blue",
        "cov": "green"
    }

    for goal in data:
        plt.plot(data[goal]["x"], data[goal]["y"], "-", color = colors[goal], label = goal.upper())

    plt.title("Average Running Time per Goal", fontsize = 23)
    plt.xlabel(r'$n_A \cdot n_B \cdot |\mathcal{V}|$', fontsize = 20)
    plt.ylabel(r'time in $s$', fontsize = 20)
    plt.legend(prop = {"size": 17})

    plt.show()



if __name__ == "__main__":
    parser = argparse.ArgumentParser(description = "Plot Values Data")
    parser.add_argument("input", help = "Input Directory")

    args = parser.parse_args()

    main(args.input)