# HW1 - TSP

Experimenty muzete spustit pomoci:

```bash
cargo run --bin hw1
```

Pro vsechna mereni jsem pouzil pocet iteraci 300, velikost populace 50, pocet opakovani 7. Inicializace populace je vzdy nahodna.

Local search pouziza perturbacni operatory move - presune vrchol na nahodne misto, swap - prohodi dve mesta, reverse - obrati nahodny cyklus.

Evolucni algoritmy vzdy pouzivaji move perturbacni operator a lisi se crossover operaci - cycle nebo move. Selekce je turnamentova a replacement strategie je truncation.

Bohuzel se mi nepovedlo udelat labely barev do grafu (kvuli zapaseni s borrow checkerem v rustu), zde je prirazeni.

```
loc_move  = RED
loc_swap  = GREEN
loc_rev   = BLUE
evo_cycle = YELLOW
evo_order = MAGENTA

```

Zde jsou vysledky mereni. Je videt ze local search konverguje vyrazne pomaleji nez evolucni algoritmy, ale chovani u obou skupin je velice podobne.

![att48](out/tsp/att48.svg)
![berlin52](out/tsp/berlin52.svg)
![eil76](out/tsp/eil76.svg)

## bonusove body

+1 3x local search

+1 2x evolucni algoritmus
