#import "@preview/cetz:0.3.2": canvas, draw
#import "@preview/cetz-plot:0.1.1": chart

#set heading(numbering: "1.")
#set text(lang: "pl")

#align(top + center)[
  #text(size: 24pt, [Algorytmy On-Line \ Lista 3])

  *Adrian Herda*

  #datetime.today().display()
]
#let data = csv("../data/data.csv")
#align(center)[
#table(
  columns: 6,
  align: center,
  inset: 10pt,
  table.header(..data.at(0)),
  ..data.slice(1).map(x => (x.at(0), x.slice(1, x.len()).map(y => str(calc.round(float(y), digits: 5)))).flatten()).flatten()
)

#canvas({
  draw.set-style(legend: (fill: white), columnchart: (bar-width: 0.7, cluster-gap: 0))
  chart.columnchart(
    size: (10, 10),
    mode: "clustered",
    y-label: "Avg ratio of used boxes to optimal number",
    x-label: "Distributions",
    y-min: 1,
    x-min: -0.5,
    x-max: 3.5,
    label-key: 0,
    value-key: (..range(1, 6)),
    labels: data.at(0).slice(1),
    data.slice(1).map(x => (x.at(0), x.slice(1, x.len()).map(y => float(y))).flatten()),
  )
})
]
