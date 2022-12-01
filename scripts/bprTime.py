import os
import pickle
import argparse

def main(inputDir: str, output: str):
    data = {}
    for file in os.listdir(inputDir):
        header = None
        for line in open(os.path.join(inputDir, file), "r").readlines():
            content = [x.split(": ")[1] for x in line.split(" -- ")]
            if header is None:
                header = int(content[1]) * int(content[2]) * int(content[3])
            if header not in data:
                data[header] = [[0.0, 0], [0.0, 0], [0.0, 0]]
            if content[5] == "AMP":
                idx = 0
            elif content[5] == "NAMP":
                idx = 1
            else:
                idx = 2
            data[header][idx][0] += float(content[8])
            data[header][idx][1] += 1
    
    with open(output, "wb") as outfile:
        pickle.dump(data, outfile)



if __name__ == "__main__":
    parser = argparse.ArgumentParser(description = "Compress COV data")
    parser.add_argument("--input", metavar = "", help = "Input Directory", required = True)
    parser.add_argument("--output", metavar = "", help = "Output File", required = True)

    args = parser.parse_args()

    main(args.input, args.output)