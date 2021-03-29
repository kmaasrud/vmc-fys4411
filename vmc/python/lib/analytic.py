import numpy as np
from numpy.random import seed, rand
import pandas as pd

def init_pos(seed_int, N):
    seed(seed_int)
    pos = rand(N)
    return pos

def squared_pos_sum(vec):
    return np.sum(np.square(vec))

def analytic_energy(dim, N, alpha,pos):
    E = N*alpha*dim*(.5 - 2*alpha**2)*squared_pos_sum(pos)
    return E


alphas = np.linspace(0,0.9, 50)
particles = [1, 10 , 100]
dim = 3


for dim in range(1, dim + 1):
    for N in particles:
        for a in alphas:
            pos = init_pos(1, N)
            E = analytic_energy(dim, N, a ,pos)/(dim*N)
            print(f"Alpha:{a} -- Dim: {dim}   Energy: {E}")

    






