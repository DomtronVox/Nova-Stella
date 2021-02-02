This document tries to lay down the design needed for the NeoSOL code base.

# Features
## General
* Programming will be in C/C++ and possibly a scripting language for modding later on.
* Strictly follow a Model-View separation so we can have several front ends.


## Level Of Detail (LOD)

LOD is an important concept for the code design. In our case LOD applies to the simulation and data not graphics. A low level of detail means the simulation is general while a high level of detail means the simulation is detailed. Low LOD uses the fewest computer resources to maintain while high LOD has more detail and is more interesting and/or relistic. 

## Entity Component Systems (ECS)
Entity component systems avoids polymorphism in game objects, replacing it instead with data inclusion. In brief, components just hold data, systems just hold logic that processes components, and entities are component containers. Systems only process entities that have their specific components.

This should work well with the LOD Simulations. Low detail symulations can define base systems and components. Each progressivly higher detail simulation will inharit from the simpiler ones. Then to turn on a higher LOD simulation on, you just add their component to an entity. 

## Modular

Breaking the code up into Modules can provide several benifits. First, it allows altering code on a per modual basis reducing the possibility of breaking other parts of the code, i.e. encapsulation. Second, it allows swapping out modules as needed changing the programs features. Finally if it is sufficiantly self-contained, when one module gets processor intensive it can be pushed to it's own thread much easier.

While modules should generally be self-contained, it is sometimes nessisary to depend on another module. However, modules should maintain clear dependancies to avoid rats nests where modules are inter-connected.

# Design

The codebase is broken into two sections: engine and modules. An engine that is fairly interconnected and handles all the underlying technical details not directly related to the game (UI, threading, etc). Modules, or mods, add in the actual game mechanics and data.

# Engine Design

The engine is further divided into three systems: Core, Model, and View.

The Core system should be kept to a minimum. It contains the main loop, cofiguration loading and data structures, and initilisation code.

The View handles UI including audio playback, displaying the world, and displaying and interactions with the GUI and HUD.

The Model handles loading modules and implementing the underlying code allowing for the game world to function.

If it becomes relavant there will likely be a third system to handle networking protocal.

## Core

## View

## Model

The model has two main parts.
* Module: Loads and manages the metadata of moduals.
* Simulation: Defines the core data structures for the game world.

# Module Design

While technically anything can work for modules, we are going to plan out specific modules to make a cohesive game.

## Satellite Mod
This system handles calculating the orbits of astrological bodies. This includes objects orbiting the sun and objects orbiting other objects. It also includes basic details about each object like mass, size, speed, and some basic volume data. 

## Personnel Mod
This system handles general simulation of the sentient entities. It implements the different personnel categories and the faction system and simulates growth, decline, and migration of the populations.

## ASMO Mod
The ASMO or Artificial Sentient Made Objects sim is directly descended from the Satellite and Personnel sims. SMO's contain personnel so 


#Module Brainstorming
##CES
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
    has autority position

faction
    has personnel
    
