#let dists = (
  "Uniform",
  "Harmonic",
  "Double_Harmonic",
)

#let files = dists.map(
  dist => "../data/dist-" + dist + ".csv"
)
