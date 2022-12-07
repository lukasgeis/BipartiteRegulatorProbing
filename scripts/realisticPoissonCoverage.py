import os
import pickle
import argparse

def main(inputDir: str, output: str):
    X = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]
    Y = [[[0.0 for i in range(len(X))] for j in range(2)] for k in range(2)]
    Z = [0 for i in range(len(X))]

    for file in os.listdir(inputDir):
        header = None
        data = {}
        for line in open(os.path.join(inputDir, file), "r").readlines():
            content = [x.split(": ")[1] for x in line.split(" -- ")]
            if header is None:
                header = (int(content[1]), int(content[2]), int(content[3]))
            if content[0] not in data:
                data[content[0]] = {}
            if content[11] not in data[content[0]]:
                data[content[0]][content[11]] = []
            data[content[0]][content[11]].append((content[5], int(content[6]), int(content[7]), int(content[10]))) 
        if (header[0] // 16) not in X:
            break
        idx = X.index(header[0] // 16)

        totalSum = [0.0, 0.0]
        totalNum = [0, 0]
        totWorst = [1.0, 1.0]
        for bpr in data:
            for instance in data[bpr]:
                opt = {}  
                for algo in data[bpr][instance]:
                    if algo[0] == "OPT":
                        opt[algo[2]] = algo[3]
                    elif algo[0] == "AMP":
                        ratio = algo[3] / opt[algo[2]]
                        totalSum[0] += ratio
                        totalNum[0] += 1
                        if ratio < totWorst[0]:
                            totWorst[0] = ratio
                    else:
                        ratio = algo[3] / opt[algo[2]]
                        totalSum[1] += ratio
                        totalNum[1] += 1
                        if ratio < totWorst[1]:
                            totWorst[1] = ratio
        Y[0][0][idx] = totalSum[0] / max(1, totalNum[0])
        Y[0][1][idx] = totWorst[0]
        Y[1][0][idx] = totalSum[1] / max(1, totalNum[1])
        Y[1][1][idx] = totWorst[1]
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