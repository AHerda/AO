#set heading(numbering: "1.")
#show heading: set block(below: 1em, above: 2em)
#show figure: set block(below: 2em, above: 1em)
#set text(lang: "pl")
#set par(
  first-line-indent: 2em,
  spacing: 0.8em,
  justify: true
)

#import "data.typ"
#import "plotting.typ"

#align(top + center)[
  #text(size: 24pt, [Algorytmy On-Line \ Lista 4])

  *Adrian Herda*

  #datetime.today().display()
]

= Treść zadania

Rozważmy probelm #smallcaps[Page Migration] na dwóch grafaach 64 wierzchołkowych, torusie trzywymiarowym i hiperkostce z wagami krawędzi 1. Niech $D in {16, 32, 64, 128, 256}$.

Ciągi żądań długości $65536$ generujemy zgodnie z następującymi rozkładami na zbiorze ${1, dots.c, 64}$:
- jednostajny $Pr[x = i] = 1 / 64$,
- harmoniczny $Pr[X = i] = 1 / (i dot H_64)$, gdzie $H_64$ jest $64$-tą liczbą harmoniczną,
- dwuharmoniczny $Pr[X = i] = 1 / (i^2 dot hat(H)_64)$, gdzie $hat(H)_64 = sum_(i=1)^64 1 / i^2$ jest $64$-tą liczbą dwuharmoniczną.

Roważmy dwa następujące algorytmy online dla problemu:
- deterministyczny #smallcaps[Move-To-Min],
- losowy #smallcaps[Coin-Flip].

Przeprowadź eksperymenty dla podanych algorytmów, grafów i rozkładów. Porównaj koszty obu podanych algorytmów dla różnych wartości D.

= Wyniki eksperymentów

#for dist in data.dists [
  #plotting.plot2(dist)
]

Jak widać na powyższych wykresach algorytm #smallcaps[MoveTo Min] jest lepszy od algorytmu #smallcaps[CoinFlip] dla rozkładu harmonicznego i dwuharmoniczego. W przypadku rozkładu jednostajnego algorytm #smallcaps[CoinFlip] jest lepszy od algorytmu #smallcaps[MoveToMin]. Można zauważyć, że dla grafu o kształcie hiperkostki średnie koszty mają asymptotykę zbliżoną do $O(D)$, a dla rozkładu kształtu trójwymiarowego torusa wydaje się to być zbliżone do $O(log D)$. Wyjątkiem jest rozkład harmoniczny gdzie algorytm #smallcaps[MoveToMin] na grafie w kształcie torusa maleje wraz ze wzrostem $D$.

Algorytm #smallcaps[MoveToMin] na grafie w kształcie torusa trójwymiarowego daje najlepsze wyniki szczególnie dla coraz większych wartości $D$
