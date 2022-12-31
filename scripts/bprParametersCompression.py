import os
import pickle
import argparse

def main(inputDir: str, output: str):
    X = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]
    Y = [[[[[0.0, 0.0] for i in range(len(X))] for j in range(2)] for k in range(4)] for l in range(3)]
    Z = [0 for i in range(len(X))]

    for file in os.listdir(inputDir):
        header = None
        if int(file[file.index("_"):]) > 20:
            continue
        data = {}
        for line in open(os.path.join(inputDir, file), "r").readlines():
            content = [x.split(": ")[1] for x in line.split(" -- ")]
            if header is None:
                header = int(content[1])
            if content[0] not in data:
                data[content[0]] = {}
            if content[11] not in data[content[0]]:
                data[content[0]][content[11]] = []
            data[content[0]][content[11]].append((content[5], int(content[6]), int(content[7]), int(content[10]))) 
        if (header // 16) not in X:
            break
        idx = X.index(header // 16)

        totValues = [[[[1.0, 1.0], [0.0, 0], [0.0, 0]] for i in range(4)] for j in range(3)]
        for bpr in data:
            for instance in data[bpr]:
                opt = {}  
                for algo in data[bpr][instance]:
                    if algo[0] == "OPT":
                        opt[algo[2]] = algo[3]
                    elif algo[0] == "AMP":
                        k = (algo[1] // (header // 4)) - 1
                        l = (algo[2] // (algo[1] // 4)) - 1
                        ratio = algo[3] / opt[algo[2]]
                        totValues[k][l][1][0] += ratio
                        totValues[k][l][1][1] += 1
                        if ratio < totValues[k][l][0][0]:
                            totValues[k][l][0][0] = ratio
                    else:
                        k = (algo[1] // (header // 4)) - 1
                        l = (algo[2] // (algo[1] // 4)) - 1
                        ratio = algo[3] / opt[algo[2]]
                        totValues[k][l][2][0] += ratio
                        totValues[k][l][2][1] += 1
                        if ratio < totValues[k][l][0][1]:
                            totValues[k][l][0][1] = ratio
        for k in range(3):
            for l in range(4):
                Y[k][l][0][idx][0] = totValues[k][l][0][0]
                Y[k][l][1][idx][0] = totValues[k][l][0][1]
                Y[k][l][0][idx][1] = totValues[k][l][1][0] / max(1, totValues[k][l][1][1])
                Y[k][l][1][idx][1] = totValues[k][l][2][0] / max(1, totValues[k][l][2][1])
        Z[idx] = len(data)
    
    fullData = {
        "x": X,
        "y": Y,
        "z": Z,
    }
    
    with open(output, "wb") as outfile:
        pickle.dump(fullData, outfile)





if __name__ == "__main__":
    parser = argparse.ArgumentParser(description = "Compress COV data")
    parser.add_argument("--input", metavar = "", help = "Input Directory", required = True)
    parser.add_argument("--output", metavar = "", help = "Output File", required = True)

    args = parser.parse_args()

    main(args.input, args.output)