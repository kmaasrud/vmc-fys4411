
"""
Calculates the analytic local energy for a set of alphas and then plots the energy for the analytic, 
Brute Force Metropolis and Importance sampling algorithms. There is also a setup for plotting the time
used for the three mentioned methods. 
"""
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
N = 100


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
sum_time_BF = sum(time_BF)
print(f"Time used BF: {sum_time_BF}")

#importance:
df_IM = pd.read_csv(importance)
energy_IM = df_IM["Energy"].values.tolist()
alpha_IM = df_IM["Alpha"].values.tolist()
time_IM = df_IM["Time"].values.tolist()
sum_time_IM = sum(time_IM)
print(f"Time used IM: {sum_time_IM}")

[float(i) for i in energy_BF]
[float(i) for i in energy_IM]
[float(i) for i in alpha_IM]
[float(i) for i in alpha_BF]
[float(i)/2 for i in time_BF]
[float(i)/2 for i in time_IM]


#analytic
alphas = [0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75, 0.8]
energy = []
time_ana = []
start_time_tot = time.time()
#calculate analytic energy
df_analytical = pd.DataFrame()
for a in alphas:
    start_time = time.time()
    E = ana.AnalyticLocalEnergy(a, dim, N, StepSize = 0.1)
    energy.append(E)
    time_ana.append((time.time() - start_time))
tot_time = (time.time() - start_time_tot)
print(f"total time:  {tot_time}") 

data = {"Alpha":alphas,"Energy":energy,'Time':time_ana}
analytical = pd.DataFrame.from_dict(data)
time_ana = analytical['Time'].values.tolist()

"""
#if we want some statistics
x = np.array(analytical['Energy'].values.tolist())
(mean, var) = block(x) 
std = np.sqrt(var)
data ={'Mean':[mean], 'STDev':[std]}
frame = pd.DataFrame(data,index=['Values'])
"""

#plots:
#analytic vs. bruteforce
plt.plot(alphas,energy, label="Analytic")
plt.plot(alpha_BF,energy_BF, label="Brute Force Metropolis")
plt.plot(alpha_IM,energy_IM, label="Importance Sampling")
plt.title(f"Numeric vs. analytic calculated energy({dim}D and {N} particles)")
plt.xlabel(r"$\alpha$")
plt.ylabel(r"$\langle E\rangle$ ($\hbar\omega$)")
plt.legend(loc = 'best', prop = {'size':13}, frameon = True)
plt.savefig(os.path.join(save_folder, f"EnergyAlpha_BF_{dim}D_{N}N.png"))
plt.clf()

#time vs. alpha, BF, IM and analytic comparison 
plt.plot(alpha_IM,time_IM, label="Importance Samplin time")
plt.plot(alpha_BF,time_BF, label="Brute Force Metropolis time")
plt.plot(alphas,time_ana, label="Analytic time")
plt.title(r"$CPU_{time}$" +  " performance comparison" + f"({dim}D and {N} particles)")
plt.xlabel(r"$\alpha$")
plt.ylabel("Time[s]")
plt.legend(loc = 'best', prop = {'size':13}, frameon = True)
plt.savefig(os.path.join(save_folder, f"TimeAlpha_IM_BF_ANA_{dim}D_{N}N.png"))
plt.clf()
