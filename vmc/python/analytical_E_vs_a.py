import plotter as plot
import os
import pandas as pd

#filepaths
PLOT_DIR = "../plots/"
DATA_DIR = "../data"
ANA_DIR = "/analytical/dim_and_n/"
NUM_DIR = "/numerical/dim_and_n/"


dim = 1
particles = 1
filename_a1 = f"analytical_{dim}D_{particles}_n_part.csv"

dir = DATA_DIR + ANA_DIR
energy_mean = []

def avglist (list):
    return sum(list) / len(list)

for thread in range(len(os.listdir(dir))):
    threadID = thread + 2
    threadID_DIR = f"ThreadId({threadID})/"
    df = pd.read_csv(dir + threadID_DIR+ filename_a1)
    energy = df['Energy'].values.tolist()
    energy_mean.append(energy[0])
    print(energy_mean)

print(avglist(energy_mean))









