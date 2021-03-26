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


for dim in range(1,2):
    print(f"dim {dim}")
    for particles in [1]:
        energy_mean = []
        for alpha in range(0, 90 + 1):
            energy_list = []
            
            for thread in range(len(os.listdir(dir))):
                threadID = thread + 2
                threadID_DIR = f"ThreadId({threadID})/"
                filename_a1 = f"analytical_{dim}D_{particles}_n_part.csv"  

                df = pd.read_csv(dir + threadID_DIR + filename_a1)    
                energy = df['Energy'].values.tolist()
                
                energy_list.append(energy[alpha])

            energy_mean.append(avglist(energy_list))
            a = df['Alpha'].values.tolist()
        
        #df_new = pd.DataFrame(list(zip(a, energy_mean)), columns = ['Alpha','Energy'])
        #pd.to_csv(path)
        print(df_new)
        print(len(energy_mean))
            
                
        #print(energy_mean)

    #df_energy = pd.DataFrame(energy_mean, columns=['Energy'])
            #print(df_energy)
        









