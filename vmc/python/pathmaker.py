import os
import matplotlib.pyplot as plt


def pathmaker(PLOT_DIR, DATA_DIR):
    if not os.path.exists(PLOT_DIR):
        os.mkdir(PLOT_DIR)

    if not os.path.exists(DATA_DIR):
        os.mkdir(DATA_DIR)

def image_path(fig_id):
    return os.path.join(PLOT_DIR, fig_id)

def data_path(data_id):
    return os.path.join(DATA_DIR, data_id)

def save_fig(fig_id):
    plt.savefig(image_path(fig_id) + ".png", format = 'png')

DATA_DIR = "../data/"
PLOT_DIR = "../plots/"
fig_id = "dim_particles_.png"

pathmaker(PLOT_DIR, DATA_DIR)
image_path(fig_id)
save_fig(fig_id)


