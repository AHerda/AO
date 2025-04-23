import matplotlib.pyplot as plt
import pandas as pd


df = pd.read_csv("data/data.csv")

# Rysowanie wykresu
plt.figure(figsize=(12, 6))
for alg in df.columns[1:]:
    plt.plot(df["Distributions"], df[alg], marker='o', label=alg)

plt.title("Porównanie algorytmów pakowania online (Bin Packing)")
plt.xlabel("Rozkład danych")
plt.ylabel("Średni współczynnik konkurencyjności")
plt.legend()
plt.grid(True)
plt.tight_layout()

# Zapisanie do pliku
plt.savefig("plots/plot.png")

plt.show()

