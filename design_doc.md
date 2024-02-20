# rusty

## short description

A dungeon crawler with procedurally generated levels,
monsters of increasing difficulty and turn-based movement.

## story

The hero's hometown is suffering from a plague of monsters.
Welling up from the deep, they seem unstoppable.
Legend tells of the Amulet of Yala - Yet Another Lost Amulet - that can be used to stem the tide.
After a long night at the tavern, the hero promises to save the day - and sets forth into the dungeon.

## basic game loop

1. enter dungeon level
2. explore, reveal the map
3. encounter enemies whom the player fights or flees from
4. find power-ups and use them to strengthen the player
5. locate the exit to the level - goto 1

## minimal viable product

1. create a basic dungeon map
2. place the player and let them walk around
3. spawn monsters, draw them and let the player kill them by walking into them
4. add health and a combat system that uses it
5. add healing potions
6. display a "game over" screen when the player dies
7. add the Amulet of Yala to the level and let the player win by reaching it

## stretch goals

1. add fields-of-view
2. add more interesting dungeon designs
3. add some dungeon themes
4. add multiple layers to the dungeon with the amulet on the last one
5. add varied weapons to te game
6. move to a data-driven design for spawning enemies
7. consider some visual effects to make combat mor visceral
8. consider keeping score
