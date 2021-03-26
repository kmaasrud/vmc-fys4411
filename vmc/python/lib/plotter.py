import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import os

import pathmaker as pth

plt.style.use('Solarize_Light2')


def readfiles(fileName, index):
    list = []
    infile = open(fileName,'r')
    infile.readline()
    for line in infile:
        numbers = line.split()
        list.append(float(numbers[index]))
    infile.close()
    return list

def plotter(x,y, label, xlabel, ylabel, title, FIG_DIR, filename):
    #make dir if does not exist
    pth.pathmaker(FIG_DIR)

    #figure size and resolution
    fig = plt.figure()
    #colour, linewith, linestyle
    plt.plot(x,y, linewidth = 2.0, label = label)
    #boundaries
    #plt.xlim(min(x)*1.1, max(x)*1.1)
    #plt.ylim(min(y)*1.1, max(y)*1.1)
    #legend
    plt.legend(loc = 'upper right', prop = {'size':14}, frameon = False)
    plt.rc('font', size=10)
    plt.rc('axes', titlesize=12)
    plt.xlabel(xlabel)
    plt.ylabel(ylabel)
    plt.title(title)
    
    pth.save_fig(FIG_DIR, filename)
    
    plt.close() 
    return fig


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
#usage

dim = 1
particles = 100

DATA_DIR = './dummydata/'
FIG_DIR = './fig/'
filename = f"dummy_{dim}D_{particles}_particles_ana"

#dataframes
df_a = pd.read_csv(DATA_DIR + filename + '.csv')

x = df_a["Alpha"]
y = df_a["Energy"]

plotter(x, y, 'label', 'xlabel', 'ylabel','title', FIG_DIR,filename)
#plotting
""" plotter1 = ploting(x_n, y_n, dim, particles, 'some lable', 'x', 'y','some_title')
"""
