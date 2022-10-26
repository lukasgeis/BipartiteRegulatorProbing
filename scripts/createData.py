from __future__ import annotations

import os
import math
import random
import argparse

def roundToTwoDecimalPlaces(value: float) -> float:
    """
    Rounds a given float to two decimal places

    Parameters:
        value: float    => Float to round
    
    Returns:
        float           => Rounded float
    """
    return float("{:.2f}".format(value))


def generateDistribution(valueSpace: int) -> list[float]:
    """
    Generates a discrete Distribution for a given Value-Space and Number of Probabilities

    Parameters:
        valueSpace: int             => Size of Value-Space = [0,1,...,valueSpace]
    
    Returns:
        list[float]                 => List of probabilities for each i in {0,1,...,vs}
    
    Note:
        Probabilities are always rounded to two decimal places.
        0 is always guaranteed to appear as a possible value in the distribution.
        The probabilities sum up to 1.
    """

    # Prevent Wrong Parameters
    valueSpace = max(1, valueSpace)
    
    # Generate Probabilities
    allProbabilities = [random.uniform(0.5, 1.0) for k in range(valueSpace + 1)]
    totalProbabilities = sum(allProbabilities)

    allProbabilities = [roundToTwoDecimalPlaces(k / totalProbabilities - 0.01) for k in allProbabilities]
    totalProbabilities = sum(allProbabilities)

    # Prevent that probabilities do not add up to 1.0 due to rounding
    if not math.isclose(totalProbabilities, 1.0):
        allProbabilities[0] = roundToTwoDecimalPlaces(allProbabilities[0] + (1.0 - totalProbabilities))
        totalProbabilities = sum(allProbabilities)
    
    if any([k < 0.0 for k in allProbabilities]):
        print("ERROR")

    # Return Distribution in the form of (value, probability) as list
    return allProbabilities

def generateGraph(output: str, name: str, naMin: int, naMax: int, nbMin: int, nbMax: int, vsMin: int, vsMax: int) -> str:
    """
    Generates a complete bipartite Graph where each edge has an unique independent discrete distribution over the same Value-Space

    Parameters:
        output: str     => Output Directory for Graph file
        name: str       => Custom Name for Graph ('Random' at default)
        naMin: int      => Minimum Number of Regulators (left Side of bipartite Graph)
        naMax: int      => Maximum Number of Regulators (left Side of bipartite Graph)
        nbMin: int      => Minimum Number of Positions (right Side of bipartite Graph)
        nbMax: int      => Maximum Number of Positions (right Side of bipartite Graph)
        vsMin: int      => Minimum Number of Value-Space size
        vsMax: int      => Maximum Number of Value-Space size

    Returns:
        Simple (Debug) Output String containing the outpath of the generated file
    """

    # Create Name and Path
    if name is None:
        name = "Random"
    if not os.path.exists(output):
        os.makedirs(output)
    
    # Choose Values for Number of Regulators, Number of Positions, Value-Space Size
    na = random.randint(max(1, naMin), max(1, naMax))
    nb = random.randint(max(1, nbMin), max(1, nbMax))
    vs = random.randint(max(1, vsMin), max(1, vsMax))

    # Create Outpath (if file already exists, increment index at end of filename)
    outpath = os.path.join(output, name + "_na" + str(na) + "_nb" + str(nb) + "_vs" + str(vs))
    number = 0
    while True:
        if not os.path.exists(outpath + "_" + str(number)):
            outpath += "_" + str(number)
            break
        else:
            number += 1

    # Create and write to file
    with open(outpath, "a") as outfile:
        # Header
        outfile.write(str(name) + " " + str(na) + " " + str(nb) + " " + str(vs) + "\n")

        # Distributions for each edge
        for ka in range(1, na + 1):
            for kb in range(1, nb + 1):
                outfile.write(str(ka) + "-" + str(kb) + " " + ",".join([str(k) for k in generateDistribution(vs)]) + "\n")
        
        # Flush Outfile
        outfile.flush()

    # Obligatory Output Message
    return "[Successfully created " + str(outpath) + "]"


if __name__ == "__main__":
    """
    Called when running script with Python. Reads Command-Line-Arguments and generates Graphs accordingly.
    """

    # Parser Arguments
    parser = argparse.ArgumentParser(description = "Create Bipartite-Regulator-Probing Graph Instances")
    parser.add_argument("--number", type = int, help = "Number of Instances to generate", required = True)
    parser.add_argument("--na_min", metavar = "", type = int, help = "Minimum Number of Regulators", required = True)
    parser.add_argument("--na_max", metavar = "", type = int, help = "Maximum Number of Regulators", required = True)
    parser.add_argument("--nb_min", metavar = "", type = int, help = "Minimum Number of Positions", required = True)
    parser.add_argument("--nb_max", metavar = "", type = int, help = "Maximum Number of Positions", required = True)
    parser.add_argument("--vs_min", metavar = "", type = int, help = "Minimum Size of Value Space", required = True)
    parser.add_argument("--vs_max", metavar = "", type = int, help = "Maximum Size of Value Space", required = True)
    parser.add_argument("--output", metavar = "", help = "Output Directory for generated Graphs", required = True)
    parser.add_argument("--name", metavar = "", help = "Custom Base-Name for Graphs ('Random' at Default)")

    args = parser.parse_args()
    
    # Generate Graphs
    for k in range(args.number):
        print(generateGraph(args.output, args.name, args.na_min, args.na_max, args.nb_min, args.nb_max, args.vs_min, args.vs_max))

