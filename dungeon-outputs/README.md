About the Digger/BSP "ruins":
- "Ruin" effect is achieved by combining the BSP (or digger) output with Cellular Automata. Really simple.
- The "Inverted" effect on the Digger is achieved in the same way.

The "comb" images are combinations of different algorithms on the same map:

- comb06:
    - Pink: inverted digger
    - Red: BSP + WFC for internal architecture
    - Yellow: BSP ruin
    - Green: Cave (random walker + cellular automata)

* comb04:
    - Yellow: Forest (cellular automata)
    - Red: WFC
    - Pink: BSP + WFC for internal architecture
    - Green: BSP ruin

- comb03: 
    - Pink: Inverted digger
    - Red: WFC
    - Yellow: BSP ruin

- comb02:
    - Red (center): WFC
    - Bottom: BSP
    - Left: BSP ruin
    - Top: Cave

- comb01:
    - Pink: BSP ruin (1)
    - Red: Forest
    - Yellow: BSP ruin (2)

Note that there're effectively two types of BSP ruins, which can be clearly seen on comb01.png:
- (1) Achieved by first running the Cellular Automata algorithm and THEN running the BSP algorithm over it.
- (2) Achieved by first running the BSP algorithm and THEN running the Cellular Automata over it.

Each algorithm is implemented on /src/map_gen, and some pipelines are on /src/map_gen/mod.rs.
WFC, however, is not working 100% -- but it still gives interesting results
sometimes. 

Some regions are not fully connected, but such regions can be culled or
connected with the help of the flood fill algorithm afterwards.
