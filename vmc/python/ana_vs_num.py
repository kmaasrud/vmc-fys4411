#Compare energy vs. alpha analytic vs. bruteforce/importance 
from lib import analytic as ana
from lib import pathmaker as pth
from lib.blocking import block
import os
import numpy as np
import pandas as pd
import time
import matplotlib.pyplot as plt
plt.style.use("seaborn")

dim = 1
N = 1

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
time_BF = df_BF["Time"].values.tolist()

#importance:
df_IM = pd.read_csv(importance)
energy_IM = df_IM["Energy"].values.tolist()
alpha_IM = df_IM["Alpha"].values.tolist()
time_IM = df_IM["Time"].values.tolist()


#analytic
alphas = np.linspace(0.2, 0.8, 50)
#[0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75, 0.8]
energy = []
time_alpha = []

#calculate analytic energy

df_analytical = pd.DataFrame()
for a in alphas:
    start_time = time.time()
    E = ana.AnalyticLocalEnergy(a, dim, N, StepSize = 0.1)
    energy.append(E)
    time_alpha.append((time.time() - start_time)*1000)
data = {"Alpha":alphas,"Energy":energy,'Time[ms]':time_alpha}
analytical = pd.DataFrame.from_dict(data)

#folder = f"../plots/ana_vs_num/{dim}D_{N}N/"
#print(analytical)
#analytical.to_csv(folder + 'File_Name.csv', index = False)

time_ana = analytical['Time[ms]'].values.tolist()

x = np.array(analytical['Energy'].values.tolist())
(mean, var) = block(x) 
std = np.sqrt(var)


data ={'Mean':[mean], 'STDev':[std]}
frame = pd.DataFrame(data,index=['Values'])




#plots:
#analytic vs. bruteforce
plt.plot(alphas,energy, label="Analytic")
plt.plot(alpha_BF,energy_BF, label="Brute Force Metropolis")
plt.plot(alpha_IM,energy_IM, label="Importance Sampling")
plt.title(f"Brute Force metropolis vs. analytic for {dim}D and {N} particles")
plt.xlabel(r"$\alpha$")
plt.ylabel(r"$\langle E\rangle$ ($\hbar\omega$)")
plt.legend(loc = 'best', prop = {'size':13}, frameon = True)

plt.savefig(os.path.join(save_folder, f"EnergyAlpha_BF_{dim}D_{N}N.png"))
plt.clf()
"""
#Analytic vs. Importance
plt.plot(alphas, energy, label="Analytic")
plt.plot(alpha_IM,energy_IM, label="Importance Sampling")
plt.title(f"Importance sampling vs. analytic for {dim}D and {N} particles")
plt.xlabel(r"$\alpha$")
plt.ylabel("$\langle E\rangle$ ($\hbar\omega$)"$ ($\hbar\omega$)")
plt.legend(loc = 'best', prop = {'size':13}, frameon = True)
plt.savefig(os.path.join(save_folder, f"EnergyAlpha_IM_{dim}D_{N}N.png"))

plt.clf()
"""

plt.plot(alpha_IM,time_IM, label="Importance Samplin time")
plt.plot(alpha_BF,time_BF, label="Brute Force Metropolis time")
plt.plot(alphas,time_alpha, label="Analytic time")

plt.clf()
