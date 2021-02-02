#General Idea
The player enters a solar system with a single ship and has free reign to develop the solar system, trade, or become a pirate. Build habitats, mine asteroids, skim gas giants, start planetary or orbital settlements, and more. The player has some long term soft goals like re-establishing contact with the home world and send out colony ships to new systems but these can be ignored. 

# Design Goals
* Levels of interaction/Automation: Players can have very specific control over small groups or very general control over large groups (I.E. can play with a one person crew or 1000 person crew). They can choose what is automated by the game and what they handle to the point that the entire game can run on it's own (Player likes designing ships let him be able to just do that)

* Level Of Detail: Allow the player to look at most things in degrees of detail based on their interests. This means they can deal with only surface information about geology if they really don't care about that aspect but can have a lot of detail about biological life because they are interested in it.

* Variety of Game Mechanics separated into four categories: Discovery (Exploration and Research), Building (Design and Construction), Transport (Logistics and Economics), and Combat.

* Procedural: Provide organized randomness allowing for "endless" content that makes sense. 

* Not a space opera: Try to get realistic space physics (movement, heat) where possible while still allowing for somewhat outlandish future tech.

#Summery
* Genre: 4X and Construction Sandbox
* Inspiration: Roguelikes, vastness of space, project RO, Aroura 4X


# Starting Conditions
Settings that can fine tune the starting solar system, equipment, goals, ect. They can all be random, all specified, or anywhere in-between.

## Solar System Configuration
### Star type
### Number and type of planets
### Asteroid population
### Level of Life (existing sentient life or no life at all)
### Exotic Cases

* Within a nebula:
* Binary Star:

##Starting Mission
The starting mission determines what equipment, personnel, and soft goals the player starts with. Presets are available, listed in last sub-section, but those are made up of a number of variables.

### Goal 
Choose none or more of the following. Does not effect game mechanics, but provides players some guidelines to follow.

* Survey: Collect a certain number of raw data points.
* Colonize: a category
    * Inhabit: 
        * Develop: A percentage. Develop this much of the solar system into habitable spaces.
        * Population: A number. Reach at minumum this many people.
    * Industrialize:
    * Militarize: Number. Set up a military presence 
* Repopulate: Send a certain number of colony ships to new systems.

### Starting Ship Configuration and Equipment

### Starting Personnel
Three counters one for each of the personnel types: crew, colonists, and officials.

### Presets
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
    * Personnel: 12 Crew, 60 Colonists.
    * Goals: Initial goals build mining and industrial enabled colony habitats.

* State Colony Mission:
    * Ship: Large, mining and construction capabilities
    * Personnel: 30 crew, 150 colonists, and 50 officials. 
    * Goals: build military, research, and city colony habitats. 

* Military Outpost Mission
    * Ship: Medium sized ship
    * Personnel: 
    * Goals: 

* Refugee Mission
    * Ship: Medium sized ship
    * Personnel: 
    * Goals: 

## Race
Choose lifeform type, race traits, and other visual aspects. Or choose from a number of presets.

### Physiology
Mental traits of the race.

### Technological Advancement
Research and Technology is discussed in detail later. This option allows the player to choose their level of technology at the beginning of the game. This determines a number of things like was this a generation ship, did they use cryosleep, or was the trip more or less instantaneous. Choices here my limit other options and vise versa (example: Generation ship requires a minimum number of personnel and size of ship).

### Presets


# Mechanics
## Personnel
There are several types of personnel that the player will need to deal with.

* Crew: Under the player's direct control, crew members need to be kept happy, but will build, mine, research, and fight as needed.

* Colonists: These are people who hired the player to help them colonize the new solar system. The player completes various contracts that the colonists request and as payment they provide goods to the player. Crew can become colonists and colonists can be hired as crew.

* Officials: Government and commercial entities from politicians to military personnel. A portion of the colonists may become officials based on circumstances. Officials generally harass the player through regulations and complaints, but have more pull and pay better for contracts.

### Leaders
Since each category can get into the thousands or billions, there needs to be a way to interact with large groups of personal. This is done through Leaders. Each leader has a lot of detail associated to them and can appear or disappear from all the personnel categories.

Leader info as follows.

* Fluff (Very little gameplay effect other then depth)
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


###Individuals
If you want a higher LOD for personnel then a simplified version of the leader info is available. 

* Fluff
    * Full name
    * Homeworld
    * Likes
    * Beliefs
    * ect,
* Character
    * Specialization
    * Age


## Locations
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


## Travel

## Factions

Factions are groups of personnel that form a like minded body. Every population usually has a mix of Factions. However, only one of the factions can be in control of things. Factions can control anything from individual ships and small settlements to entire fleets and planets. Player actions can affect reputation with a faction and factions can hire the player to do contracts. 

The player's crew is represented as an independent faction by default, but can merge with another faction. NPC factions can merge under certain circumstances.

* 

Technology levels are handled on a per faction basis and information can be traded. (See Science, Research, and Technology section). 

## Raw Data, Research, and Technology
Raw Data is the representation of the work put into exploration missions. There are as many types of Raw Data as there are branches of science. Each Raw Data point collected is tagged with a location and time.

Research is the process of converting Raw Data points into Science points. Science points increase until the branch gains a level. How many Science points needed are random. Below is a table listing out the various branches of science.

| Branch | Description | Source |


Technologies are procedurally discovered based on science level progression. Each technology will either improve a process dramatically or allow new processes. New technologies can even open up new branches of science. Most Technologies are predefined and based on both common and some uncommon science fiction.

As Raw Data is collected, research done, and Technology discovered the Solar Codex will be filled with information specific to the solar system which the player can review. Location and time tags will be incorporated into the information. This is almost entirely story and provides flavor.


## Missions and Contracts
The player designs missions and assigns crew who will complete them. Missions are long or short range tasks that gather raw research data and resources, does construction, and moves people and materials. These generally require one or more vehicles and other special equipment depending on what needs to be done. Missions can be set to repeating so routine resupply missions can be created. Some missions can be fully automated but most need to be maned.

Contracts are jobs that the player is hired to do by either colonists or officials. Payment is usually in goods or work. Contracts usually require multiple missions to finish. For example a colonist habitat might higher the player to expand the habitat to decrease overcrowding and in exchange give the player processed goods.


# Use Cases
## Survey Mission
The captain Vark'tal awakes from cryo-sleep and quickly grabs one of the convincingly placed bags. Closing and disposing of the bag, Kar pushes off the cryo-bed and floats towards the command chair tucking his tails close so as not to inadvertently hit anything. Buckling himself down in the command chair, Kar woke up systems and running checks. He had gone through this routine several times before during the journey to make sure everything was fine, but this time was different. They had arrived. After 143.4 years they had arrived at their target a red dwarf system with an estimated six planets. The ship had just crossed the system's heliosphere a month past.

Checking the hydroponics, Kar gave a satisfied smile at the blue growth with green fruit he saw. The automated seeding had been successful and the first crop was almost ready to harvest. Checking the radioactive system he noted they were running low but the solar arrays that were unfurling would supplement it until the small crew could find a new source. Next Kar scanned the system looking for the planets and started gathering raw astrological data that they would need to study later. He was able to detect the asteroid field and set in a course. It will be a lonely few months since there was little reason to wake the others until they had reached the asteroid. But Kar would be far too busy to notice. 

Three months later the crew was assembled in the hanger. All six had recovered from the cryo-sleep and were ready to get to work. They were the advanced survey team and were to collect and process as much data as possible before the colony ship 10 light years behind them had reached the system. The captain assigned three crew members to take one of the shuttles and run a geological survay on a nearby asteroid cluster and extract some radioactive material if possible. The other three begin work on repairing the ship after the long journy. In the mean time the ships sensors would be collecting data and trying to pinpoint all the astrological bodies orbiting the sun.

By the time the survey team returned the ship was back to tip top shape and the three crew members were busy studying the astrological data and doing basic chores around the ship. The survey team brought back some interesting metallic isotopes but no metals. Rotating out, a new group was sent out for another survey while the isotopes where studied in the ships lab. This time the survey team returned with some radioactive material and the crew began processing it to usable fuel.

Now that the ship was refueled the captain set a course for one of the system's gas giants. More research was done on the astrological data by the crew who had identified the orbits of five of the six planets and seven moons. The gas giant they were heading to had three. Once they arrived at the gas gient they began construction of several magnetic nets that would collect gases as they escaped and a small storage station. They also made several trips to the nearby moons collecting geological data. One of the moons had some chlorine based flora which excited the botanist.

It has been two years since the crew arrived. Still set up around the gas giant, now named Dulras, the small storage station had been upgraded with material harvested from the moons into a medium sized station giving the crew more room along with a living area spun for gravity. The larger labs and manufacturing space had allowed the crew to advance their understanding of the system. The magnetic nets now covered a large area and the material was being processed into fuel. They were half way through building a planet lander. Once that was done they would head to the third planet which had water and significant amounts of flora. This planet was likely to become their home eventually.


# Implementation Design

