from matplotlib import pyplot as plt
import seaborn as sns
import pandas as pd
from pathlib import Path

plots_folder = Path(__file__).parent / 'plots'
plots_folder.mkdir(exist_ok=True)

data = pd.read_csv('l1.csv', delimiter=';')
data['sredni_koszt'] = data['total_cost'] / data['n']
print(data.head())

hue_order = data['rodzaj_listy'].unique()
palette = sns.color_palette(n_colors=len(hue_order))

dists = {
    'Uniform': 'Jednostajny',
    'Harmonic': 'Harmoniczny',
    'Geometric': 'Geometryczny',
    'DoublyHarmonic': 'Podwójnie harmoniczny',
}

for distribution in data['distribution'].unique():
    subset = data[data['distribution'] == distribution]
    sns.lineplot(data=subset, x='n', y='sredni_koszt', hue='rodzaj_listy', hue_order=hue_order, palette=palette)
    plt.title(f'Rozkład: {dists[distribution]}')
    plt.savefig(plots_folder / f'distribution_{distribution}.png', dpi=300)
    plt.close()
