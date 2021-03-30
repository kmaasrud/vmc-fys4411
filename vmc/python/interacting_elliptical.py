import os
import matplotlib.pyplot as plt
plt.style.use("seaborn")
from lib.utils import read_csv, find_cargo_root

data_folder = os.path.join(find_cargo_root(), "data", "interacting_elliptical")
save_folder = os.path.join(os.path.dirname(find_cargo_root()), "doc", "assets")
if not os.path.isdir(save_folder):
    os.mkdir(save_folder)

bruteforce = read_csv(os.path.join(data_folder, "BruteForceMetropolis.csv"))
importance = read_csv(os.path.join(data_folder, "ImportanceMetropolis.csv"))

plt.plot(bruteforce["Alpha"], bruteforce["Energy"], "-o", label="Brute force")
plt.plot(importance["Alpha"], importance["Energy"], "-o", label="Importance")

plt.xlabel(r"$\alpha$")
plt.ylabel(r"$\langle E \rangle\ (\hbar\omega)$")
plt.legend()

plt.savefig(os.path.join(save_folder, "interacting_elliptical.png"))
