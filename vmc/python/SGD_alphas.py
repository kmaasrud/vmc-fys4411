import pandas as pd
import os
import matplotlib.pyplot as plt


import numpy as np


#location for files and plots

PLOT_DIR = "../plots/"
DATA_DIR = "../data"
FILENAME_PLOT = 'SGD_alphas'
PLOT_DIR = "./"


#figure size and resolution
fig = plt.figure()
plt.style.use("seaborn")
#colour, linewith, linestyle
#boundaries
#plt.xlim(min(x)*1.1, max(x)*1.1)
plt.ylim(0.15, 0.95)
#legend
plt.legend(loc = 'best', prop = {'size':14}, frameon = False)
plt.rc('font', size=10)
plt.rc('axes', titlesize=12)
plt.xlabel("Iterations")
plt.ylabel(r"$\alpha$")
plt.title(r"SGD: Start $\alpha$")




start_alphas = ["0.2","0.3", "0.4", "0.5", "0.6", "0.7", "0.8", "0.9"]

for start_alpha in start_alphas:
    dim = 3

    DATA_DIR = f"../data/sgd_noninteracting/start-alpha/start-alpha_{start_alpha}.csv"

    df = pd.read_csv(DATA_DIR)

    energy = df["Energy"]
    
    alpha = df["Alpha"]
    x = np.linspace(0, len(alpha), len(alpha))
    plt.plot(x, alpha, label  = start_alpha, linewidth = 2)

plt.legend()
plt.draw()
plt.show()
#plt.save_fig(PLOT_DIR + "SGD_start-alpha.png")


