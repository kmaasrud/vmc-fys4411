import os

#location for files and plots

PLOT_DIR = "../plots/"
DATA_DIR = "../data"


if not os.path.exists(PLOT_DIR):
    os.mkdir(PLOT_DIR)

if not os.path.exists(DATA_DIR):
    os.mkdirs(DATA_DIR)

def image_path(fig_id):
    return os.path.join(PLOT_DIR, fig_id)

def save_fig(fig_id):
    plt.savefig()

img_id = 1
image_path(f'{img_id}')