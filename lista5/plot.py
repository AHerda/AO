import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

# 1) Load data
data_path = "results.csv"
df = pd.read_csv(data_path)

# Ensure correct dtypes
df["D"] = df["D"].astype(int)
df["p"] = df["p"].astype(float)
df["avg_cost"] = df["avg_cost"].astype(float)
df["avg_copies"] = df["avg_copies"].astype(float)

df = df.sort_values(by=["D", "p"])
print(df)

# Create output directory for plots
plots_dir = "plots/"

# Common Seaborn style
sns.set_theme(style="whitegrid", font_scale=1.2)

# High resolution
HIGH_DPI = 300

# 2) Bar Plot: Average Cost by p for each D
plt.figure(figsize=(10, 7), dpi=HIGH_DPI)
sns.lineplot(
    data=df,
    x="p",
    y="avg_cost",
    hue="D",
    palette="tab10",
    marker="o",
    markersize=6,
)
plt.title("Average Cost vs. Write Probability")
plt.xlabel("Write Probability (p)")
plt.ylabel("Average Cost")
plt.legend(title="Threshold D")
plt.tight_layout()
plt.savefig(plots_dir + "bar_avg_cost_vs_p.png", dpi=HIGH_DPI)
plt.close()

# 3) Bar Plot: Average Max Copies by p for each D
plt.figure(figsize=(10, 7), dpi=HIGH_DPI)
sns.lineplot(
    data=df,
    x="p",
    y="avg_copies",
    hue="D",
    palette="tab10",
    marker="o",
    markersize=6,
)
plt.title("Average Maximum Replicas vs. Write Probability")
plt.xlabel("Write Probability (p)")
plt.ylabel("Average Maximum Number of Copies")
plt.legend(title="Threshold D")
plt.tight_layout()
plt.savefig(plots_dir + "bar_avg_max_copies_vs_p.png", dpi=HIGH_DPI)
plt.close()

plt.figure(figsize=(10, 7), dpi=HIGH_DPI)
sns.lineplot(
    data=df,
    x="D",
    y="avg_cost",
    hue="p",
    palette="tab10",
    marker="o",
    markersize=6,
)
plt.title("Average Cost vs. Treshold D")
plt.xlabel("Treshold D")
plt.ylabel("Average Cost")
plt.legend(title="Threshold D")
plt.tight_layout()
plt.savefig(plots_dir + "bar_avg_cost_vs_D.png", dpi=HIGH_DPI)
plt.close()

# 3) Bar Plot: Average Max Copies by p for each D
plt.figure(figsize=(10, 7), dpi=HIGH_DPI)
sns.lineplot(
    data=df,
    x="D",
    y="avg_copies",
    hue="p",
    palette="tab10",
    marker="o",
    markersize=6,
)
plt.title("Average Maximum Replicas vs. Treshold D")
plt.xlabel("Treshold D")
plt.ylabel("Average Maximum Number of Copies")
plt.legend(title="Threshold D")
plt.tight_layout()
plt.savefig(plots_dir + "bar_avg_max_copies_vs_D.png", dpi=HIGH_DPI)
plt.close()
