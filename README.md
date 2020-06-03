# Genetic-Tetris

### Description
This is a very basic implementation of the classic game of [Tetris](https://en.wikipedia.org/wiki/Tetris) developed 
for the purpose of exploring genetic learning algorithms. The implementation is written in Rust, using the [ggez](https://github.com/ggez/ggez)
library for rendering and input handling.

The quality of the Tetris implementation is fairly low, and it is missing features such as block rotation near edges, gradual speed increase, and next-block visualization. (in fact the next block is currently invisible to the learner as well)

Three different agents are implemented:
* Random Agent - Making completely random moves
* Human Agent - Letting you play the game
* Genetic Agent - Learning to play the game by itself

### Video Showcase
[![Youtube Video](https://img.youtube.com/vi/iQggYrU_yrk/0.jpg)](https://www.youtube.com/watch?v=iQggYrU_yrk)

### Learning implementation
TL;DR The Genetic Agent learns an appropriate weighting of a set of heuristic functions on the Tetris state map.

##### [Heuristic Functions](https://github.com/DarioSucic/Genetic-Tetris/blob/master/src/misc/heuristics.rs)
The algorithm makes use of the following 4 heuristics:
* Surface roughness heuristic - Defined as the sum of height differences for adjacent columns.
* Height heuristic - Defined as the height of the highest non-empty block on the map.
* Line completion heuristic - Defined as the number of full lines. (i.e lines containing no empty blocks)
* Ceiling gap heuristic - Defined as the number of empty blocks under non-empty blocks.

Interestingly enough, a proper weighting of these simple heuristics produces a surprisingly competent Tetris player.

##### Training Process
1. Generation of Genetic Agents is initialized with random weights.
2. Every agent's performance is evaluated (see below), giving a set of fitness values.
3. The fitness values are normalized so that their sum is 1.
4. A selection is made from the population, weighted by the normalized fitness values.
5. A new generation is produced by breeding the selection individuals, and possibly introducing mutations.
6. The process is repeated from 2.

##### Performance Evaluation
To evaluate the performance of an individual, a certain number of runs are performed, and the average score over the runs is calculated.
For each run, the agent selects an action as follows:
1. All possible drop locations (for all rotations) are calculated, resulting in state maps.
2. Each state map is passed to a number of heuristic functions, each producing a numeric output.
3. The Loss is calculated as the dot product between the heuristic output vector and the agent's weight vector.
4. The action with the lowest loss is chosen.
