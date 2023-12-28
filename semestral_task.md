# Semestrální úloha - Generalized TSP

Experimenty můžete spustit pomocí následujícího příkazu. Příkaz vyplotuje všechny obrázky nacházející se v tomto dokumentu - tedy buď graf porovnání různých metod, nebo vizualizace řešení. Pokud by se nějaký svg obrázek nebo gif nezobrazoval, nachází se všechny ve složce **out/gtsp/**.

```bash
cargo run --release --bin sem -- --stats --viz --num-rep 7 --pop-size 64
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

V následujích grafech je znározněná závislost fitness různých metod na počtu iterací.
 - Pokud jsou metody na grafu buď jen varianty lokálního prohledávání, a nebo jen varianty evolučních algoritmů, pak znázorněný počet iterací odpovídá počtu iterací daných metod.
 - V případě porovnání lokálního prohledávání a evolučního algoritmu odovídá znázorněný počet iterací počtu iterací evolučního algoritmu. U lokálního prohledávání je skutečné číslo iterace rovno velikosti populace evolučního algoritmu (všude 64) krát znázorněné číslo iterace. Tímto způsobem vyjadřuje jeden krok na grafu stejné množství vyhodnocení fitness funkce pro oba typy algoritmů.
 - Celkový počet iterací pro konkrétní problémy je přizpůsoben pro lepší čitelnost grafu. Tedy např. pokud se v nějakém bodě všechny algoritmy zaseknou v lokálním optimu, pak je graf zobrazen pouze do tohoto bodu.

Hodnoty fitness jsou posunuty do 1 odečtením nejlepší známé fitness a následně zlogaritmovány. Optimum by teda na grafu mělo zhruba odpovídat hodnotě 0 (log 1 = 0). Výsledky jsou půměrovány z několika opakování (defalutně 7).

Vysvětlivky na grafech:
 - **local** - lokální prohledávání
 - **evo** - evoluční algoritmus
 - **move** - move perturbační operátor v kombinaci s city operátorem
 - **swap** - swap perturbační operátor v kombinaci s city operátorem
 - **rev** - reverse perturbační operátor v kombinaci s city operátorem
 - **cycle** - cycle crossover v kombinaci s uniform_city crossoverem
 - **order** - order crossover v kombinaci s uniform_city crossoverem
 - **heuristic** - inicializace daného algoritmu heuristikou

Pro měření používám 2 typy datasetů. První jsou datasety poskytnuté na stránkách předmětu, tedy soubory **a**, **b**, **c**, **d**, **e** a **f** (pořadí odpovídá velikosti datasetů). Vzdálenosti u těchto instancí nesplňují vlastnosti metriky (konkrétně trojúhelníkovou nerovnost), a proto tyto datasety nejsou úplně vhodné na vizualizaci (i kdybych bych udělal např. force directed layout, tak skutečné vzdálnosti nikdy nebudou odpovídat vzdálenostem ve 2D euklidovském prostoru).

Proto jsem si vygeneroval druhý typ svých vlastních datasetů, soubory **g1**, **g2** a **g3** (pořadí odpovídá velikosti datasetů), u kterých vzdálenosti odpovídají euklidovské metrice a mám uložené 2D pozice, které můžu přímo použít pro vizualizaci (bude v poslední sekci tohoto dokumentu). Data generuji tak, že nejdříve náhodně vygeneruji požadovaný počet bodů, poté jednotlivé body přiřadím do skupin pomocí algoritmu k-means.

Zde jsou velikosti jednotlivých problémů pro představu jejich složitosti:

| Dataset | Počet vrcholů | Počet skupin |
|---|---|---|
| a | 24 | 5 |
| b | 48 | 10 |
| c | 100 | 20 |
| d | 150 | 30 |
| e | 200 | 40 |
| f | 442 | 89 |
| g1 | 24 | 5 |
| g2 | 100 | 20 |
| g3 | 500 | 80 |

### Porovnání při základním nastavení parametrů

Zde jsou všechny pravděpodobnosti jednotlivých stavebních operátorů nastaveny na 50%. Tedy např. **evo move cycle** je evoluční algoritmus pravděpodobnostmi: 50% **move**, 50% **city**, 50% **cycle** a 50% **uniform_city**.

Je vidět že si většinou nejlépe vede local search s **rev** perturbací. Zároveň evoluční algoritmy mají sice pomalejší rozjezd, ale časem lokální metody doženou a mají někdy potenciál řešení vylepšit ještě dál.

![default_a.svg](out/gtsp/default_a.svg) 
![default_b.svg](out/gtsp/default_b.svg) 
![default_c.svg](out/gtsp/default_c.svg) 
![default_d.svg](out/gtsp/default_d.svg) 
![default_e.svg](out/gtsp/default_e.svg) 
![default_f.svg](out/gtsp/default_f.svg) 
![default_g1.svg](out/gtsp/default_g1.svg) 
![default_g2.svg](out/gtsp/default_g2.svg) 
![default_g3.svg](out/gtsp/default_g3.svg) 

### Porovnání různých variant lokálního prohledávání

Nyní se zaměříme na lokální prohledávání. Zde jsem se pokusil najít optimální pravděpodobnosti aplikace perturbačních operátorů pro lokální prohledávání. To jsem udělal tak, že jsem iteroval pravděpodobnosti od 0 do 1 s krokem 0.1 pro všechny kombinace **move**, **swap**, **reverse** a **city** perturbací a porovnával součet průměrných fitness skrze všechny datasety. Výsledné pravděpodobnosti byly: 0 **move**, 0 **swap**, 0.9 **reverse** a 0.9 **city**. Tedy operátory **move** a **swap** jsou ignorovány, oba **reverse** a **city** se provedou s pravděp. 0.81, buď **reverse**, a nebo **city** pravděp. 0.18 a ani jeden pravděp. 0.01.

Výsledky po zpětném zamyšlení vypadají docela rozumě. Byl vybrán **reverse** operátor který dopadl nejlépe v předchozích měřeních, **city** operátor musí být zahrnut pro mutaci měst v rámci skupin. Pravděp. 0.18 dává slušnou šanci na individuální perturbaci buď permutace skupin, nebo perturbaci měst ve skupině. Zároveň je malá šance že se neaplikuje ani jeden operátor, což výrazně zrychlí prohledávání.

Na následujících grafech jsou vždy znázorněny nejprve 3 varianty lokálního prohledávání totožné s těmi v předchozí sekci. K tomu navíc je zde **local tweaked**, který má nastavené výše zmíněné optimální pravděpodobnosti. Je vidět, že ve většině případů došlo k výraznému zlepšení.

![local_a.svg](out/gtsp/local_a.svg) 
![local_b.svg](out/gtsp/local_b.svg) 
![local_c.svg](out/gtsp/local_c.svg) 
![local_d.svg](out/gtsp/local_d.svg) 
![local_e.svg](out/gtsp/local_e.svg) 
![local_f.svg](out/gtsp/local_f.svg) 
![local_g1.svg](out/gtsp/local_g1.svg) 
![local_g2.svg](out/gtsp/local_g2.svg) 
![local_g3.svg](out/gtsp/local_g3.svg) 

### Porovnání různých variant evolučního algoritmu

Pro tuto část jsem původně chtěl podobně jako v předchozí sekci najít optimální pravděpodobnosti jednotlivých operátorů pro evoluční algoritmus. Toho je už ale mnohem těžší dosáhnout, protože k pravděpodobnostem perturbačních operátorů se přidají pravděpodobnosti crossover operátorů a máme tedy celkem 7 proměnných (hledat v kombinacích s krokem 0.1 by trvalo příliš dlouho).

Nejprve jsem to zkusil redukovat na 5 proměnných vynecháním **move** a **swap**, protože **reverse** se zatím ukázal jako nejlepší. To ale vedlo na zvláštní výsledky, které se výrazně měnily pokud jsem experiment zopakoval a při porovnání se základním nastavením parametrů se nejevily o moc lepší. Řekl bych, že je to způsobeno tím, že běh algoritmu je už tolik ovlivněn náhodou, že je velký rozptyl mezi jednotlivýmy běhy a je velice těžké je mezi sebou porovnávat v malém počtu běhů.

Poté jsem ještě zkusil random search na daných 5 proměnných a následně vylepšovat některá řešení lokálním prohledáváním (fitness byla vyhodnocena způměrováním několika běhů evolučního algoritmu s danými pravděpodobnostmi). Ale i tento přístu vedl na velice zvláštní a nestabilní výsledky, nejspíše opět z důvodů zmíněných víše.

Proto jsem nakonec tento přístup zavrhl a rozhodl se alespoň kvalitněji porovnat 6 konkrétních variant parametrů.

![evo_a.svg](out/gtsp/evo_a.svg) 
![evo_b.svg](out/gtsp/evo_b.svg) 
![evo_c.svg](out/gtsp/evo_c.svg) 
![evo_d.svg](out/gtsp/evo_d.svg) 
![evo_e.svg](out/gtsp/evo_e.svg) 
![evo_f.svg](out/gtsp/evo_f.svg) 
![evo_g1.svg](out/gtsp/evo_g1.svg) 
![evo_g2.svg](out/gtsp/evo_g2.svg) 
![evo_g3.svg](out/gtsp/evo_g3.svg) 

### Porovnání nejlepších variant různých algoritmů

![best_a.svg](out/gtsp/best_a.svg) 
![best_b.svg](out/gtsp/best_b.svg) 
![best_c.svg](out/gtsp/best_c.svg) 
![best_d.svg](out/gtsp/best_d.svg) 
![best_e.svg](out/gtsp/best_e.svg) 
![best_f.svg](out/gtsp/best_f.svg) 
![best_g1.svg](out/gtsp/best_g1.svg) 
![best_g2.svg](out/gtsp/best_g2.svg) 
![best_g3.svg](out/gtsp/best_g3.svg) 

## Vizualizace

### Iniciální řešení

vždy random a pak heuristika

![viz_init_random_g1.svg](out/gtsp/viz_init_random_g1.svg) 
![viz_init_heuristic_g1.svg](out/gtsp/viz_init_heuristic_g1.svg) 
![viz_init_random_g2.svg](out/gtsp/viz_init_random_g2.svg) 
![viz_init_heuristic_g2.svg](out/gtsp/viz_init_heuristic_g2.svg) 
![viz_init_random_g3.svg](out/gtsp/viz_init_random_g3.svg) 
![viz_init_heuristic_g3.svg](out/gtsp/viz_init_heuristic_g3.svg) 

### Nejlepší nalezená řešení

![viz_best_sol_g1.svg](out/gtsp/viz_best_sol_g1.svg) 
![viz_best_sol_g2.svg](out/gtsp/viz_best_sol_g2.svg) 
![viz_best_sol_g3.svg](out/gtsp/viz_best_sol_g3.svg) 

### Běh lokálního prohledávání

![viz_local_g2.gif](out/gtsp/viz_local_g2.gif) 
![viz_local_g3.gif](out/gtsp/viz_local_g3.gif) 

### Běh evolučního algoritmu

![viz_evo_g2.gif](out/gtsp/viz_evo_g2.gif) 
![viz_evo_g3.gif](out/gtsp/viz_evo_g3.gif) 