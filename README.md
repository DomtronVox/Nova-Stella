# Nova Stella
**This readme represents the goal not the current state of the project.**

## Quick Summary 
* **Genre**: 4X Strategy Simulation
* **Inspiration**: Roguelikes, vastness of space, project RHO, Aroura 4X
* Mod-able.
* Focus on the idea of Level of Detail simulations.

## Summary
Nova Stella, Latin for New Star, is a 4X sci-fi strategy game built onto a collection of simulations that attempt to be as accurate as is feasible for their domain. Each simulation is built with __Level of Detail__ kept in mind, allowing the player to choose how detailed they want different parts of the game's simulation. 

The gameplay is focused on exploring and developing a single, procedurally generated star system and follows the traditional 4X game pattern:

* Exploration: Survey the orbital bodies to discover their uses and dangers, then research the biological and mineral materials found therein.
* Expand: Build stations, planet side colonies, and ships. Design new ships and plan out logistics for all your stations.
* Exploit: Mine up all the resources you discover and use the lifeforms for research and improvement of your people.
* Exterminate: Fight against other factions vying for control of the system.

Players can configure the star system's initial state then pick or design a starting scenario which will greatly affects the way the game will run. Players can have soft goals that can lead to a "victory", but the game only really ends when the players stop playing.

## Mods
The game can be extended by adding more data for generation to pull from or through the use of scripting to expand the different simulations.


# Compiling

[Install rust and cargo](https://www.rust-lang.org/install.html). Don't forget to add rust's directory to your PATH environment variable (bottom of linked page).


Run the following command to build the GUI client:

`cargo run`

Note if your connection is slow it helps to run `cargo fetch` before cargo run for some odd reason.












