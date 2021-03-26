from lib import plotter as plot
import os
import pandas as pd

#filepaths

DATA_DIR = "../data/dim_and_n/"
ANA_DIR = "analytical/"
NUM_DIR = "numerical/"



def avglist (list):
    return sum(list) / len(list)


for dim in range(1,4):
    print(f"dim {dim}")
    for particles in [1,10,100]:
        energy_mean_n, energy_mean_a = [], []
        for alpha in range(0, 90):
            energy_list_n, energy_list_a = [], []
            for thread in range(len(os.listdir(DATA_DIR + ANA_DIR))):
                threadID = thread + 2
                threadID_DIR = f"ThreadId({threadID})/"
                
                file_id = f"_{dim}D_{particles}_n_part"  
               
                path_n = os.path.join(DATA_DIR, NUM_DIR, threadID_DIR, 'numerical' + file_id + '.csv')
                path_a = os.path.join(DATA_DIR, ANA_DIR, threadID_DIR,'analytical' + file_id + '.csv')
                
                df_n = pd.read_csv(path_n)  
                df_a = pd.read_csv(path_a) 

                energy_n = df_n['Energy'].values.tolist()    
                energy_a = df_a['Energy'].values.tolist()

                energy_list_n.append(energy_n[alpha])
                energy_list_a.append(energy_n[alpha])

            
            energy_mean_n.append(avglist(energy_list_n))
            energy_mean_a.append(avglist(energy_list_a))

            a = df_n['Alpha'].values.tolist()
        
        df_new = pd.DataFrame(list(zip(a, energy_mean_n, energy_mean_a)), columns = ['Alpha','Energy_numerical','Energy_analytical'])
        df_new['Energy_analytical'] += 0.001
        PLOT_DIR = "../plots/dim_and_n/"
        plot.plot_dataframe(df_new, PLOT_DIR, file_id)
        #path = DATA_DIR + f'mean_a_n/{dim}D_{particles}_n_part.csv'
        #df_new.to_csv(path)
       
            
        #print(energy_mean)

    #df_energy = pd.DataFrame(energy_mean, columns=['Energy'])
            #print(df_energy)
        









