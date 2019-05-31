\null\clearpage
Collision Detection and Reaction
=================================

Finding out who hit what
------------------------

### The Brute Force Method

<!-- TODO: Runs at O(n^2) and is simple to implement, but can get really slow when
there are many items on screen to check -->

### Building Quad Trees

<!-- TODO: Easier on the CPU but harder to implement, every frame you build a quad tree
and use that to check on collisions -->

Collision Reaction
--------------------

### The Direction + Velocity Method
<!-- TODO: A-la mario 1, you get inside a block, and react according to where the character is going-->

### The "Before and After" Method
<!-- TODO: Snapshot before and after updating, react accordingly, allows for more advanced stuff -->

Some wilder stuff
-------------------

### The "Tile + Offset" Method
<!-- TODO: Useful for games like pacman, check the direction where you are going using the offset, if the next cell is a wall, react -->

Common Issues with Collision Detection
----------------------------------------

### The "Bullet Through Paper" problem

<!-- TODO: How a really small object at fast speeds can go through a thin wall without
the collision detection algorithm realizing it -->
