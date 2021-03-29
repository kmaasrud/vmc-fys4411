import pandas as pd
import os
import matplotlib.pyplot as plt


import numpy as np


#location for files and plots

PLOT_DIR = "../plots/"
DATA_DIR = "../data"
FILENAME_PLOT = 'SGD_learning-rate'
PLOT_DIR = "./"


#figure size and resolution
fig = plt.figure()
plt.style.use("seaborn")
#colour, linewith, linestyle
#boundaries
#plt.xlim(min(x)*1.1, max(x)*1.1)
#plt.ylim(0.15, 0.55)
#legend
plt.legend(loc = 'best', prop = {'size':14}, frameon = False)
plt.rc('font', size=10)
plt.rc('axes', titlesize=12)
plt.xlabel("Iterations")
plt.ylabel(r"$\alpha$")
plt.title("SGD: Learning rate")




learning_rates = ["0.00005", "0.0001", "0.0002", "0.0004", "0.0008", "0.0016", "0.0032", "0.0064"]

for learning_rate in learning_rates:
    dim = 3

    DATA_DIR = f"../data/sgd_noninteracting/learning-rate/learning-rate_{learning_rate}.csv"

    df = pd.read_csv(DATA_DIR)

    energy = df["Energy"]
    
    alpha = df["Alpha"]
    x = np.linspace(0, len(alpha), len(alpha))
    plt.plot(x, alpha, label  = learning_rate, linewidth = 2)

plt.legend()
plt.draw()
plt.show()
#plt.save_fig(PLOT_DIR + "SGD_learning-rate.png")


