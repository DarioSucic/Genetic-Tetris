# Genetic-Tetris

### Description
This is a very basic implementation of the classic game of [Tetris](https://en.wikipedia.org/wiki/Tetris) developed 
for the purpose of exploring genetic learning algorithms. The implementation is written in Rust, using the [ggez](https://github.com/ggez/ggez)
library for rendering and input handling.

Three different agents are implemented:
* Random Agent - Making completely random moves
* Human Agent - Letting you play the game
* Genetic Agent - Learning to play the game by itself

### Video Showcase
[![Youtube Video](https://img.youtube.com/vi/iQggYrU_yrk/0.jpg)](https://www.youtube.com/watch?v=iQggYrU_yrk)

### Learning implementation
TL;DR The Genetic Agent learns an appropriate weighting of a set of heurisic functions on the Tetris state map.

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
3. The Loss is calculated as the dot product between the heuristic function vector and the agent's weight vector.
4. The action with the lowest loss is chosen.
