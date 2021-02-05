#General Idea
The player enters a solar system with some sort of starting ship or fleet and has free reign to develop the solar system, trade, or become a pirate. Build habitats, mine asteroids, skim gas giants, start planetary or orbital settlements, and more. The player has some long term soft goals like re-establishing contact with the home world and send out colony ships to new systems but these can be ignored. Conflicts are mostly environmental but other factions can also be an issue.

# Design Goals
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


# Game Scenario

Games are configured via a scenario made up of three parts.

* Starting Conditions: How the solar system is laid out.
* Starting Mission: What resources and soft goals the player is provided,
* Species: What your people are like including cultural stuff and technology advancement.

The Scenario can be configured and saved, but the Starting Conditions, Mission, and Species can also be individually configured and saved. So players can easily mix and match the 3 parts allowing, for example, several equipment load-outs to be used with the same system and species setups without having to repeat the settings.

## Starting Conditions
Settings that can fine tune the starting solar system, equipment, goals, ect. They can all be random, all specified, or anywhere in-between.

Solar System Configuration
* Star 
    * Count: Allowing for at least Binary and Trinary stars.
    * Type
* Number and type of planets
    * lots of variety available here, setting values or ranges for each planetary body.
    * Planet count range
    * Each planet
        * Type
        * Size
* Asteroid population
* Level of Life (ranging from existing sentient life or no life at all)
* Exotic Cases
    * Within a nebula

### Presets

Use some well known systems like Sol and Alpha Centauri, some challenging systems, and some exotic ones.

##Starting Mission
The starting mission determines what equipment, personnel, and soft goals the player starts with. Below are the different values a scenario can have.

### Goal 
Choose none or more of the following. Does not effect game mechanics, but provides players some guidelines to follow within the game.

* Survey: Collect a certain number of raw data points.
* Colonize: a category
    * Inhabit: 
        * Develop: A percentage. Develop this much of the solar system into habitable spaces.
        * Population: A number. Reach at minumum this many people.
    * Industrialize: Number. Gross production value.
    * Militarize: Number. Set up a military presence. 
* Repopulate: Send a certain number of colony ships to new systems.
* (check box for all the above) Send home: Uses goal and requires shipping the resource home.

### Starting Ship Configuration and Equipment
TODO Expand.

### Starting Personnel
Three counters one for each of the personnel types: crew, colonists, and officials.

Note: The difference between these three is explained later in this document.

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
    * Ship: Medium sized ship, with some combat and decent construction capabilities.
    * Personnel: 12 Crew, 60 Colonists, 10 Officials.
    * Goals: Produce a self sustaining outpost with some military capacity.

* Refugee Mission
    * Ship: Medium sized ship, random capabilities from low to moderate.
    * Personnel: Random count of crew and colonists, possibly more then the ship can support.
    * Goals: Survive, prepare to welcome perusers.

## Species
Choose lifeform type, species traits, and other visual aspects. Or choose from a number of presets.

### Physiology/Culture
Mental traits of the species effecting how they act during the game.

### Technological Advancement
Research and Technology is discussed in detail later. This option allows the player to choose their level of technology at the beginning of the game. This determines a number of things like was this a generation ship, did they use cryosleep, or was the trip more or less instantaneous. Choices here my limit other options and vise versa (example: Generation ship requires a minimum number of personnel and size of ship).

### Presets
Several variation of humans, a few classical and unique aliens as well.

# Mechanics

## Personnel
There are several types of personnel that the player will need to deal with.

* Crew: Under the player's direct control, crew members need to be kept happy, but will build, mine, research, and fight as needed.

* Colonists: These are people who hired the player to help them colonize the new solar system. The player completes various contracts that the colonists request and as payment they provide goods to the player. Crew can become colonists and colonists can be hired as crew.

* Officials: Government and commercial entities from politicians to military personnel. A portion of the colonists may become officials based on circumstances. Officials generally harass the player through regulations and complaints, but have more pull and pay better for contracts.

### Leaders
Since each category can get into the thousands or billions, there needs to be a way to interact with large groups of personal. This is done through Leaders. Each leader has a lot of detail associated to them and can appear or disappear randomly from all the personnel categories. Crew leaders are obviously yours to command while the other two may go against you or help you depending on the situation.

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
    * Driving goals


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


## Travel
TODO

## Factions

Factions are groups of personnel that form a like minded body. Every population usually has a mix of Factions. However, only one of the factions can be in control of things. Factions can control anything from individual ships and small settlements to entire fleets and planets. Player actions can affect reputation with a faction and factions can hire the player to do contracts. 

The player's crew is represented as an independent faction by default, but can merge or be subservient with another faction. NPC factions can merge, split, and become subservient to others under certain circumstances.

Technology levels are handled on a per faction basis and information can be traded. (See Science, Research, and Technology section). 


## Science, Research, and Technology
Raw Data is the representation of the work put into exploration missions. There are as many types of Raw Data as there are branches of science. Each Raw Data point collected is tagged with a location and time.

Research is the process of converting Raw Data points into Science points. Science points increase until the branch gains a level. How many Science points needed are random and Raw Data points can be waisted. Below is a table listing out the various branches of science.

| Branch | Description | Source |
TODO


Technologies are procedurally discovered based on science level progression. Each technology will either improve a process dramatically or allow new processes. New technologies can even open up new branches of science. Most Technologies are predefined and based on both common and some uncommon science fiction.

As Raw Data is collected, research done, and Technology discovered the Solar Codex will be filled with information specific to the solar system which the player can review. Location and time tags will be incorporated into the information. This is almost entirely story fluff to provide flavor.


## Missions and Contracts
The player designs missions and assigns crew who will complete them. Missions are long or short range tasks that gather raw data for science, collect resources, does construction, moves people and/or materials, and anything else that can be done in the game. These generally require one or more vehicles and other special equipment depending on what needs to be done. Missions can be set to repeating so routine resupply and trade missions can be created. Some missions can be fully automated, but most need to be maned.

**Contracts** are jobs that the player is hired to do by either colonists or officials. Payment is usually in goods or work. Contracts usually require multiple missions to finish. For example a colonist habitat might hire the player to expand the habitat to decrease overcrowding and in exchange give the player processed goods.


# Use Cases
## Survey Mission
The captain Vark'tal awakes from cryo-sleep and quickly grabs one of the convincingly placed bags. Closing and disposing of the bag, Kar pushes off the cryo-bed and floats towards the command chair tucking his tails close so as not to inadvertently hit anything. Buckling himself down in the command chair, Kar woke up systems and running checks. He had gone through this routine several times before during the journey to make sure everything was fine, but this time was different. They had arrived. After 143.4 years they had arrived at their target a red dwarf system with an estimated six planets. The ship had just crossed the system's heliosphere a month past.

Checking the hydroponics, Kar gave a satisfied smile at the blue growth with green fruit he saw. The automated seeding had been successful and the first crop was almost ready to harvest. Checking the radioactive system he noted they were running low but the solar arrays that had deployed a while ago would supplement it until the small crew could find a new source. Next Kar scanned the system looking for the planets and started gathering raw astrological data that they would need to study later. He was able to detect the asteroid field and set in a course. It will be a lonely few months since there was little reason to wake the others until they had reached the asteroids. But Kar would be far too busy to notice. 

Three months later the crew was assembled in the hanger. All six had recovered from the cryo-sleep and were ready to get to work. They were the advanced survey team and were to collect and process as much data as possible before the colony ship 10 light years behind them had reached the system. The captain assigned three crew members to take one of the shuttles and run a geological survey on a nearby asteroid cluster and extract some radioactive material if possible. The other three begin work on repairing the ship after the long journey. In the mean time the ships sensors would be collecting data and trying to pinpoint all the astrological bodies orbiting the sun.

By the time the survey team returned, the ship was back to tip top shape and the three crew members were busy studying the astrological data and doing basic chores around the ship. The survey team brought back some interesting metallic isotopes but no metals. Rotating out, a new group was sent out for another survey while the isotopes where studied in the ships lab. This time the survey team returned with some radioactive material and the crew began processing it to usable fuel.

Now that the ship was refueled the captain set a course for one of the system's gas giants. More research was done on the astrological data by the crew who had identified the orbits of five of the six planets and seven moons. The gas giant they were heading to had three of the identified moons. Once they arrived at the gas giant they began construction of several magnetic nets that would collect gases escaping the behemoth and a small storage station. They also made several trips to the nearby moons collecting geological data. One of the moons had some chlorine based flora which excited the botanist.

It has been two years since the crew arrived. Still set up around the gas giant, now named Dulras, the small storage station had been upgraded with material harvested from the moons into a medium sized station giving the crew more room along with a living area spun for gravity impoving their health and living conditions. The larger labs and manufacturing space had allowed the crew to advance their understanding of the system. The magnetic nets now covered a large area and the material was being processed into fuel. They were half way through building a planet lander. Once that once done they would head to the third planet which had water and significant amounts of flora. This planet was likely to become their home eventually.

