This document tries to lay down the design needed for the NeoSOL code base.

# Features
## General
* Programming will be in Rust and possibly a scripting language for modding later on.
* We are going to be seperating out the simulation from the client to allow changing the client later if needed.


## Level Of Detail (LOD)

LOD is an important concept for the code design. In our case LOD applies to the simulation and data not graphics. A low level of detail means the simulation is shallow with few details while a high level of detail means the simulation is deep with many details. Low LOD uses the fewest computer resources to maintain while high LOD has more detail and is more interesting and/or realistic.

The idea is to allow variable LOD in each area of the simulation, so it can be shallow in one area and deep in another. This A) allows players to only mess with stuff they are interested in and B) makes the simulation easier to run on machines.

## Entity Component Systems (ECS)
Entity component systems avoids polymorphism in game objects, replacing it instead with data inclusion. In brief, components just hold data, systems just hold logic that processes components, and entities are component containers. Systems only process entities that have their specific components.

This should work well with the LOD Simulations. Low detail simulations can define base systems and components. Each progressively higher detail simulation will expand on the simpler ones. Then to turn on a higher LOD simulation on, you just add a more detailed component creating it based on the lower detail component.

For example you could have a component detailing a ship, and it would have only basic details, but then a player can do some micromanagement and a higher detail component would be attached to the system extending the lower detail one. The system would check if the higher detail component exists and if so does additional computations.


# Engine Design

We are going to split the program between Simulation and client. Simulation will implement all the things that actually affect the in game world while the client is just a window into that world implementing all the stuff needed for players to manipulate it. This is actually fairly easy to do with Rust and specs (ECS library) as each client can just register more components and spin up additional systems for it's specific setup while relying on existing data that the simulation itself has defined.


# Simulation Design

## Physics
This sub-system handles calculating the orbits of astrological bodies, heat simulations, etc. 

### Orbits
There are several ways to go about calculating orbits. 
* Brute forcing the Newtonian gravity formula between every object is fairly accurate, but computationally expensive.
* Keplar's orbital parameters are computationally simple as well as more human readable, but the different bodies would not interact.
* In some cases, to be accurate we would need to apply relativity based equations.

Ideally we would use Newtonian calculations in most situations where the body's orbit is subject to change, relativistic calculations only where absolutely necessary, and keplar orbital perimeters wherever the orbit is stable.

For now we:
* have a satellite component which tracks mass, position, and speed. 
* use a single system that calculates newtonian based orbits.
Note: Mass will need to be updated by any system that may modify the objects mass like say a system that manipulates cargo on a ship.


### Heat
TODO



## Personnel/Economics
This sub-system handles simulation of the sentient entities and their actions. It implements the different personnel categories and the faction system and simulates growth, decline, and migration of the populations and their economic pursuits.

## Engineering
This sub-system would be responsible for defining and maintaining any artificial structures.


## Atmospherics
This sub-system would be responsible for simulating atmospheres. Likely both planetary and artificial ones.

## Geological
This sub-system would be responsible for simulating ground and underground parts of natural bodies and their exploitation.

# Module Brainstorming
## CES
planet/moon/asteroid
   is satellite
   has atmosphere (maybe zero)
   has geology
   has ecology (maybe zero)
   contains ASMOs
   
space station/ship/habitat
    is satellite
    is ASMO 
    has atmosphere (maybe zero)
    has ecology (maybe zero, maybe should be handled as a different sim from a planet ecology)
    has personnel

colony
    is grounded (relative to a satellite)
    has atmosphere (maybe open or pulling from the satellite)
    has managed systems (buildings)
    has personnel

leader
    is personnel (different from has personnel? maybe should be leader)
    has authority position

faction
    has personnel
   







## Locations (TODO REWORK)

### Overview 
Space is vast even when restricted to one solar system. The game also needs to be capable of simulating fine details. To do both in a way that a player and their computer can manage, layered location based maps are used. Locations on these maps link to other maps with more detail until you reach the highest detail supported. To accomplish this relational maps are used. Relational maps have a list of locations with relative position data between each. Locations are points of interest that the player may want to interact with. This includes planets, moons, asteroids, comets, stations, ships, continents, landmarks, ect. For example the overall system map's locations would be space bodies like planets, collections of asteroids, comets, and traveling ships but would not include moons because they belong in a map with more detail. A planet orbital map would have locations for moons, dust/ice collections, and stations or ships that are not traveling. In addition new locations can be added in between locations to allow construction of stations and ships or to perform certain missions.

Most space bodies have two location maps: one for the surface and one for orbit unless they are not large enough to have an orbit like most constructed objects. These two maps also exhibit two types of maps that hold different data: orbital and ground maps. In orbital maps each location has a pole, polar coordinate, and an orbit vector associated with it. For example a moon would have its planet as the pole, a polar coordinate describing its current position, and an orbit vector giving the polar coordinate for any specific time. In ground maps (TODO what special data would ground map locations have?).

While Relational maps are used in most places, some locations may need a grid map. (TODO expand on why where and when tile/grid maps are used)

### Sun
The sun has all other bodies orbiting it. Because of this the Sun is it's own pole and has no polar coordinate or vector in the system wide map.

### Planets
Planets are large chunks of rock with vast land and riches, but the significant gravity makes transport expensive. Their pole is the Sun and they have two location maps: a surface map and an orbit map.

### Moons
Essentially large asteroids that orbit planets. Lower gravity but fewer resources. Also have two location maps same as planets.

### Asteroids and Comets
Small chunks of rock or ice that orbit the sun. Asteroid locations represent clusters. These do not usually have orbital maps. 

### Stations and Ships
Constructed by the player
