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









