import argparse
import pandas as pd

parser = argparse.ArgumentParser()
parser.add_argument("file", type=str)

args = parser.parse_args()

data = pd.read_json(args.file, lines=True)

occ = {
    "opt": {},
    "amp": {},
    "namp": {}
}

for _, row in data.iterrows():
    for alg in ["opt", "amp", "namp"]:
        for entry in row[alg]:
            triple = entry[0]
            for i in range(3):
                if triple[i] not in occ[alg]:
                    occ[alg][triple[i]] = 0
                occ[alg][triple[i]] += 1

for alg in ["opt", "amp", "namp"]:
    print("Algorithm: ", alg.upper())
    for k, v in sorted(
        occ[alg].items(), key=lambda item: item[1], reverse=True
    ):
        print(k, " - ", v)
    print("\n")
