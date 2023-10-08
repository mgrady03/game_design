# Far From Fatality

by People Preparing Bits

## Team Members
* Advanced Topic Subteam 1: Networked Multiplayer
	* Zi Han Ding
    * Anderis Dobrowolski
    * Cole Eichner

* Advanced Topic Subteam 2: Enemy AI
	* Jon Riklan
    * Mary Grady
    * Vansh Desai

* Advanced Topic Subteam 2: Room Generation
	* Tyler Liberatore
    * Jacob Kopco
    * David Platon

## Game Description

Far From Fatality is an action RPG with puzzle solving elements. 4 Heroes will work together in order
to solve puzzles and fight enemies in order to progress through the dungeon and defeat the boss. The
game will have a top-down tile based map, with each player and enemy being able to move unlocked from 
the general grid. The first level of the dungeon will be a tutorial to allow the player to learn and get 
used to the mechanics of the game.The players will then go through procedurally generated levels where puzzles
will be spaced throughout in order to progress. The final level of the dungeon will have a boss that the players must work together in order to defeat. The boss will have an adaptive AI that will learn from what the players have done throughout the dungeon in order to try to defeat them. The enemies throughout the dungeon will have more
simple AI in order to move and attack the players.

## Advanced Topic Description

### Networked Multiplayer

The networked multiplayer for the game comes in the form of the 4 player playstyle. The 4 players
will all be able to move independently of each other and this movement will be present on the
other players' screens. The players will also be able to solve the puzzles and those puzzles will
similarly be solved on the other players' side. The multiplayer will be done through LAN. 
    
### Enemy AI

Enemy AI will mainly focus on the implementation of the boss fight. Using utility AI, we will assign a point system that will calculate the systematically most efficient move that the enemy should make at any given instance when in combat with the player. Based on this point system, the enemy will decide its actions that will prove most dire for the player. This should make the boss fight extremely difficult for the player to beat without thinking like the enemy. In addition, we will track the player's combat decisions throughout the game. We can measure the average time the player takes between attacks, as well as its movement, to allow the enemy to know the most efficient time to attack and block the player. For smaller enemies, we will have a basic AI that follows the player around from point A to point B while making its basic default assigned attacks. Finally, the number of smaller enemies will be dependent on the number of active players. We will have N+1 smaller enemies in any given dungeon, where N is the number of active players. 
Four behaviors for boss include attack, block, avoid, and change target.

### Room Generation

The Room generation will take place in 2 forms. Each individual level will have a procedurally
generated layout which will then have generated rooms placed throughout. Puzzles will be 
designed so that they can be randomly placed throughout any room and level layouts. The
puzzles themselves will not be procedurally generated, it will just be randomized where specific
puzzles are placed. Using DFS to determine whether floor is navigable and if re-generation is necessary.

## Midterm Goals

* A Boss Level
* Playable but not complete boss fight
* Establish connections between players. Packets are sent and received between machines over LAN.

## Final Goals

* 25%: Fully navigable dungeon
	* Map/Tile Size: 16x16px tiles and 10x8 tiles for each screen and 14 total screens to make up the map
* 25%: Fully fleshed out boss fight; fully functional boss according to description above; functioning with environment and players
* 25%: 4 player cooperative multiplayer over LAN
* 5%: Each player has their own attack action:
	* 1 player has a fast, low damage sword (melee).
 	* 1 player has a slow, high damage axe (melee).
  	* 1 player has a fast, low damage bow (ranged).
  	* 1 player has a slow, high damage staff (ranged).

## Stretch Goals

* 4-way Player PvP
* Three different puzzle tilesets integrated into the room generation
