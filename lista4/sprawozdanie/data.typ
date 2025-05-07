#let dists = (
  "Uniform",
  "Harmonic",
  "Double_Harmonic",
)

// #let files = dists.map(
//   dist => "../data/dist-" + dist + ".csv"
// )

#let files = (
  "../data/dist-Double_Harmonic_alg-CoinFlip.csv",
  "../data/dist-Double_Harmonic_alg-MoveToMin.csv",
  "../data/dist-Double_Harmonic_struct-Hypercube.csv",
  "../data/dist-Double_Harmonic_struct-Torus.csv",
  "../data/dist-Harmonic_alg-CoinFlip.csv",
  "../data/dist-Harmonic_alg-MoveToMin.csv",
  "../data/dist-Harmonic_struct-Hypercube.csv",
  "../data/dist-Harmonic_struct-Torus.csv",
  "../data/dist-Uniform_alg-CoinFlip.csv",
  "../data/dist-Uniform_alg-MoveToMin.csv",
  "../data/dist-Uniform_struct-Hypercube.csv",
  "../data/dist-Uniform_struct-Torus.csv",
)

#let variants = (
  "CoinFlip",
  "MoveToMin",
  "Hypercube",
  "Torus",
)
