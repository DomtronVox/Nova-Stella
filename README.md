# Nova Stella
**This readme represents the goal not the current state of the project.**

* Genre: 4X Strategy Sandbox
* Inspiration: Roguelikes, vastness of space, project RHO, Aroura 4X
* Mod-able.
* Unique mechanic based around the concept of Level of Detail.

Nova Stella, Latin for New Star, is a 4X strategy game focused on exploring and developing a single, procedurally generated star system. The game is designed around a __Levels of Detail__ system allowing the player to delve as deep as they want into each mechanic and ignore others. The games mechanics are built around the traditional 4X game pattern:

* Exploration: Study the orbital bodies discovering their uses and dangers then research the biological and minerals discovered on them.
* Expand: Build stations, planet side colonies, and ships. Design new ships and plan out logistics for all your stations.
* Exploit: Mine up all the resources you discover and use the lifeforms for research and improvement of your people.
* Exterminate: Fight against other factions vying for control of the system.

Or don't and just hand off any thing to the games automation if you don't want to mess with it.


## Starting Scenarios 

The game has a number of starting scenarios you can play as well as the ability to make your own. Be a small survey vessel, fleeing refugees, hired construction company for a group of colonists, or build your own scenarios. Game objectives can be ignored, but results could turn out poorly for you if you're not careful.


## Mods
The game can be extended by adding more data for generation to pull from or through the use of scripting to expand on the mechanics.


## Design Goals
* **Level Of Detail**: Allow the player to look at most things in multiple levels of detail based on their interests. For example this means they can deal with only surface information about geology if they really don't care about that aspect, but can have a lot of detail about biological life because they are interested in it - or vice versa.

* **Levels of Interaction** (aka Automation): A system of automation that is reactive, automating details of the game but moving out of the player's way if they want to do something themselves. Players can choose what is automated by the game and what they do themselves to the point that the entire game could run on it's own. For example if a Player likes designing ships, allow them to just do that.

* **Four Categories of Game Mechanics**
    * Discovery (Exploration and Research), 
    * Building (Design and Construction), 
    * Transport (Logistics and Economics), 
    * Combat

* **Procedural**: Provide organized randomness allowing for "endless" content that makes sense.
    * Generation of a stable systems in various configurations (like binary/trinary stars).
    * Events that spawn interesting situations to solve or benefit from.

* **Simulation**: Overall try to be realistic while remaining performant on user computers.
    * **Physics**: Realistic physics (movement, heat) to allow for interesting edge cases, like the [switching orbits of Janus and Epimetheus](https://en.wikipedia.org/wiki/Epimetheus_(moon)#Orbit).
    * **Geological**: Basic simulation of planets and smaller bodies surface and sub-surface makeup.
    * **Atmospherics**: Basic simulation of atmospheres to allow for example terraforming.
    * **Population/Economic**: Basic simulation of population groups and what they need to survive.
    * **Engineering**: Basic simulation for artificial vehicles/structures and their movements.



# Compiling

[Install rust and cargo](https://www.rust-lang.org/install.html). Don't forget to add rust's directory to your PATH environment variable (bottom of linked page).


Run the following command to build the GUI client:

`cargo run`

Note if your connection is slow it helps to run `cargo fetch` before cargo run for some odd reason.












