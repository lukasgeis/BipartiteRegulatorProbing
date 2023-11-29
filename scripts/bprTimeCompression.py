import os
import pickle
import argparse

def main(inputDir: str, output: str):
    X = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]
    Y = [[0.0, 0.0, 0.0] for x in range(len(X))]
    for file in os.listdir(inputDir):
        header = None
        if int(file[(file.index("_") + 1):]) > 20:
            continue
        totTime = [[0.0, 0] for i in range(3)]
        for line in open(os.path.join(inputDir, file), "r").readlines():
            content = [x.split(": ")[1] for x in line.split(" -- ")]
            if header is None:
                header = int(content[1])
            if (header // 16) not in X:
                break 
            if content[5] == "AMP":
                idx = 0
            elif content[5] == "NAMP":
                idx = 1
            else:
                idx = 2
            totTime[idx][0] += float(content[8])
            totTime[idx][1] += 1
        for i in range(3):
            Y[X.index(header // 16)][i] = totTime[i][0] / max(1, totTime[i][1]) 
    
    fullData = {
        "x": X,
        "y": Y,
    }

    with open(output, "wb") as outfile:
        pickle.dump(fullData, outfile)



if __name__ == "__main__":
    parser = argparse.ArgumentParser(description = "Compress COV data")
    parser.add_argument("--input", metavar = "", help = "Input Directory", required = True)
    parser.add_argument("--output", metavar = "", help = "Output File", required = True)

    args = parser.parse_args()

    main(args.input, args.output)