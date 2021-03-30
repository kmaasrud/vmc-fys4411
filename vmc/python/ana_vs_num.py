#Compare energy vs. alpha analytic vs. bruteforce/importance 

from lib import analytic as ana
from lib import pathmaker as pth
import os
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
plt.style.use("seaborn")

dim = 1
N = 10

save_folder = f"../plots/ana_vs_num/{dim}D_{N}N/"
data_folder = f"../data/ana_vs_num/{dim}D_{N}N/"

pth.pathmaker(data_folder)
pth.pathmaker(save_folder)

bruteforce = os.path.join(data_folder, "BruteForceMetropolis.csv")
importance = os.path.join(data_folder, "ImportanceMetropolis.csv")


#bruteforce:
df_BF = pd.read_csv(bruteforce)
energy_BF = df_BF["Energy"].values.tolist()
alpha_BF = df_BF["Alpha"].values.tolist()


#importance:
df_IM = pd.read_csv(importance)
energy_IM = df_IM["Energy"].values.tolist()
alpha_IM = df_IM["Alpha"].values.tolist()

alphas = alpha_IM
#analytic

energy = []
pos = ana.init_pos(3, N)
for a in alphas:
    energy.append(ana.analytic_energy(dim, N, a, pos))
   
#plots:
#analytic vs. bruteforce
plt.plot(alphas,energy, "-o", label="Analytic")
plt.plot(alpha_BF,energy_BF, "-o", label="Brute Force Metropolis")

plt.title(f"Brute Force metropolis vs. analytic for {dim}D and {N} particles")
plt.xlabel(r"$\alpha$")
plt.ylabel(r"$\frac{\langle E\rangle}{N}$ ($\hbar\omega$)")
plt.legend(loc = 'best', prop = {'size':13}, frameon = True)
plt.savefig(os.path.join(save_folder, f"EnergyAlpha_BF_{dim}D_{N}N.png"))

plt.clf()

#Analytic vs. Importance
plt.plot(alphas, energy, "-o", label="Analytic")
plt.plot(alpha_IM,energy_IM, "-o", label="Importance Sampling")

plt.title(f"Importance sampling vs. analytic for {dim}D and {N} particles")
plt.xlabel(r"$\alpha$")
plt.ylabel(r"$\frac{\langle E\rangle}{N}$ ($\hbar\omega$)")
plt.legend(loc = 'best', prop = {'size':13}, frameon = True)
plt.savefig(os.path.join(save_folder, f"EnergyAlpha_IM_{dim}D_{N}N.png"))

plt.clf()


