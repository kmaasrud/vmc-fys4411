import plotter as plot
import os

#filepaths
PLOT_DIR = "../plots/"
DATA_DIR = "../data"
ANA_DIR = "/analytical"
NUM_DIR = "/numerical"

#dataframes


if not os.path.exists(PLOT_DIR):
    os.mkdir(PLOT_DIR)

if not os.path.exists(DATA_DIR):
    os.mkdirs(DATA_DIR)

def image_path(fig_id):
    return os.path.join(PLOT_DIR, fig_id)

def data_path(data_id):
    return os.path.join(DATA_DIR, data_id)

def save_fig(fig_id):
    plt.savefig(image_path(fig_id) + ".png", format = 'png')

