# HW1 - TSP

Experimenty muzete spustit pomoci:

```bash
cargo run --release --bin hw1 -- --num-rep 7 --num-iters 3000 --pop-size 50
```

Pro vsechna mereni jsem pouzil pocet iteraci 3000, velikost populace 50, pocet opakovani 7. Inicializace populace je vzdy nahodna.

Local search pouziza perturbacni operatory move - presune vrchol na nahodne misto, swap - prohodi dve mesta, reverse - obrati nahodny cyklus.

Evolucni algoritmy vzdy pouzivaji move perturbacni operator a lisi se crossover operaci - cycle nebo move. Selekce je turnamentova a replacement strategie je truncation.

Zde jsou vysledky mereni. Je videt ze local search konverguje vyrazne pomaleji nez evolucni algoritmy, ale chovani u obou skupin je velice podobne.

![att48](out/tsp/att48.svg)
![berlin52](out/tsp/berlin52.svg)
![eil76](out/tsp/eil76.svg)

## Vizualizace

100 iteraci:

![opt](out/tsp/iter100_viz.svg)

3000 iteraci:

![opt](out/tsp/iter3000_viz.svg)

Optimum:

![opt](out/tsp/opt_viz.svg)

## bonusove body

+1 3x local search

+1 2x evolucni algoritmus
