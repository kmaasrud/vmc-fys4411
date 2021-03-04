import matplotlib.pyplot as plt
import numpy as np

def readfiles(fileName, index):
    list = []
    infile = open(fileName,'r')
    infile.readline()
    for line in infile:
        numbers = line.split()
        list.append(float(numbers[index]))
    infile.close()
    return list


def plotting(x,y,lab, xlabel, ylabel, title, filename):
    #figure size and resolution
    fig = plt.figure(figsize = (12,6), dpi = 300)
    #colour, linewith, linestyle
    plt.plot(x,y, color = 'black', linewidth = 2.0, label = lab)
    #boundaries
    #plt.xlim(min(x)*1.1, max(x)*1.1)
    #plt.ylim(min(y)*1.1, max(y)*1.1)
    #legend
    plt.legend(loc = 'upper left', prop = {'size':14}, frameon = False)
    plt.rc('font', size=10)
    plt.rc('axes', titlesize=20)
    plt.xlabel(xlabel)
    plt.ylabel(ylabel)
    plt.title(title)
    plt.savefig('./fig/%s.png' %filename)
    return fig



#usage
some_list1_with_values = readfiles("./data/dummy_1D1N_analytical.txt",0)
some_list2_with_values = readfiles("./data/dummy_1D1N_analytical.txt",2)
some_list3_with_values = readfiles("./data/dummy_experiment.txt",0)
some_list4_with_values = readfiles("./data/dummy_experiment.txt",2)



#plotting
plotting(some_list1_with_values, some_list2_with_values, 'some lable', 'list1', 'list2','some_title', 'filename_without_end')
plotting(some_list3_with_values, some_list4_with_values, 'some lable', 'list1', 'list2','some_title', 'filename_without_end_2')


