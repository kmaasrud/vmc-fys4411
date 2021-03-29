#Compare energy vs. alpha analytic vs. bruteforce/importance 

from lib import analytic as ana
import os
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

save_folder = "../plots/analytic_vs_numeric"
if not os.path.isdir(save_folder):
    os.mkdir(save_folder)

#bruteforce:
df_BRUTEFORCE = pd.read_csv(DATA_DIR_BRUTEFORCE)
energy_BF = df_BRUTEFORCE["Energy"].values.tolist()
alpha_BF = df_BRUTEFORCE["Alpha"].values.tolist()
cpu_BF = df_BRUTEFORCE["cpu"].values.tolist()

#importance:
df_IMPORTANCE = pd.read_csv(DATA_DIR_IMPORTANCE)
energy_IMP = df_IMPORTANCE["Energy"].values.tolist()
alpha_IMP = df_IMPORTANCE["Alpha"].values.tolist()
cpu_IMP = df_IMPORTANCE["cpu"].values.tolist()

#analytic
for dim in range(1, dim + 1):
    for N in particles:
        energy = []
        for a in alphas:
            pos = init_pos(1, N)
            energy.append(analytic_energy(dim, N, a ,pos)/(dim*N))
        
#plots:
        #analytic vs. bruteforce
        plt.plot(energy,alphas, "-o", label="Analytic")
        plt.plot(energy_BF,alpha_BF, "-o", label="Brute Force Metropolis")
    
        plt.title(f"Brute Force metropolis vs. analytic for {dim}D and {N} particles")
        plt.xlabel(r"$\alpha$")
        plt.ylabel(r"$\frac{\langle E\rangle}{N}$ ($\hbar\omega$)")
        plt.legend(loc = 'best', prop = {'size':13}, frameon = True)
        plt.savefig(os.path.join(save_folder, f"EnergyAlpha_BF_{dim}D_{N}N.png"))

        plt.clf()

        #analytic vs. Importance
        plt.plot(energy,alphas, "-o", label="Analytic")
        plt.plot(energy_IMP,alpha_IMP, "-o", label="Importance Sampling")
        
        plt.title(f"Importance sampling vs. analytic for {dim}D and {N} particles")
        plt.xlabel(r"$\alpha$")
        plt.ylabel(r"$\frac{\langle E\rangle}{N}$ ($\hbar\omega$)")
        plt.legend(loc = 'best', prop = {'size':13}, frameon = True)
        plt.savefig(os.path.join(save_folder, f"EnergyAlpha_BF_{dim}D_{N}N.png"))

        plt.clf()
       

