# Semestrální úloha - Generalized TSP

Experimenty můžete spustit pomocí následujícího příkazu. Příkaz vyplotuje všechny obrázky nacházející se v tomto dokumentu - tedy buď graf porovnání různých metod, nebo vizualizace řešení. Pokud by se nějaký svg obrázek nebo gif nezobrazoval, nachází se všechny ve složce **out/gtsp/**.

```bash
cargo run --release --bin sem
```

## Implementace

Podle zadání jsem vytvořil a porovnával 3 různé algoritmy pro řešení daného problému: lokální prohledávání, evoluční algoritmus a specializovaný algoritmus - v mém případě konkrétně heuristický algoritmus kombinovaný buď s lokálním prohledáváním nebo evolučním algoritmem.

### Reprezentace

Samotný problém reprezentuji podobně jako je tomu v souborech poskytunutých datasetů, tedy jako symetrickou matici vzdáleností (takže řeším jen úlohy s neorientovaným grafem) a k tomu list listů který rozděluje města do jednotlivých skupin (regionů).

Řešení reprezentuji jako permutaci jednotlivých skupin a k tomu pro každou skupinu je přiřazeno jedno konkrékrétní město, které se v dané skupině nachází. Díky této reprezentaci můžu použít část implementace z prvního úkolu.

### Lokální prohledávání

Standardní lokální prohledávání, v kódu je voláno pomocí obecné funkce *evolutionary_search* implementujicí primálně evoluční algoritmus (radši píšu, abyste se nelekl když to uvidíte v kódu). To že to skutečně vykonává lokální prohledávání je dosaženo tím, že populace má velikost 1, operátory selekce a křížení jsou identita a replacement strategie je truncation. Takže jediná věci co se mění je perturbační/mutační operátor. Dělám to takto z důvodu větší jednoduchosti kódu, protože je sdílené rozhraní pro všechny varianty algoritmů.

Používám 4 základní perturbační operátory: **move**, **swap**, **reverse** a **city**. Operátory **move**, **swap** a **reverse** mění pouze permutaci skupin a konkrétní města v nich ponechávají. Tyto operátory jsou přepoužity z prvního úkolu. Operátor **city** naopak mění pouze konkrétní města ve skupinách a permutaci skupin ponechává stejnou.
 - **move** perturbační operátor náhodně přesune jednu skupinu v permutaci
 - **swap** perturbační operátor prohodí dvě náhodné skupiny v permutaci
 - **reverse** perturbační operátor obrátí náhodnou podsekvenci permutace skupin
 - **city** perturbační operátor v rámci skupiny náhodně změní vybrané město (s uniformní pravděpodobností) a to pro každou skupinu s pravděpodobností 1/(počet skupin)

Nikdy nepoužívám pouze jeden konkrétní operátor. Implementoval jsem ještě jeden operátor, který může obsahovat kombinaci výše zmíněných operátorů a ke kažnému má přiřazenou požadovanou pravděpodobnost. Tento operátor při vyhodnocení aplikuje na vstupní data každý jeho vnitřní perturbační operátor s danou pravděpodobností. Pravděpodobnosti jsou na sobě nezávislé, může se tedy aplikovat více operátorů najednou. Ve většině případů používa kombinaci **city** operátoru a jednoho z **move**, **swap** a **reverse** operátorů.

### Evoluční algoritmus

Pro všechny běhy evolučního algoritmu jsou následující parametry stejné:
 - Velikost populace je 64, pro základní použití bez kombinace s heuristickým přístupem je **iniciální populace** vytvořena z náhodně vygenerovaných permutací s náhodným výběrem měst v rámci skupin
 - Operátor **selekce** je nastaven na **rank selection**, kde velikost výběru je polovina populace.
 - Operátor **replacement strategie** je nastaven na **truncation replacement**, při které se zachovává konstantní velikost populace.

Jediné co se vždy liší jsou operátory **perturbace** a **crossover**. Operátor **perturbace** můžeme vybírat ze stejných variant jako je popsáno v sekci o lokálním prohledávání.

Operátor **crossover** je podobně jako operátor **perturbace** implementován v několika základních variantách, které pak můžeme kombinovat dohromady do jednoho crossover operátoru, který každou variantu vyhodnotí s nastavitelnou pravděpodobnosti. Zde se avšak vyhodnotí nejvíše jeden, proto součet pravděpodobností musí být menší nebo roven 1. Také podobně jako u **perturbace** i zde jsou dva typy operátorů, jeden co mění pouze permutaci skupin, druhý co mění pouze výběr města v rámci skupiny. Pro změnu permutace skupin jsou tu operátory **cycle** a **order** (opět převzato z prvního úkolu), pro změnu města v rámci skupiny je zde operátor **uniform_city**.
 - **cycle** crossover operátor identifikuje totožné cykly skupin na stejných indexech a prohodí je.
 - **order** crossover operátor vezme náhodnou podsekvenci prvního rodiče a k ní doplní zbývající index z druhého rodiče v pořadí postupně jak jdou za sebou u druhého rodiče. Tuto operaci udělá symetricky pro oba rodiče.
 - **uniform_city** crossover operátor, je podobný jako uniformní binární křížení - pro 2 řešení pro totožné skupiny měst s 50% pravděpodobností prohodí jejich vybraná města.

Pro experimenty je většnou použita kombinace **uniform_city** crossoveru a jednoho z **cycle** a **order** crossoverů.

### Heuristický algoritmus kombinovaný s lokálním nebo evolučním algoritmem

Heuristický algoritmus vygeneruje řešení s rozumnou fitness v polynomiálním čase. Funguje velice jednodušše, nejprve vybere náhodný vrchol a poté postupně přidává do permutace vrchol, který má nejkratší vzdálenost k poslednímu přidanému vrcholu a je ze skupiny, která ještě nebyla navštívena.

V kombinaci s lokálním prohledáváním je výsledek jednoduše použit jako iniciální řešení.

V kombinaci s evolučním algoritmem se vygeneruje do iniciální populace několik těchto heuristických řešení s různými počátečními vrcholy. Avšak není takto vygenerována celá populace, jen její čtvrtina. Zbytek jsou opět náhodné permutace jako v klasické inicializaci evolučního algoritmu. Dle mých pozorování to vede na výrazně lepší výsledky. Pokud vygeneruji populaci celou z heuristických řešení, pak se velice rychle zasekne v lokálním optimu.

## Porovnání metod

TODO: popsat co je obecne na grafech - optimum je 0, jaka data pouzivam, jaky maji data charakter, jak porovnávám local search a evolucni algoritmus, ruzne pocty iteraci u ruznych problemu, atd..

### Porovnání při základním nastavení parametrů

### Porovnání různých variant lokálního prohledávání

### Porovnání různých variant evolučního algoritmu

### Porovnání nejlepších variant různých algoritmů

## Vizualizace

test

![viz_best_sol_g3](out/gtsp/viz_best_sol_g3.svg) 

test2

![viz_local_g2.gif](out/gtsp/viz_local_g2.gif) 