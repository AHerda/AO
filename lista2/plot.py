from matplotlib import pyplot as plt
import seaborn as sns
import pandas as pd
from pathlib import Path
import os
import re

files = os.listdir("data")
dists = {
    'Uniform': 'Jednostajny',
    'Harmonic': 'Harmoniczny',
    'Geometric': 'Geometryczny',
    'Double_Harmonic': 'Podwójnie harmoniczny',
}
methods = [
    "FIFO",
    "FWF",
    "LRU",
    "LFU",
    "RAND",
    "RMA"
]


for file in files:
    data = pd.read_csv(f"data/{file}")
    n = int(re.findall("[0-9]+", file)[0])
    dist = ("", "")
    for dist_temp in dists.keys():
        if re.search(dist_temp, file):
            dist = (dists[dist_temp], dist_temp)
    for method in methods:
        sns.lineplot(data=data[['k', method]], x='k', y=method, label=method)
    plt.legend()
    plt.title(f"Rozkład {dist[0]}, n = {n}")
    plt.savefig(f'plots/n-{n}_dist-{dist[1]}.png', dpi=300)
    plt.close()


