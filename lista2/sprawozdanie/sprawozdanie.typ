#import "plotting.typ": plot2

#set heading(numbering: "1.")
#set text(lang: "pl")
#set par(
  first-line-indent: 1em,
  spacing: 1.2em,
  justify: true,
)

//#plot2(80, "Double_Harmonic")
//#plot2(100, "Double_Harmonic")
#align(top + center)[
  #text(size: 24pt, [Algorytmy On-Line \ Lista 2])

  *Adrian Herda*

  #datetime.today().display()
]

= Treść zadania

Dla problemu stronicowania rozważamy cache o pojemności k dla zbioru n stron żą-
danych zgodnie z podanym rozkładem. Zbadaj średni koszt żądania strony dla podanych
rozkładów, algorytmów, k i n.

Rozważ następujące rozkłady zmiennej losowej $X$ dla $n$ elementów (ze zbioru ${1, dots , n}$):
- jednostajny $P r[X = i] = 1 / n$
- harmoniczny $P r[X = i] = 1 / (i dot H_n)$, gdzie $H_n$ jest liczbą harmoniczną,
- dwuharmoniczny $P r[X = i] = 1 / (i^2 dot hat(H)_n)$, gdzie $hat(H)_n = sum_(i=1) 1 / i^2$ jest n-tą liczbą dwuharmoniczną,
- geometryczny $P r[X = i] = 1 / 2^i$, dla $i < n$, i $P r[X = n] = 1 / 2^(n−1)$.


Zastosuj następujące metody obsługi cache’a:
- FIRST IN FIRST OUT (FIFO),
- FLUSH WHEN FULL (FWF),
- LEAST RECENTLY USED (LRU),
- LEAST FREQENTLY USED (LFU) – licznik użycia strony przechowujemy nawet jeśli strony nie ma w cache’u,
- RANDOM (RAND) – losujemy stronę do wyrzucenia z rozkładem jednostajnym w całym cache’u,
- RANDOMIZED MARKUP ALGORITHM (RMA) – stosujemy algorytm oznaczający i wyrzucamy stronę nieoznaczoną, losowaną z rozkładem jednostajnym.

Przeprowadź eksperymenty dla n ze zbioru ${20, 30, 40, 50, 60, 70, 80, 90, 100}$ i $k$ ze zbioru
${ n / 10 , dots , n / 5 }$ (np. dla $n = 40$ mamy $k in {4, 5, 6, 7, 8}$). Przygotuj krótkie sprawozdanie ilustrujące uzyskane wyniki.

= Wyniki

Eksperyment polegał na losowaniu $1000000$ stron a następnie wyciągania ich z cache'a który był obsługiwany przez różne algorytmy opisywane w rozdziale 1. Testy były wykonywane na różnego rodzaju rozkładach prawdopodobieństwa stosowanych do losowania wyciąganych stron, na różnych wielkościach cache'a oraz na różnych zbiorach stron.

== Rozkład jednostajny

#plot2(100, "Uniform") <Uni100>

@Uni100 przedstawia wykres średnich kosztów dla $n = 100$ oraz $k in {10, dots, 20}$ dla różnych metod obsługi cache'a. Algorytm FWF (ang. Flush When Full) widocznie odstaje od innych algorytmów, które wydają się być dokładnie tak samo wydajne w stronach losowanych w rozkładzie jednostajnym.

== Rozkład harmoniczny

#plot2(100, "Harmonic") <Har100>

Na @Har100 widać porównanie opisywanych metod obsługi chache'a względem ich średniego kosztu przy rozkładzie harmonicznym. Algorytm FWF podobnie jak w poprzednim rozkładzie wypada najgorzej. Algorytmy typu FIFO (ang. First In, First Out) oraz RAND (ang. Randomized) mają bardzo zbliżony średni koszt. Najlepsze okazują się algorytmy -- w kolejności -- LRU (ang. Least Recently Used), RMA (ang. Randomized Markup Algorithm) oraz LFU (ang. Least Frequently Used), gdzie ten ostani ma średni koszt prawie dwa razy mniejsszy od najgorszego.

== Rozkład dwuharmoniczny

#plot2(100, "Double_Harmonic") <DHar100>

@DHar100 ma dokładnie tą samą kolejność algorytmów co @Har100. Jedyna różnica pomiędzy tymi wykresami jest taka że wszystkie algorytmy mają dużo mniejszy średni koszt i maleje on w sposób bardziej geometryczny niż wykres z rozkładem harmonicznym.


== Rozkład geometryczny
