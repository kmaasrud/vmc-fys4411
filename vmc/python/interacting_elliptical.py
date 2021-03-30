import os
import sys
import matplotlib.pyplot as plt
import numpy as np
plt.style.use("seaborn")
from lib.utils import read_csv, find_cargo_root

mean = lambda x: sum(x) / len(x)
std = lambda x: np.sqrt(sum(map(lambda y: (y - mean(x))**2, x)) / len(x))

data_folder = os.path.join(find_cargo_root(), "data", "interacting_elliptical")
save_folder = os.path.join(os.path.dirname(find_cargo_root()), "doc", "assets")
if not os.path.isdir(save_folder):
    os.mkdir(save_folder)

bruteforce = read_csv(os.path.join(data_folder, "BruteForceMetropolis.csv"))
importance = read_csv(os.path.join(data_folder, "ImportanceMetropolis.csv"))

# List comprehension madness ahead
# transposed_data = {key: [[] for _ in range(len(all_data[0]["Energy"]))] for key in all_data[0].keys()}
# for i, data in enumerate(all_data):
#     for key in data.keys():
#         for j, val in enumerate(data[key]):
#             transposed_data[key][j].append(val)

plt.plot(bruteforce["Alpha"], bruteforce["Energy"], "-o", label="Brute force")
plt.plot(importance["Alpha"], importance["Energy"], "-o", label="Importance")

plt.xlabel(r"$\alpha$")
plt.ylabel(r"$\langle E \rangle\ (\hbar\omega)$")
plt.legend()

plt.savefig(os.path.join(save_folder, "interacting_elliptical.png"))
