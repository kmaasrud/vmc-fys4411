import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import os

from . import pathmaker as pth

plt.style.use('seaborn')


def readfiles(fileName, index):
    list = []
    infile = open(fileName,'r')
    infile.readline()
    for line in infile:
        numbers = line.split()
        list.append(float(numbers[index]))
    infile.close()
    return list

def plotter(x,y, label, xlabel, ylabel, title, PLOT_DIR, fig_id):
   
    #figure size and resolution
    fig = plt.figure()
    #colour, linewith, linestyle
    plt.plot(x,y, linewidth = 2.0, label = label)
    #boundaries
    #plt.xlim(min(x)*1.1, max(x)*1.1)
    #plt.ylim(min(y)*1.1, max(y)*1.1)
    #legend
    plt.legend(loc = 'best', prop = {'size':14}, frameon = False)
    plt.rc('font', size=10)
    plt.rc('axes', titlesize=12)
    plt.xlabel(xlabel)
    plt.ylabel(ylabel)
    plt.title(title)
    
     #make dir if does not exist
    pth.pathmaker(PLOT_DIR)

    pth.save_fig(PLOT_DIR,fig_id)
    
    plt.close() 
    return fig

def plot_dataframe(df, PLOT_DIR, fig_id):
    plt.figure()
    df.plot(x = 'Alpha')

    pth.pathmaker(PLOT_DIR)
    plt.xlabel('Alpha')
    plt.ylabel('Energy')
    plt.title('Energy vs alpha, numerical vs. analytical')

    pth.save_fig(PLOT_DIR,fig_id)

    plt.close()
    #make one dataframe for all the energies and plot as a function of alpha
    
    return 0


def plot_E_alpha_gaussian(x_n, y_n, err_n, x_a, y_a, err_a, dim, particles):
    df_a = pd.read_csv(DATA_DIR + f"dummy_{dim}D_{particles}_particles_ana.csv")
    df_n = pd.read_csv(DATA_DIR + f"dummy_{dim}D_{particles}_particles_num.csv")

    fig = plt.figure()
    plt.errorbar(x_n + 0.005, y_n, err_n, label = 'numerical',
                capsize= 4,
                fmt=".")

    plt.errorbar(x_a - 0.005, y_a, err_a, label = 'analytical', 
                 capsize= 4,
                 fmt="")

    plt.title(fr"$\langle E \rangle$ vs. $\alpha$ in {dim} dimension(s) for {particles} particles")
    plt.xlabel(r"$\alpha$")
    plt.ylabel(r"$\langle E \rangle$")
    plt.legend(loc = 'upper left', prop = {'size':10}, frameon = False)
    plt.rc('font', size = 10)
    plt.rc('axes', titlesize = 15)
    plt.savefig(FIG_DIR + f'dummy_{dim}D_{particles}_particles_error.png') 
    return fig


