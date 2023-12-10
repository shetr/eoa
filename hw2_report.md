# HW2

Experimenty muzete spustit pomoci nasledujiciho příkazu. Od minula jsem se snažil odstranit warningy, už by to mělo být bez nich. Výstupní grafy se uloží do složky out/g_funcs

```bash
cargo run --release --bin hw2 -- --num-rep 7 --num-iters 3000 --pop-size 64
```

## Parametry EA

Základní parametry jsou nastavitelné prímo při volání příkazu: velikost populace (--pop-size 64), počet iterací (--num-iters 3000), počet opakování měření pro zprůměrování (--num-rep 7). Tyto hodnoty jsou zároveň použity pro experimenty zmíněné v tomto dokumentu.

Většina parametrů evolučního algoritmu zůstává stejná u různých metod. Jediné co se liší je způsob reprezentace fitness funkce a takzvaný FitnessTransformer trait, který má konkrétní implementace pro Stochastic Ranking a NSGA-II. FitnessTransformer vlastně umožňuje původní fitness převést na nějakou vlastní strukturu, která se chová jako fitness a je možné jednotlivé její instance porovnávat a využít například v selekci. Například pro NSGA-II se fitness ve formě vektoru floatů převede na strukturu obsahující číslo fronty a crowding distance. Díky tomuto provedení jsou všechna volání evolučního algoritmu provedena pomocí funkce general_evolutionary_search a konkrétní metoda se vybere nastavením parametrů.

Pro všechny běhy evolučního algoritmu jsou parametry nastaveny následujícím způsobem: 

 - **Iniciální populace** je při každém běhu vygenerována nová, ale ze stejného nastavení. Náhodně vygenerované vektory jsou omezeny na obdelíkovou oblast, ve které je optimalizace konkrétní funkce zadána (podle dokumentu). Vektory se generují podle normálního rozdělení se středem zhruba ve středu této oblasti a rozptylem odpovídajícím její velikosti.
 - Operátor **selekce** je nastaven na **rank selection**, kde velikost výběru je polovina populace.
 - Operátor **crossover** je nastaven na **arithmetric crossover**. Jsou vygenerováni vždy 2 potomci ze 2 rodičů. Každý potomek se nachází náhodně na úsečce mezi jeho rodiči.
 - Operátor **perturbace** je nastaven na perturbaci vektoru reálných čísel přičtením náhoného vektoru vygenerovaného podle **normálního rozdělení**, kde směrodatná odchylka je nastavena na 1/10 velikosti prostoru, ve kterém optimalizujeme.
 - Operátor **replacement strategie** je nastaven na **truncation replacement**, při které se zachovává konstatní velikost populace.
 - **Ukončovací podmínka** je nastavená čistě na dosažení požadovaného počtu iterací.

## Výsledky

Na ose x jsou opět vyznačeny iterace jako v předchozím úkolu. Zde to avšak, dle mého názoru, už dává smysl, protože porovnávám 2 evoluční algoritmy s téměř totožným nastavením parametrů, a ne local search a evoluční algoritmus jak tomu bylo v minulém úkolu.

V měřeních jsou porovnány 3 metody: Stochastic Ranking, NSGA-II s 2 hodnotami (fitness + součet omezení), NSGA-II s více vícero hodnotami (fitness + každé omezení zvlášť). Porovnání bylo vyhodnoceno jak pro základní požadované funkce g06, g08, g11 a g24, tak i pro složitější bonusové funkce g04, g05, g09 a g21.

Řešení reportovaná na grafech

## Funkce g06, g08, g11 a g24

![att48](out/g_funcs/g06_fitness.svg)
![att48](out/g_funcs/g08_fitness.svg)
![att48](out/g_funcs/g11_fitness.svg)
![att48](out/g_funcs/g24_fitness.svg)

## Funkce g04, g05, g09 a g21


![att48](out/g_funcs/h_g04_fitness.svg)
![att48](out/g_funcs/h_g05_fitness.svg)
![att48](out/g_funcs/h_g09_fitness.svg)
![att48](out/g_funcs/h_g21_fitness.svg)

## Bonusove body

+1 For implementing the multi-objective (not only bi-objective) approach. 

+1 	For comparing the algorithms on more complex problems. 