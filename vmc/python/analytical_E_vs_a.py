import plotter as plot
import os
import pandas as pd

#filepaths
PLOT_DIR = "../plots/"
DATA_DIR = "../data"
ANA_DIR = "/analytical/dim_and_n/"
NUM_DIR = "/numerical/dim_and_n/"
dir = DATA_DIR + ANA_DIR

def avglist (list):
    return sum(list) / len(list)


for dim  in range(1,4):
    for particles in [1]:
        for alpha in range(0, 90 + 1):
            alpha *= 0.01
            for thread in range(len(os.listdir(dir))):
                threadID = thread + 2

                threadID_DIR = f"ThreadId({threadID})/"
                filename_a1 = f"analytical_{dim}D_{particles}_n_part.csv"  

                df = pd.read_csv(dir + threadID_DIR + filename_a1)    
                energy = df['Energy'].values.tolist()
            
            
                energy_list = []
                
                energy_list.append(energy[alpha])

                energy_mean = avglist(energy_list)
                print(avglist(energy_list))

    #df_energy = pd.DataFrame(energy_mean, columns=['Energy'])
            #print(df_energy)
        









