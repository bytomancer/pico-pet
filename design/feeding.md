# Thoughts about feeding
I'm going to write these notes assuming we take the garden idea.

How does feeding work:
MAX_STOMACH = 1440 (minutes in a day)

Your weekly velocity determines what a feeding rewards you,
but for simplicity for now, we'll assume it's 400.

If the stomach is below 60, your pet starves in 1 hour.
This is the alert window.

When you feed, we need to make sure there is room for the feeding.
You cannot feed if the unit would be more than half wasted.
So, assuming 400 velocity,
you cannot feed when stomach is 1241 or higher.
The pet(s) shake their head.

Assuming you do feed,
add +velocity to the stomach,
clamp the max to MAX_STOMACH though.
Then write to our NVM/SaveState:
- current timestamp of this latest feeding, overwriting old values
- current stomach at time of last feeding, overwriting old values

You can always try to immediately re-feed by pressing accept again.

## What happens when you hit 0 stomach?
First, record 1 mistake.
Then, the pets eat enough food automatically to reach >= 50% stomach.
In the case of 400 velocity, they would eat 2,
because 800 stomach >= 720 which is 50%.
If there isn't enough food, you eat what you can.

## What happens when you hit 0 stomach and have 0 food in the pantry?
During the auto-eating, if there's 0 pantry food...
We record 1 additional mistake.
Then we refill the hunger to 100%.
Then we attempt to CANNABILIZE.

# Canabilize
Order every pet who is currently alive,
ordered by age,
then kill the oldest.

If there is only 1 pet, we destroy the village,
and the player's save is reset.
(Maybe cosmetics unlocked are kept?)...

# Watering
One additional thing I want to add is "watering".
Once per day, when you set your reminder,
you will receive your "watering" alert.
When this alert pings, nothing bad happens.

Once per day, you should water your garden.

If you sleep the garden without watering,
record 1 missed watering.

You can only "water" when you did your daily tasks.
It's up to you to write on a paper what your tasks are.  
Examples:
- brush teeth
- take pills
- drink a glass of water

## End of Week
Watering has these effects at the end of the week.
6-7 water: +1 population to the village, +2 growth to all members
4-5 water: +2 growth to all members
2-3 water: +1 growth to all members
0-1 water: no growth...

# End of Week Report
Every end of week, we do the following...
1. Retire a pet if required.
1. Kill a pet if required.
1. Introduce a new pet if possible.
1. Grow all the pets (even the new pet).
1. Check if we should adjust velocity.

## Retirement:
1. Go through every pet, and mark them as "retireable" if their age >= retirement age.
1. Order all "retireable" pets by their age.
1. Delete the oldest retirable pet.

## Killing
If the player has 3 or more mistakes, we kill a pet.
1. Order every pet by their age.
1. Delete the oldest pet.

## Growing
We should tune growing assuming pets spawn with growth = 2 due to the timing.
Every pet species has 4 stages (maybe fewer is okay too).
Every species has a different retirement age, but they average 20 probably.
(NOTE: 20 is based on village capacity, tuning it can be very important)

Example species, onion:
- Growth 0-6: shoots (week 1-3 if healthy)
- Growth 7-18: bulb (week 4-10 if healthy)
- Growth 19-20: budding (week 11 if healthy)
- Growth 20+: flowering (week 12+ if healthy)
- Retirement age: 18 weeks

A pet's growth caps at 32 (16 healthy weeks)!

Example onion growth rates if always healthy:
- week of birth: 2
- week 2: 4
- week 3: 6
- etc

## Adding to the population
If the garden does well,
the player chooses 1 plot to till in the end of week report,
and that spot immediately gets a new pet.

The pet that grows there is based on the 8 adjacent neighbors at the exact moment you till,
before the growth phase.

You cannot till an occupied area.

## Velocity adjustment
We need to track how many food the user gains every week.
There are 10,080 minutes in a week.
For some buffer, we'll just use 10,000.

If a user gets 25 food per week,
take 10,000 / 25,
then your velocity is 400.

Look at the current expected velocity,
then the actual velocity of this week.
If the new velocity is 1.25x the old velocity,
or if the player has more food banked than their velocity per week,
we OFFER a velocity increase.

If the player got only .75x of the old velocity,
we can OFFER a velocity decrease.

If the player accepts an increase,
immediately eat 1 new day worth of food too,
because it has a cost.

If the player accepts a decrease,
add 1 new day to the pantry,
because they are probably scraping by.

# Cosmetics
You are awarded 2 ultimate currency at the end of the week if:
- 6-7 days of watering.
- 0 mistakes.

You can buy a cosmetic in the cosmetic shop with this currency.

There are cosmetics for the garden (background),
and cosmetics you can put on pets (hats).
