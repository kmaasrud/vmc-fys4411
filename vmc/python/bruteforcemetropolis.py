from lib import plotter as plot
import os
import numpy as np
import pandas as pd

stepsize = [0.1, 0.5, 0.25, 0.75, 0.125, 0.375, 0.625, 0.875]
dim = 3

xlabel = 'Alpha'
ylabel = 'Energy'

for dim in range(1, dim + 1):
    file_id = f"{dim}D"
    df_new = pd.DataFrame()
    for step in stepsize:
        DATA_DIR = f"../data/bruteforce_vs_importance/BruteForceMetropolis/step_size{step}/"
        PLOT_DIR = f"../plots/bruteforce_vs_importance/BruteForceMetropolis/step_size{step}/"        
        
        label = f'Stepsize: {step}'
        title = f'Brute Force Metropolis: Energy vs. alpha for stepsize = {step}, dim = {dim}'
        

        df_path = os.path.join(DATA_DIR, file_id + '.csv')
        df = pd.read_csv(df_path)
        df_energy = pd.DataFrame(df['Energy'])
        
        
        df_new = df_new[f'{step}'].append(df_energy)
        #df_new = df_new.append(df_energy)
   
    #print(y)    

    print(df_new)
    #plot.plotter(x,y,label, xlabel, ylabel, title, PLOT_DIR, file_id)
  


