import numpy as np
from random import random, seed


def AnalyticLocalEnergy(alpha, dim, N, StepSize):
    #seed for random number generator
    seed()

    r = np.zeros((N,dim), np.double)
    omega = 1

    
    for i in range(N):
        for j in range(dim):
            r[i,j] = (random()) 

    if dim == 1:
        r1 = r[0,0]
        for i in range(0, N):
            r1 += r[i,0]**2
        E = N*alpha*dim + (0.5*omega*omega - 2.0*alpha*alpha)*(r1)
    if dim == 2:
        r1 = r[0,0]; r2 = r[0,1]
        for i in range(0, N):
            r1 += r[i,0]**2
            r2 += r[i,1]**2
        E = N*alpha*dim + (0.5*omega*omega - 2*alpha*alpha)*(r1 + r2)
    if dim == 3:
        r1 = r[0,0]; r2 = r[0,1]; r3 = r[0,2]
        for i in range(0, N):
            r1 += r[i,0]**2
            r2 += r[i,1]**2
            r3 += r[i,2]**2
        E = N*alpha*dim + (0.5*omega*omega - 2*alpha*alpha)*(r1 + r2 + r3)
        

    return E
   