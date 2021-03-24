import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import os


#global variables
DATA_DIR = './dummydata/'
FIG_DIR = './fig/'

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

def ploting(x,y, label, xlabel, ylabel, title, PLOT_DIR, FILENAME_PLOT):
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

    if not os.path.exists(PLOT_DIR):
        os.makedirs(r"" + PLOT_DIR)
    plt.show()
    #plt.savefig(r""  +  PLOT_DIR + FILENAME_PLOT) 
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


dir_data_dummy_a = DATA_DIR + f"dummy_{dim}D_{particles}_particles_ana.csv"

dim = 1
particles = 1
dir_data_dummy_n = DATA_DIR + f"experiment_{dim}D_{particles}_particles_num.csv"

#dataframes
df_a = pd.read_csv(dir_data_dummy_a)
df_n = pd.read_csv(dir_data_dummy_n)

x_n = df_n["Alpha"]
y_n = df_n["Energy"]
err_n = np.sqrt(df_n['Variance'])

x_a = df_a["Alpha"]
y_a = df_a["Energy"]
err_a = np.sqrt(df_a['Variance'])

#plotting
""" plotter1 = ploting(x_n, y_n, dim, particles, 'some lable', 'x', 'y','some_title')
dim = 1
particles = 100
plotter2 = plot_E_alpha_gaussian(x_n, y_n, err_n, x_a, y_a, err_a, dim, particles)
 """