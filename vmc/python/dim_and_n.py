import os
import sys
import matplotlib.pyplot as plt
import numpy as np
plt.style.use("seaborn")
from lib.utils import read_csv, find_cargo_root

mean = lambda x: sum(x) / len(x)
std = lambda x: np.sqrt(sum(map(lambda y: (y - mean(x))**2, x)) / len(x))

data_folder = os.path.join(find_cargo_root(), "data")
save_folder = os.path.join(os.path.dirname(find_cargo_root()), "doc", "assets", "dim_and_n")
if not os.path.isdir(save_folder):
    os.mkdir(save_folder)

all_data = []
for core_num in range(2, 10):
    all_data.append(read_csv(os.path.join(data_folder, "dim_and_n", f"ThreadId({core_num}).csv")))

# List comprehension madness ahead
transposed_data = {key: [[] for _ in range(len(all_data[0]["Energy"]))] for key in all_data[0].keys()}
for i, data in enumerate(all_data):
    for key in data.keys():
        for j, val in enumerate(data[key]):
            transposed_data[key][j].append(val)

mean_data = {key: [mean(thread_vals) for thread_vals in vals] for key, vals in transposed_data.items()}
std_data = {key: [std(thread_vals) for thread_vals in vals] for key, vals in transposed_data.items()}

for N in list(set(mean_data["N"])):
    x1 = []; y1 = []
    x2 = []; y2 = []
    x3 = []; y3 = []
    for n, d, a, e in zip(mean_data["N"], mean_data["Dim"], mean_data["Alpha"], mean_data["Energy"]):
        if n == N:
            if d == 1:
                x1.append(a), y1.append(e / N)
            if d == 2:
                x2.append(a), y2.append(e / N)
            if d == 3:
                x3.append(a), y3.append(e / N)

    plt.plot(x1, y1, "-o", label="Dimensionality 1")
    plt.plot(x2, y2, "-o", label="Dimensionality 2")
    plt.plot(x3, y3, "-o", label="Dimensionality 3")
    mean_min = mean([x[y.index(min(y))] for x, y in [(x1, y1), (x2, y2), (x3, y3)]])
    plt.vlines(mean_min, min(y1), max(y3), colors="black", linestyles="dashed", label=f"Mean minimum: {round(mean_min, 2)}")
    plt.title(f"Number of particles N = {int(N)}")
    plt.xlabel(r"$\alpha$")
    plt.ylabel(r"$\frac{\langle E\rangle}{N}$ ($\hbar\omega$)")
    plt.legend(loc = 1, prop = {'size':13}, frameon = True)
    plt.savefig(os.path.join(save_folder, f"EnergyAlpha_{N}N.png"))

    plt.clf()

    x1 = []; y1 = []
    x2 = []; y2 = []
    x3 = []; y3 = []
    for n, d, a, e in zip(mean_data["N"], mean_data["Dim"], mean_data["Alpha"], std_data["Energy"]):
        if n == N:
            if d == 1:
                x1.append(a), y1.append(e)
            if d == 2:
                x2.append(a), y2.append(e)
            if d == 3:
                x3.append(a), y3.append(e)

    plt.clf()
    plt.plot(x1, y1, "-o", label="Dimensionality 1")
    plt.plot(x2, y2, "-o", label="Dimensionality 2")
    plt.plot(x3, y3, "-o", label="Dimensionality 3")
    plt.title(f"Number of particles N = {int(N)}")
    plt.xlabel(r"$\alpha$")
    plt.ylabel(r"$\sigma_{\langle E\rangle}$")
    plt.legend(loc = 1, prop = {'size':13}, frameon = True)
    plt.savefig(os.path.join(save_folder, f"std_{N}N.png"))
    
    plt.clf()
