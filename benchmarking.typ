
= Information about apuli-lib's performance
in terms of speed and accuracy.
What matters is minimizing the amount of guesses required to finding the correct aswer
And the speed of the algorithm (mainly that it is implemented in a reasonable way)
And obviously that the program doesn't give wrong answers


== Benchmarks

#table(
  columns: (auto, auto, auto),
  inset: 10pt,
  align: horizon,
  [Version, revision], [Number of solved games], [Average guess count],
  "0.1.0, 1",
  [
    $5: 1366 slash 3085$ \
    $6: 0 slash 4221$
  ],
  " 4.091734197730956",
  "0.1.0, 2",
  [
    $5: 0 slash 3085$ \
    $6: 0 slash 4221$
  ],
  "12 (to do)",
  )

