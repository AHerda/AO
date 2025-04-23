#import "@preview/cetz:0.3.2": canvas, draw
#import "@preview/cetz-plot:0.1.1": chart, plot

#set heading(numbering: "1.")
#set text(lang: "pl")
#set par(
  first-line-indent: 2em,
  spacing: 1.2em,
  leading: 1em,
  justify: true,
)

#align(top + center)[
  #text(size: 24pt, [Algorytmy On-Line \ Lista 3])

  *Adrian Herda*

  #datetime.today().display()
]

= Treść zadania

Rozważmy problem #smallcaps[Bin Packing] z kubełkami wielkości $1$ i $100$-elementowymi ciągami elementów.

Ciągi elementów będą losowane z rozkładem jednostajnym na przedziale $[1,1]$ i powtarzane $k$ razy zgodnie z następującymi rozkładami na zbiorze ${1,dots.c,10}$ (losowanie robimy do uzyskania $100$ elementów):
  - jednostajny $Pr[X = i] = 1 / 10$,
  - harmoniczny $Pr[X = i] = 1 / (i dot H_10)$, gdzie $H_10$ jest $10$-tą liczbą harmoniczną,
  - dwuharmoniczny $Pr[X = i] = 1 / (i^2 dot hat(H)_10)$, gdzie $hat(H)_10 = sum_(i = 1)^10 1 / i^2$ jest $10$-tą liczbą dwuharmoniczną,
  - geometryczny $Pr[X = i] = 1 / 2^i$, dla $i < 10$, i $Pr[X = 10] = 1 / 2^9$.

Rozważmy następujące algorytmy online dla problemu #smallcaps[Bin Packing]:
  - #smallcaps[Next Fit],
  - #smallcaps[Random Fit],
  - #smallcaps[First Fit],
  - #smallcaps[Best Fit],
  - #smallcaps[Worst Fit],

Przeprowadź eksperymenty dla podanych algorytmów i rozkadów oraz oszacuj średnią wartość współczynnika konkurencyjności dla wszytskich przypadków. Do oszacowania użyj wartości optymalnych dla wylosowanych przykładów danych (jeśli nbie potrafisz policzyć optymalnej wartości użyj oszacowania w postaci zaokrąglenia w górę do wartości całkowitej sumy elementów ciągu).

= Wyniki eksperymentów

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
    draw.set-style(
      legend: (
        fill: white,
        default-position: "inner-north-east",
        scale: .5,
      ),
      columnchart: (bar-width: 0.7, cluster-gap: 0.05)
    )
    chart.columnchart(
      size: (12, 8),
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

  #canvas({
    draw.set-style(
      axes: (
        stroke: .5pt,
        tick: (stroke: .5pt),
      ),
      legend: (
        stroke: black,
        fill: white,
        orientation: ttb,
        default-position: "inner-east",
        scale: 50%,
      ),
    )

    plot.plot(
      size: (12, 8),
      axis-style: "scientific-auto",
      x-grid: true,
      y-grid: true,
      y-label: "Avg ration of used boxes to optimal number",
      x-label: "Distributions",
      x-ticks:  range(1, data.len()).zip(data.slice(1).map(x => x.at(0))),
      x-tick-step: none,
      {
        for i in  range(1, data.at(0).len()) {
          plot.add(
            range(1, data.len()).zip(data.slice(1).map(x => float(x.at(i)))),
            label: data.at(0).at(i),
          )
        }
      }
    )
  })
]


== Wnioski

Najlepsze wyniki ogólnie osiągają algorytmy Best Fit i First Fit, zwłaszcza dla rozkładów bardziej skoncentrowanych (np. Double Harmonic i Geometric).
Next Fit wypada najsłabiej, co jest zgodne z literaturą – to prosty algorytm, ale mało efektywny.
Algorytm Random radzi sobie gorzej niż First/Best Fit, ale lepiej niż Next Fit – jego wydajność jest niestabilna.
Dla wszystkich rozkładów, Worst Fit nie wypada najlepiej – często generuje nieefektywne wykorzystanie pojemników.
