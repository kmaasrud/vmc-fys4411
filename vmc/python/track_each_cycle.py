import os
import matplotlib.pyplot as plt
from lib.utils import read_csv, find_cargo_root

data_folder = os.path.join(find_cargo_root(), "data")

true_val = 15

bruteforce = read_csv(os.path.join(data_folder, "track_each_cycle", "BruteForceMetropolis.csv"))
bruteforce_error = [abs(val - true_val) for val in bruteforce["Energy"][-100:]]
importance = read_csv(os.path.join(data_folder, "track_each_cycle", "ImportanceMetropolis.csv"))
importance_error = [abs(val - true_val) for val in importance["Energy"][-100:]]

plt.plot(bruteforce["MCCycles"][-100:], bruteforce_error)
plt.plot(importance["MCCycles"][-100:], importance_error)
plt.show()
