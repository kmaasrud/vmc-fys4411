# Common imports
import os
from numpy import log2, zeros, mean, var, sum, loadtxt, arange, array, cumsum, dot, transpose, diagonal, sqrt
from numpy.linalg import inv


def data_path(dat_id):
    return os.path.join(DATA_ID, dat_id)


def block(x):
    # preliminaries
    n = len(x)
    d = int(log2(n))
    s, gamma = zeros(d), zeros(d)
    mu = mean(x)

    # estimate the auto-covariance and variances 
    # for each blocking transformation
    for i in range(d):
        n = len(x)
        # estimate autocovariance of x
        gamma[i] = (1/n) * sum((x[0:n-1] - mu)*(x[1:n] - mu))
        # estimate variance of x
        s[i] = var(x)
        # perform blocking transformation

        # Thanks @Schoyen and @gregwinther for this:
        # We might get a situaion where the array is not easily split in two equal sizes
        x_1 = x[0::2] # Extracting all numbers at odd positions
        x_2 = x[1::2] # Numbers at even positions
        # If length is not equal, remove highest number
        if (len(x_1) > len(x_2)):
            x_1 = x_1[:-1]
        elif (len(x_2) > len(x_1)):
            x_2 = x_2[:-1]

        # Blocking transformation
        x = 0.5*(x_1 + x_2)
   
    # Test observator from theorem (chi^2-distributed)
    factor_1 = (gamma/s)**2
    factor_2 = 2**arange(1, d+1)
    # Do the same length check again
    if (len(factor_1) > len(factor_2)):
        factor_1 = factor_1[:-1]
    elif (len(factor_2) > len(factor_1)):
        factor_2 = factor_2[:-1]

    M = (cumsum((factor_1 * factor_2[::-1])[::-1]))[::-1]

    # we need a list of magic numbers
    q = array([6.634897,9.210340, 11.344867, 13.276704, 15.086272, 16.811894, 18.475307, 20.090235, 21.665994, 23.209251, 24.724970, 26.216967, 27.688250, 29.141238, 30.577914, 31.999927, 33.408664, 34.805306, 36.190869, 37.566235, 38.932173, 40.289360, 41.638398, 42.979820, 44.314105, 45.641683, 46.962942, 48.278236, 49.587884, 50.892181])

    # use magic to determine when we should have stopped blocking
    for k in arange(0,d):
        if(M[k] < q[k]):
            break
    if (k >= d-1):
        print("Warning: Use more data")
    return mu, s[k]/2**(d-k)


if __name__ == "__main__":
    from pandas import DataFrame
    import pandas as pd
    # Where to save the figures and data files
    dim = 1
    particles = 1

    DATA_ID = "./dummydata/"
    filename = f"experiment_{dim}D_{particles}_particles_num.csv"

    infile = open(data_path(filename),'r')

    df = pd.read_csv(infile)

    x = array(df['Energy'].values.tolist())

    (mean, var) = block(x) 
    std = sqrt(var)

    data ={'Mean':[mean], 'STDev':[std]}
    frame = pd.DataFrame(data,index=['Values'])
    print(frame) 
    infile.close()
