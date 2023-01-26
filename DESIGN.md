See README.md file for a high level summary of the game.

# Design Goals

* **Domain Simulations**: Distinct but lightly interconnected simulations over various aspects of the game world. Do not necessarily need any input from the player but player actions do effect them. We want to try and be as accurate as possible, but will obviously have to make concessions to actually allow it to run on computers.

    * **Level Of Detail**: Each simulation is built to have multiple levels of detail. This term is well known in rendering for it's ability to do less work for models far away and not as easily visible which decreases computer resource use. In our case it works similarly, but, instead of purely physical distance, sim LoD is based on the player's interest in simulating that aspect of the game world (or their computer's ability to handle the simulation). Multiple levels of detail can be used at once as it will be applied per entity. 

    * **Simulation Categories (WIP)**
        * Orbital. Movement of orbital bodies.
        * Geology. The interior makup of natural orbital bodies.
        * Atmospherics. Both for planets and artificial enclosures.
        * Biosphere. Both artificial and natural.
        * Engineering. Function of artificial mechatronic systems aka the physical workings of ships and stations. 
        * Infrastructure. Function of planetary developments.
        * Personel. Specific crew simulation of the player's faction.
        * Economic. General flow of people and goods.
        * Faction. Interaction of political blocks.
        

* **Gameplay Automation**: A system of automation that is reactive, automating details of the game but moving out of the player's way if they want to do something themselves. Players can choose what is automated by the game and what they do themselves to the point that the entire game could run on it's own. For example if a Player likes designing ships, allow them to just do that.
    * Discovery (Exploration and Research) 
    * Building (Design and Construction) 
    * Transport (Logistics and Economics)
    * Conflict (Combat and Factions)


* **Procedural**: We want a modular and layered Procedural generation system.
    * Modular: Each thing existing in the game world can have several functions dedicated to generating them in various ways. 
    * Layered: These are then grouped together into layers where upper layers are more general details which will trickle down and effect how the lower layer generation procedures are run.
    * The desire is to allow adding in new procedures with as little hassle as possible while still allowing everything to be coherent and interconnected.



# Simulation
