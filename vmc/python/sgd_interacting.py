import os
import matplotlib.pyplot as plt
plt.style.use("seaborn")
from lib.utils import read_csv, find_cargo_root

data_folder = os.path.join(find_cargo_root(), "data", "sgd_interacting")
save_folder = os.path.join(os.path.dirname(find_cargo_root()), "doc", "assets", "plots")
if not os.path.isdir(save_folder):
    os.mkdir(save_folder)

bruteforce = read_csv(os.path.join(data_folder, "BruteForceMetropolis.csv"))
importance = read_csv(os.path.join(data_folder, "ImportanceMetropolis.csv"))

plt.plot(bruteforce["SGDStep"], bruteforce["Alpha"], label="Brute force")
plt.plot(importance["SGDStep"], importance["Alpha"], label="Importance")

plt.xlabel("Number of SGD steps")
plt.ylabel(r"$\alpha$")
plt.legend(loc=7)
plt.savefig(os.path.join(save_folder, "sgd_interacting.png"))
