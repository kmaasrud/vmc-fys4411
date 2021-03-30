import os
import matplotlib.pyplot as plt

def pathmaker(DIR):
    if not os.path.exists(DIR):
        os.makedirs(DIR)
        print(f'made directory {DIR}')
        
def join_path(DIR, id):
    return os.path.join(DIR, id)

def save_fig(DIR,id):
    plt.savefig(join_path(DIR, id) + ".png", format = 'png')

