import os
import pickle
import argparse
import matplotlib.pyplot as plt
plt.rcParams['text.usetex'] = True
plt.rcParams['axes.titlepad'] = 10

def main(inputDir: str, sGoal: str):
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
        data[goal] = [{
            "x": [],
            "y": []
        } for i in range(3)]
        for entry in fetchedData[goal]:
            for i in range(3):
                if fetchedData[goal][entry][i][1] > 0:
                    data[goal][i]["x"].append(entry)
                    data[goal][i]["y"].append(fetchedData[goal][entry][i][0] / fetchedData[goal][entry][i][1])
        for i in range(3):
            data[goal][i]["y"] = [t for _,t in sorted(zip(data[goal][i]["x"], data[goal][i]["y"]), key = lambda pair: pair[0])]
            data[goal][i]["x"] = sorted(data[goal][i]["x"])

    colors = {
        "max": "red",
        "sum": "blue",
        "cov": "green"
    }

    plt.plot(data[sGoal][2]["x"], data[sGoal][2]["y"], "-", color = "red", label = str(sGoal.upper()) + " - OPT" )
    plt.plot(data[sGoal][0]["x"], data[sGoal][0]["y"], "--", color = "blue", label = str(sGoal.upper()) + " - AMP")
    plt.plot(data[sGoal][1]["x"], data[sGoal][1]["y"], ":", color = "green", label = str(sGoal.upper()) + " - NAMP")

    plt.title("Average Running Time per Goal", fontsize = 23)
    plt.xlabel(r'$n_A \cdot n_B \cdot |\mathcal{V}|$', fontsize = 20)
    plt.ylabel(r'time in $s$', fontsize = 20)
    plt.legend(prop = {"size": 17})

    plt.show()



if __name__ == "__main__":
    parser = argparse.ArgumentParser(description = "Plot Values Data")
    parser.add_argument("input", help = "Input Directory")
    parser.add_argument("--goal", metavar = "", required = True)

    args = parser.parse_args()

    main(args.input, args.goal)