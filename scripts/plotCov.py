import pickle
import matplotlib.pyplot as plt

data1 = pickle.load(open("logs/COV_COMPRESSED", "rb"))
data2 = pickle.load(open("logs/COV_WORST_COMPRESSED", "rb"))

x = [8,16,32,64,128,256,512,1024]
y = [[[0 for i in range(len(x))] for k in range (2)] for t in range(2)]

for entry in data1:
    vAmp = 0.0
    cAmp = 0.0
    vNamp = 0.0
    cNamp = 0.0
    for k in entry[1]:
        for l in entry[1][k]:
            if entry[1][k][l][0] > 0.0:
                vAmp += entry[1][k][l][0]
                cAmp += 1
            if entry[1][k][l][1] > 0.0:
                vNamp += entry[1][k][l][1]
                cNamp += 1
    y[0][0][x.index(entry[0][0])] = vAmp / max(1, cAmp)
    y[1][0][x.index(entry[0][0])] = vNamp / max(1, cNamp)

for entry in data2:
    vAmp = 1.0
    vNamp = 1.0
    for k in entry[1]:
        for l in entry[1][k]:
            if entry[1][k][l][0] < vAmp:
                vAmp = entry[1][k][l][0]
            if entry[1][k][l][1] < vNamp:
                vNamp = entry[1][k][l][1]
    y[0][1][x.index(entry[0][0])] = vAmp 
    y[1][1][x.index(entry[0][0])] = vNamp 

plt.plot(x, y[0][0])
plt.plot(x, y[0][1])
plt.plot(x, y[1][0])
plt.plot(x, y[1][1])

plt.title("Coverage Approximations")
plt.xlabel("Number of Regulators/Positions")
plt.ylabel("Ratio to Greedy-Offline-Approximation")

plt.legend(["AMP - Average", "AMP - Worst", "NAMP - Average", "NAMP - Worst"])

plt.show()