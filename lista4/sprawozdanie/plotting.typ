#import "@preview/cetz:0.3.2": canvas, draw
#import "@preview/cetz-plot:0.1.1": plot

#import "data.typ"

#let plots = data.files.map( file => {
  let results = csv(file)
  let colors = (
    red,
    green,
    blue,
    yellow,
    purple,
    black,
    orange
  )

  return (file, canvas({
    import draw: *

    // Set-up a thin axis style
    set-style(
      axes: (
        stroke: .5pt,
        tick: (stroke: .5pt),
      ),
      legend: (
        stroke: black,
        orientation: ttb,//ltr,
        default-position: "inner-east",
        item: (spacing: .3),
        scale: 50%
      ),
    )

    plot.plot(
      size: (12, 8),
      axis-style: "scientific-auto",
      x-grid: true,
      y-grid: true,
      x-label: "D",
      y-label: "Average cost",
      y-break: true,
      {
        for (i, color) in range(1, results.at(0).len()).zip(colors) {
          plot.add(
            results.slice(1).map(
              x => (
                float(x.at(0)), float(x.at(i))
              )
            ),
            label: results.at(0).at(i),
            style: (stroke: color)
          )
        }
      }
    )
  }))
})

#let plot(n) = {
  figure(
    plots.at(n).at(1),
    caption: plots.at(n).at(0),
  )
}

#let plot2(dist, variant) = {
  let re = "dist-" + dist + "_(alg|struct)-" + variant
  let pos = plots.map(x => x.at(0)).position(x => x.contains(regex(re)))
  figure(
    plots.at(pos).at(1),
    caption: "Average cost of page migration with " + dist.replace("_", " ") + " distribution and " + variant,
  )
}
