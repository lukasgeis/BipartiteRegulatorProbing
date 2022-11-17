import os
import pickle
import argparse

def main(inputDir: str, output: str, worst: bool):
    compressedData = []

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
        
        cBprData = {
            1: {
                1: [],
                2: [],
                3: [],
                4: []
            },
            2: {
                1: [],
                2: [],
                3: [],
                4: []
            },
            3: {
                1: [],
                2: [],
                3: [],
                4: []
            }
        }
        for bpr in data:
            bprData = {
                1: {
                    1: [],
                    2: [],
                    3: [],
                    4: []
                },
                2: {
                    1: [],
                    2: [],
                    3: [],
                    4: []
                },
                3: {
                    1: [],
                    2: [],
                    3: [],
                    4: []
                }
            }
            for instance in data[bpr]:
                iOpt = {}
                iData = {}
                for algo in data[bpr][instance]:
                    if algo[0] == "OPT":
                        iOpt[algo[2]] = algo[3]
                    else:
                        if algo[1] not in iData:
                            iData[algo[1]] = {}
                        if algo[2] not in iData[algo[1]]:
                            iData[algo[1]][algo[2]] = [0.0, 0.0]
                        if algo[0] == "AMP":
                            iData[algo[1]][algo[2]][0] = algo[3] / iOpt[algo[2]]
                        else:
                            iData[algo[1]][algo[2]][1] = algo[3] / iOpt[algo[2]]
                k = 1
                for kKey in iData:
                    l = 1
                    for lKey in iData[kKey]:
                        bprData[k][l].append(iData[kKey][lKey])
                        l += 1
                    k += 1
            for k in [1,2,3]:
                for l in [1,2,3,4]:
                    if worst:
                        vAmp = 1.0
                        vNamp = 1.0
                        for entry in bprData[k][l]:
                            if entry[0] < vAmp:
                                vAmp = entry[0]
                            if entry[1] < vNamp:
                                vNamp = entry[1]
                        cBprData[k][l].append((vAmp, vNamp))
                    else:
                        vAmp = 0.0
                        cAmp = 0
                        vNamp = 0.0
                        cNamp = 0
                        for entry in bprData[k][l]:
                            if entry[0] > 0.0:
                                vAmp += entry[0]
                                cAmp += 1
                            if entry[1] > 0.0:
                                vNamp += entry[1]
                                cNamp += 1
                        cBprData[k][l].append((vAmp / max(1, cAmp), vNamp / max(1, cNamp)))
        tBprData = {
            1: {
                1: None,
                2: None,
                3: None,
                4: None
            },
            2: {
                1: None,
                2: None,
                3: None,
                4: None
            },
            3: {
                1: None,
                2: None,
                3: None,
                4: None
            }
        }
        for k in [1,2,3]:
            for l in [1,2,3,4]:
                if worst:
                    vAmp = 1.0
                    vNamp = 1.0
                    for entry in bprData[k][l]:
                        if entry[0] < vAmp:
                            vAmp = entry[0]
                        if entry[1] < vNamp:
                            vNamp = entry[1]
                    tBprData[k][l] = (vAmp, vNamp)
                else:
                    vAmp = 0.0
                    cAmp = 0
                    vNamp = 0.0
                    cNamp = 0
                    for entry in cBprData[k][l]:
                        for entry in bprData[k][l]:
                            if entry[0] > 0.0:
                                vAmp += entry[0]
                                cAmp += 1
                            if entry[1] > 0.0:
                                vNamp += entry[1]
                                cNamp += 1
                        tBprData[k][l] = (vAmp / max(1, cAmp), vNamp / max(1, cNamp))

        compressedData.append((header, tBprData))
        
    with open(output, "wb") as outfile:
        pickle.dump(compressedData, outfile)            



if __name__ == "__main__":
    parser = argparse.ArgumentParser(description = "Compress COV data")
    parser.add_argument("--input", metavar = "", help = "Input Directory", required = True)
    parser.add_argument("--output", metavar = "", help = "Output File", required = True)
    parser.add_argument("--worst", metavar = "", type = bool, help = "Worst case instead of average")

    args = parser.parse_args()

    main(args.input, args.output, args.worst)