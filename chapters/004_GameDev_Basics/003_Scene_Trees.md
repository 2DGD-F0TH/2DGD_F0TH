{{pagebreak}}

Scene Trees
===========

:::::: {.epigraph author="Donald Knuth"}
Trees sprout up just about everywhere in computer science...
::::::

What is a scene
---------------

A scene usually represents a single screen in our game: it can be a menu or a single level. Many engines (like Godot) make use of this kind of abstraction to "break down" a game into more manageable pieces.

The problem is that single scene can contain tens if not even hundreds of elements, thus efficient management is necessary to avoid losing track of pieces of your game, as well as simplifying the drawing routines.

Scene trees and their functionalities
-------------------------------------

A scene tree is "yet another abstraction layer": pieces of your level are arranged in a parent-child relationship, which encourages [composition-based approaches](#composition) heavily, making the code more flexible and easier to maintain.

![How a scene tree looks (specifically in Godot)](./images/scene_trees/scene_tree_example.png){width=30%}

Each scene tree contains one or more "nodes" that represent a component of our level, like a sprite. These nodes can be grouped "logically" but scene trees can bring a lot more to the table.

### How scene trees can make drawing entities easier

Let's imagine a game like the famous Galaxian: we have a ship that shoots aliens, and sometimes aliens can react by "breaking formation" and attacking the player. Sometimes a single alien can break formation, sometimes it's a group of three.

![Example of a ship attack formation](./images/scene_trees/ship_attack_example.svg){width=40%}

The situation here is more complex than it seems: this "troop" has a "captain" leading two other ships, who are following at a fixed distance and angle: so if the captain moves, the "soldier ships" move, if the leader rotates, the "soldier ships" will rotate accordingly.

![What happens when the ship attack formation rotates](./images/scene_trees/ship_attack_rotation_example.svg){width=40%}

This can quickly get messy, since we'll have to rotate the leader according to the screen, then rotate the "soldier ships" according to the leaer first, and then to the screen.

Scene trees can be used to make things easier, each node will rotate in relation to its parent.

Implementing a scene tree
-------------------------

{{placeholder}}

<!-- TODO: A very simple scene tree implementation -->
