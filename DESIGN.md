See [README.md](README.md) file for a quick summary of the game.

# High Level Overview

## Design Goals

* **Domain Simulations**: Distinct but lightly interconnected simulations over specific domains of the game world. Do not necessarily need any input from the player, but player actions do affect them. We want to try and be as accurate as possible, but will obviously have to make concessions to actually allow it to run on computers.

    * **Level Of Detail**: Each simulation is built to have multiple levels of detail, LoD. This term is well known in rendering for it's ability to do less work for models far away and don't need high detail decreasing the program's resource use. Our use of the term works in a similar way, but, instead of physical distance being the decider of what level to use, sim LoD is based on the player's interest in simulating that aspect of the game world (and their computer's ability to handle the simulation). Interest meaning both automatically deciding if something is important to the player, and taking cues from the players actions. Multiple levels of detail can be used at once as it will be applied per entity. 

    * **Simulation Categories (WIP)**
        * Orbital. The function of orbital bodies.
        * Mapping. Tile based mapping of large satellites. 
        * Geology. Simulation of mineral resources and structure of natural satellites.
        * Atmospherics. Both for planets and artificial enclosures.
        * Biosphere. Both for artificial and naturally occuring Ecosystems.
        * Mechatronic. Function of artificial mechatronic systems aka the physical workings of ships and stations. 
        * Infrastructure. Function of planetary developments.
        * Faction. Interaction of institutions and political blocks.
        * Personnel. Specific crew simulation of the player's faction.
        * Economic. General flow of people and goods.
        

* **Gameplay Automation**: Reactive gameplay automation. Automating details of the player's interaction with the game world, but moving out of the way if the player wants to do something themselves. Players can choose what is automated by the game and what they do themselves. For example if a Player likes designing ships, allow them to just do that.


* **4x Style Game**: We want to roughly follow the 4x paradigm:
    * Explore (Exploration and Research) 
    * Expand (Design and Construction) 
    * Exploit (Logistics and Economics)
    * Exterminate (Combat and Factions)


* **Procedural**: We want a modular and layered Procedural generation system.
    * Modular: Each thing existing in the game world can have several functions dedicated to generating them to allow different approaches and outcomes. 
    * Layered: These are then grouped together into layers where upper layers are more general details which will trickle down and effect how the lower layer generation procedures are run.
    * The desire is to allow adding in new procedures with as little hassle as possible while still allowing everything to be coherent and interconnected.


* **Moddable**: Mod support is important to allow players to customize their experience and expand the game outside the core design goals.
    * One type of mod is the data pack which just adds data to existing mechanics. There will be the ability to append and override existing data tables for procedural generation, define starting conditions/senarios, and configure other aspects of the game.
    * The other type is functional mods that include code which change the games mechanics.
    
## Gameplay Overview

The player chooses a starting scenario and a star system when beginning a new game. The star system is either generated based on various requirements (including fully random parameters) or pre-created to mimic something (Sol, other real solar systems, solar systems from fiction, etc). The starting scenario provides the initial setup like soft goals, starting equipment, Other factions you start with, planned events, and so on.

Once in the game, the player has free reign to interact with the solar system how they see fit; trading, become a pirate, build habitats, mine asteroids, skim gas giants, start planetary or orbital settlements, and more. Depending on the starting scenario, the player may have some long term **soft** goals, for example reestablishing contact with the home system or send out colony ships to new systems, but these can generally be ignored though there may be consequences depending on the scenario. 

Overall the game follows the 4X game pattern. Exploring the solar system to identify the various orbital bodies and their contents and further your research goals. Expanding your starting equipment - stations, ships and the like - to improve your capabilities. Exploiting the natural resources to fuel further expansion. And in some cases exterminating opposing factions.

The game is built around the ideas of scaled Level of Detail and automation of tasks allowing the player to focus on aspects of the game they like. You can work to develop the whole system as a large faction that brings the other factions under it's control, just a small faction under a larger faction providing specific services, or focus on a single ship/station/colony and it's crew doing a specific jobs. Likewise you can order a whole fleet of ships to accomplish general tasks or control a single ship more precisely.

To accomplish this the simulation needs to be able to scale, hence LoD, depending how much focus the player gives to a certain in-world entity. On top of that need, is the need for several layers of context specific UIs that can give less or more detailed control and information. 

### UI

Very rough sketch at the moment. Will likely change significantly.

**System View**

There will be a view that provides general system information. All orbital entities can be viewed and filtered for in this view so long as they have been discovered. This view also give access to much of the rest of the game allowing to focus on different ships and colonies.


**Mission View**

Focused on allowing you to setup missions for your various ships, providing a low detail, high automation control of everything you own.

The player designs missions and assigns crew who will complete them. Missions are tasks that gather raw data for science, collect resources, do construction, move people and/or materials, and anything else that can be done in the game. These generally require one or more vehicles and, depending on what needs to be done, special equipment. Missions can be set to repeat so routine resupply and trade missions can be created. Some missions can be uncrewed, but most need personnel. 

**Entity View**

Allows you to view specific information about various entities in the game making it the meat of the UI. Highly context based, showing very different things depending on if you are looking at a planet or a player owned ship.

Planets would show the surface. Colonies show infrastructure and economics. Ships and stations would show crew and engineering information.

**Faction Management**

Allows you to manage your faction policies and look as well as interact with other factions making deals and so forth. Contextual based on what type of faction you are.


**Ship/Station Designer**

Module based designer that lets the player layout a ship, station, or similar and dictate it's cabibilities.


# Design Details 

We are dividing the codebase into a game world server and player client. The game world server can be run standalone or with a player client connected to it, mostly for testing pourposes. The player client provides player interactivity to the game world simulations via the UI. We may build a few clients out depending on the needs. Initially we will likely have a simple client that is just for viewing the game world, then expand it once the simulations are working well.

Multiple player clients is a long term possible but undecided goal, for now we will focus on singleplayer.
 
## Game Starting Conditions

### Faction Agenda

The Agenda sets the faction's long term soft goals for the player and AI. Agendas are made up of an overarching goal along with grouped smaller tasks that lead up to accomplishing the Mandate. Agendas also come with some configurable events that will happen when conditions are met.

Essentially the Faction Agenda functions as a player guide while also allowing some story to be injected into the game. Some Faction Agendas can be very minimal while others could have a lot of details built into them.

Faction Agendas can define suggested options for both the Star System and Starting Faction Conditions, but these will not be enforced allowing the player to pick any preset or use custom settings for one or both. 

### Star System

Settings that can fine tune the starting solar system. They can individually be random, use a range, or specified exactly. Presets can be chosen that recreate a system and the settings can also be saved locally as a custom pre-set for later reuse.

Solar System Configuration Ideas (will mostly depend how we program the simulations themselves)
* Star 
    * Count: Allowing for at least Binary and Trinary stars.
    * Type: Effects satellites generation and luminosity calculations.
* Number and type of planets
    * lots of variety available here, setting values or ranges for each planetary body.
    * Planet count range
    * Each planet
        * Type
        * Size
        * Satellites 
* Asteroids
    * Mineral richness
    * Number of belts
    * density per belt
* Level of Life (ranging from existing space level sentient life to no life at all)
* Exotic Cases
    * Within a nebula


### Starting Faction Conditions

The last setup window relates to the state of the players starting faction. This encompasses what ships, cargo, and crew they have from day one. Just like the Star System configuration, the starting conditions will have available presets as well as allow the player to save their own custom designs.

The following will be areas that can be defined:
* Faction name, fluff description, and logo, preset or imported.
* A list of ships where each ship can be designed via the normal in game ship designer or randomized based on class and intended function.
* Cargo manifest that will be spread across all ships cargo bays, and cannot exceed their capacity.
* Personnel that make up your faction.
    

## Simulations

The game world is built around the ECS paradigm. As such each simulation is more a set of specific components and systems that get run on the game world entities.

This is helpful as we will be able to build out several ECS systems and components to allow for multiple levels of detail. The detail can also be different on different entities allowing low accuracy simulations to be made on less important entities while high accuracy ones can be applied to important entities.

Detail level can be switched via a number of conversion functions that can take the component(s) used for one level of detail and convert it to the components of another level of detail. For example in the orbital simulation we can convert a Kepler equation based orbit to equations for Newton's Laws of Motion which is a bit more resource intensive but also more accurate. We just need to be careful not to perform the conversion too often as some could be costly resource wise and lose us any performance gains from this system.

So simulations are divided into multiple levels of detail, and each level of detail is composed of systems and components. We then have conversion functions that can shift the level of detail for a specific entity. Finally each simulation would have checks to decide when an entity needs to change it's level of detail for that specific simulation.

We will also need to keep in mind that simulations are not self contained but interact with each other. For example the Biosphere can effect Atmospherics and vice versa.

### Orbital. The function of orbital bodies.

The orbital system is one of two positioning systems we use, and is used to simulate the movements of satellites, both natural and artificial.

Due to the complexities involved, we are breaking this into several sections.

#### Abstractions

First, of all we need a way to create simplified abstractions. This is primarily useful for smaller, multitudinous satellites, like planetary rings and asteroid fields. Both these things have a similar orbit around their respective primary orbital bodies, have many items spread somewhat evenly throughout the orbit, and could technically be depleted if a big enough operation is under taken.

However, it is obviously untenable to simulate hundreds of thousands of individual entities for each ring or belt. Hence making an abstraction where the group is represented by one entity, but can have specific satellites singled out of the mass spawning an entity that the player can interact with.


#### Basic Orbits

A near perfect simulation would use Newton's equations for gravity mixed with some relativity based equations in certain cases. However, this is a n-body problem that is too resource intensive to brute force at scale. In addition predicting orbital paths with this method is even more resource intensive. So we will reserve the Newton Gravity Equations for a **high** level of detail setting. Though we will likely not try to tackle the relativity aspect even at this level of detail.

The next best simulation is using Keplar's orbital parameters. They are able to predict at any time the location and speed of objects and are significantly less resource intensive. However (assuming my understanding of the equations is sufficient) they can not take into account the effects of other orbital bodies, rather just describe the current known orbit. So this will be placed as the **medium** level of detail setting for orbital motion simulation.

Using only Keplar's orbital parameters would leave out fun quarks like the switching orbits of Saturn's moons Epimetheus and Janus which essentially swap places maintaining a near identical orbit. To allow for this and maintain reasonable resource usage we will use Newtonian calculations only in situations where the body's orbit is subject to change, and keplar orbital perimeters wherever the orbit is stable. To accomplish this we will need to run regular checks on all satellites and when they approach another body based on distance and mass of the bodies, we will convert to Newton's equations, until the bodies are far enough away at which point they get converted back to Keplar's orbital parameters. 

The design is roughly:
* A core "Satellite" ECS component which tracks mass, position, and speed. 
* Two different ECS components for the two simulations.
* Two ECS systems to handle the two simulations.
* A check system that checks all "Satellite" ECS components and converts based on mass and distance to the nearest body.
    * Sooner then later we will need to implement a distance based grouping system so each satellite does not need to be compared to every other Satellite.
Note: Mass will need to be updated by any other simulation that may modify the objects mass like say a when the cargo on a ship is changed.


#### Orbital Changes

TODO We need a way to calculated transfer orbits automatically, or allow the player more direct control (maybe).


### Mapping. Tile based mapping of large satellites. 

TODO Debating, would allow tile map type view of planet surfaces and large orbital settlements like O'neal cylinders. Would allow precise positioning of things like colonies on planets. Also provide more detail to planets. Might be better to just have some sort of node/site system that plops down colonies on the planets surface and does not have the colony layout shown just a technical ui listing out colony stats.

### Geology. Simulation of mineral resources for natural satellites.

TODO

### Atmospherics. Both for planets and artificial enclosures.

TODO

### Biosphere. Both artificial and natural.

TODO

### Mechatronic. Function of artificial mechatronic systems aka the physical workings of ships and stations. 

TODO

### Infrastructure. Function of planetary developments.

TODO

### Faction. Interaction of institutions and political blocks.

Factions:

* Represent organized collections of people. 
* Are typed based on their stated goals. 
* Can be allied, neutral, or in outright conflict with other factions of similar type. And can be subservient to other factions. 
* Have their own knowledge/technology base. 
* "own" people and equipment (ships, stations, colonies) and can lay claim to parts of or entire natural satellites.

The player is represented by their own faction which they have full control over. However, the player's actions can drive away personnel to other factions or cause computer controlled factions to come down on them. The player's faction will often be subservient to other factions, meaning they will need to work within that factions rules or be punished for stepping out of line. However the solar system is large and even as a small faction the player can avoid becoming an underling of a bigger faction. Plus there is always the option of the player building up to become their own power house. 

Computer controlled factions can merge, split, and change allegiances to other factions under certain circumstances. The player's faction can also do so, but is much more in the player's control.

Knowledge is handled on a per faction basis and can be traded. (See Knowledge - Data Acquisition, Research, and Technology section). 

### Faction Types and Interactions

Players can freely change their factions goals and type, but the computer factions will react to this accordingly. If the player's faction suddenly claims to be a government of their own, other government factions may try to squash you. If the player claims to be a commercial faction, but builds up large amounts of armaments, other factions may respond unfavorably to it.

List of faction types:
* Government
    * Government factions will claim wide sweeping sections of the solar system, then try to maintain control of it via military expansion. 
    * Any other factions within the government faction's domain will be requested to be subserviant. If factions do not agree, the government will try to force them out of the territory.
* Local Government
    * Local Government factions are subservient to a government and focus on the development of infrastructure for a small region of that government's territory be it a tract of land or an orbital settlement.
* Commercial (with specialization options like hospitality or resource extraction)
    * Commercial factions focus on a specialization and try to grow their holdings to best exploit the resources that fall under their domain.
* Civilian (TODO not quite happy with this one, may drop)
    * Civilian factions are especially large and/or powerful groups of people that do not adhere to any particular government and mostly just focus on developing infrastructure.
    * Civilian factions often upgrade into a government faction or to a sub-government faction if they get Incorporated into a government faction.
* Outlaw
    * Outlaw factions specifically exist to take goods from other factions.
    * Government Factions try to destroy these factions.
    * These factions are formed from particularly angered personnel groups. Will form at a faster rate if they are unable to form other factions.

#### Physiology/Culture

TODO Would be important information for the computer to pull from to determine how it should act.


#### Knowledge - Data Acquisition, Research, and Technology

Scientific Data is the representation of work put into data missions. There are as many types of Raw Data as there are branches of science.

Research is the process of converting Scientific Data points into Science points. Science points increase until the branch of science gains a level. How many Science points needed are random. 

Technologies are procedurally discovered on each branch level up. Gaining a technology means you will either, gain a bonus relevant to the branch, or unlock a milestone technology which is a new "thing" you can use. Milestone technologies are obviously predefined while the random bonus is procedural and based on amount of work needed to acquire that level up.

As Raw Data is collected, research done, and Technologies discovered the Solar Codex will be filled with procedurally generated entries specific to the gmae's solar system which the player can review. Location and time tags will be incorporated into the information. This is almost entirely story fluff to provide flavor and worldbuilding.

Aside from scientific data, there would also be some other kinds of data that could be shared with allies or traded to interested factions. Big one may be faction operations data like what infrastructure they have and where it is.

## Contracts

**Contracts** are jobs that factions can put out for other factions to complete. For example a colonist habitat might hire the player to expand the habitat because of overcrowding and in exchange give the player processed goods. The player's faction can also put out contracts to get the AI to do things for them.


### Personnel. Simulation of people.

The Personnel simulation represents groups of people. Personnel provide work and make up the factions. There are several types of personnel that the player will need to deal with.

* Crew: Under a Faction's direct control, crew members need to be kept happy, but will build, mine, research, and fight as needed.

* Civilians: People that are doing their own thing under their faction's governance. They are part of the government type factions, but do not necessarily work directly for it instead doing business in the background as individuals. The player can source contracts from civilian groups and receive payment (in goods or money). Crew can become Civilians and Civilians can be hired as Crew.

* Leaders: Hold leadership positions in Factions. Provide special bonuses or enables a Faction to do certain things but can also mandate things the faction (or other subservient factions) needs to do.

#### Leaders

Since Crew and Civilians can get into the thousands or billions, there needs to be a way to interact with large groups of personal. This is done through Leaders. Each leader has a lot of detail associated with them. Leaders in your faction are obviously yours to command, but players will also need to interact with other faction leaders depending on the situation, especially if the player faction is subservient to another faction.

Leader info as follows.

* Fluff (Very little gameplay effect other then worldbuilding depth)
    * Full name
    * Homeworld
    * Likes
    * Beliefs
    * ect,
* Character
    * Specialization
    * Age
    * Mental traits
    * Skills/Bonuses
    * Driving goals


#### Individuals (Crew and Civilians)

If you want a higher LOD for select personnel then a simplified version of the leader info is used. 

* Fluff
    * Full name
    * Homeworld
    * Likes
    * Beliefs
    * ect,
* Character
    * Specialization
    * Age


### Economic. General flow goods.

TODO

## Procedural Generation

TODO


# Misc Ideas to help guide design

## Starting mission presets

* Survey Mission
    * Ship: Small, self-sufficient ship, one shuttle, research equipment.
    * Personnel: 4-8 Crew
    * Goals: Collect and categorize data about the solar system then transmit it back to the home world.

* Pioneer Colony Mission
    * Ship: Medium, mining and construction capabilities.
    * Personnel: 12-24 Crew. 
    * Goals: Build a sustainable homestead and survive.

* Commercial Colony Mission
    * Ship: Medium, mining and construction capabilities
    * Personnel: 12 Crew, 60 Civilians.
    * Goals: Initial goals build mining and industrial enabled colony habitats.

* State Colony Mission:
    * Ship: Large, mining and construction capabilities
    * Personnel: 30 crew, 150 Civilians, and 50 Leaders. 
    * Goals: build military, research, and city colony habitats. 

* Military Outpost Mission
    * Ship: Medium sized ship, with some combat and decent construction capabilities.
    * Personnel: 12 Crew, 60 Civilians, 10 Leaders.
    * Goals: Produce a self sustaining outpost with some military capacity.

* Refugee Mission
    * Ship: Medium sized ship, random capabilities from low to moderate.
    * Personnel: Random count of crew and civilians, possibly more then the ship can support.
    * Goals: Survive, prepare to welcome pursuers.

## ECS Entity Design. 

Some examples of different types of Entities and general ideas of what ECS components they would need.

### Planet/Moon/Asteroid

* Orbital: orbits sun(s) or other orbital entities
* Atmosphere: Optional as some may have none. 
* Geology: Mineral contents for exploitation as well as space on the surface and inside for 
* Biosphere (maybe zero)
 

   
### space station/ship/habitat

* Orbital
* Mechatronic
* Atmosphere (maybe zero)
* Biosphere (maybe zero, maybe should be handled as a different sim when in a controlled environment like hydroponics?)
* Personnel

### colony

* Mapping position (tied to specific maped entity)
* Atmosphere (may be closed or pulling from the satellite)
* Infrastructure
* Personnel

## leader

* Individual
* Leader

### faction

* has personnel
    
## Example of gameplay

### Survey Mission Story

The captain Vark'tal awakes from cryo-sleep and quickly grabs one of the convincingly placed bags. Closing and disposing of the bag, Kar pushes off the cryo-bed and floats towards the command chair tucking his tails close so as not to inadvertently hit anything. Buckling himself down in the command chair, Kar woke up systems and running checks. He had gone through this routine several times before during the journey to make sure everything was fine, but this time was different. They had arrived. After 143.4 years they had arrived at their target a red dwarf system with an estimated six planets. The ship had just crossed the system's heliosphere a month past.

Checking the hydroponics, Kar gave a satisfied smile at the blue growth with green fruit he saw. The automated seeding had been successful and the first crop was almost ready to harvest. Checking the radioactive system he noted they were running low but the solar arrays that had deployed a while ago would supplement it until the small crew could find a new source. Next Kar scanned the system looking for the planets and started gathering raw astrological data that they would need to study later. He was able to detect the asteroid field and set in a course. It will be a lonely few months since there was little reason to wake the others until they had reached the asteroids. But Kar would be far too busy to notice. 

Three months later the crew was assembled in the hanger. All six had recovered from the cryo-sleep and were ready to get to work. They were the advanced survey team and were to collect and process as much data as possible before the colony ship 10 light years behind them had reached the system. The captain assigned three crew members to take one of the shuttles and run a geological survey on a nearby asteroid cluster and extract some radioactive material if possible. The other three begin work on repairing the ship after the long journey. In the mean time the ships sensors would be collecting data and trying to pinpoint all the astrological bodies orbiting the sun.

By the time the survey team returned, the ship was back to tip top shape and the three crew members were busy studying the astrological data and doing basic chores around the ship. The survey team brought back some interesting metallic isotopes but no metals. Rotating out, a new group was sent out for another survey while the isotopes where studied in the ships lab. This time the survey team returned with some radioactive material and the crew began processing it to usable fuel.

Now that the ship was refueled the captain set a course for one of the system's gas giants. More research was done on the astrological data by the crew who had identified the orbits of five of the six planets and seven moons. The gas giant they were heading to had three of the identified moons. Once they arrived at the gas giant they began construction of several magnetic nets that would collect gases escaping the behemoth and a small storage station. They also made several trips to the nearby moons collecting geological data. One of the moons had some chlorine based flora which excited the botanist.

It has been two years since the crew arrived. Still set up around the gas giant, now named Dulras, the small storage station had been upgraded with material harvested from the moons into a medium sized station giving the crew more room along with a living area spun for gravity impoving their health and living conditions. The larger labs and manufacturing space had allowed the crew to advance their understanding of the system. The magnetic nets now covered a large area and the material was being processed into fuel. They were half way through building a planet lander. Once that once done they would head to the third planet which had water and significant amounts of flora. This planet was likely to become their home eventually.
