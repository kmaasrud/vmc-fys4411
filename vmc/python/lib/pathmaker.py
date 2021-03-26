import os
import matplotlib.pyplot as plt

def pathmaker(DIR):
    if not os.path.exists(DIR):
        os.mkdir(DIR)
        
def image_path(PLOT_DIR, fig_id):
    return os.path.join(PLOT_DIR, fig_id)

def data_path(data_id):
    return os.path.join(DATA_DIR, data_id)

def save_fig(PLOT_DIR, fig_id):
    plt.savefig(image_path(PLOT_DIR,fig_id) + ".png", format = 'png')

