# Town concept
Pets live in a 7x7 garden

┌───┬───┬───┬───┬───┬───┬───┐
│   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┤
│   │   │   │ H │   │   │   │
├───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │
├───┼───┼───┼───┼───┼───┼───┤
│   │   │   │   │   │   │   │
└───┴───┴───┴───┴───┴───┴───┘

Every day you should water a tile.
Whichever tile is the best (or latest if tied) watered will spawn a new pet.

## End of week
Every end of the week, there's an alert to view your report card.
If the pet sleeps without showing the report card,
reduce happiness by 1 for all pets,
then do the check.

In the report card:
You'll see total pomos, breaks, etc.
You'll see the watering results.

Check if any pets are older than their expected lifespan.
Kill off the oldest pet who is past their prime.
Show the user which one died.

Check if any pets are at 0 happiness.
If so, they die.

Now, if the user got at least 75% of their pomos compared to their expected...
And if the 20 pet limit is not currently met...
Sort all the plots based on water level.
Sub-sort them based on most recent watering.
At this best watered spot, spawn a new pet.

# What stats does a pet have?
Happiness: 0-16 4b
X: 0-7 3b
Y: 0-7 3b
Age (days): 0-255 8b

# What stats does the town have?
Hunger (calculated)
Hunger rate (calculated)
Expected pomos for the week: 0-255 8b
DateTime of last feeding u32
Hunger at last feeding u32?
Currently stored food: 0-65,535 u16
First day of week: 0-7 3b
Days left in tutorial mode: 0-16 4b
DateTime of last save u32
