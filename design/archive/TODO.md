# TODO

This list is not final. It will grow and evolve.

Done items should be removed from the list.

- Misc
  - When adjusting date/time, there needs to be adjustments made to the feeding schedule
  - Warn users when adjusting date/time that feeding schedule may be affected (maybe unnecessary?)
  - Idle setting, how many minutes
  - Store the `MainScene::menu_item_selected` as a static/global for reloading the scene
  - Handle AM/PM time adjustment in settings scene
- Intro Scene
  - Introduce pet
  - Explain the pomodoro method
  - Explain how feeding works
  - Force the player to save each of the setting components
    - Possibly describe how they each work (why must we have a clock?)
  - Finalize by saving to NVM and pet greeting scene
  - Remove the current code which checks for invalid NVM then autowrites
- Stats Scene
  - Displays information about the pet:
    - Birthday (requires intro scene)
    - Age (should this go up if date changes?)
    - Current daily hunger
    - Illness if applicable
- Eating Scene
  - Opening the scene after eating today says the pet cannot eat again.
  - Upon opening the eating scene present pet's hunger
    - With no feed history (fresh start), pet hungers for 1 tomato
  - Allow the player to select how many tomatoes to feed pet
    - Pet can be fed up to its hunger
  - Inform player that all tomatoes not fed will be juiced automatically
  - Confirm button
    - Write the day to NVM
      - Pet's hunger is adjusted
      - Pet's health is adjusted
      - Tomatoes fed removed, remaining juiced
- Shop Scene
  - Spend raspberries for cosmetics
  - Cosmetics are locked behind "level" (HP)
- Pet maintenance
  - Introduce check, runs at feeding deadline
    - Must have save data for last time a day was written to NVM
- Pet rework
  - Switch from Ferris to a custom designed pet
  - Switch from lofi girl to custom animation(s)


## General Ideas
Just ideas, not committed to them
- Start
  - Pet starts the game with 5 HP.
  - Pet starts the game with 5 Max HP.
  - Pet starts the game with a renewing hunger of 1.
- Feeding
  - Entering the feeding scene, player can commit to feeding.
  - All tomatoes up to the hunger are consumed.
  - If hunger remains, pet consumes 6 juice per tomato of hunger missing.
  - Every leftover tomato becomes 5 juice.
  - Feeding must be done by the configured "eating deadline"; and a date is saved from teh last feeding.
  - Failing to eat enough causes the pet to lose 1 HP per tomato of hunger remaining.
  - Eating enough tomatoes with no juice consumed regains 1 HP.
- Pet leveling
  - Occurs during feeding, when a tomato is fed.
  - Pet will monitor the amount of juice the player has accumulated.
  - If the player has enough juice to let the pet's hunger be satisfied for 5 consecutive feedings and the pet's HP is at maximum:
    - Pet consumes +1 tomato during the feeding.
    - Pet's renewing hunger goes up by 1.
    - Pet's Max HP and HP both raise by 1.
  - If the player has not fed the pet enough and the HP drops to 0:
    - Pet is now "sick".
    - Pet's hunger immediately drops by 1 (min 1).
    - Pet's max HP drops by 1 (min 5).
    - Pet's current HP increases to 3.
    - A sick pet can only consume juice.
    - All tomatoes accumulated are juiced instead of fed whole.
    - Each 6 juice consumed regains 1 HP.
    - The player has the option to feed a sick pet a single raspberry, restoring 1 additional HP.
    - Sickness ends when the pet's HP is restored to full.
  - Need a plateau effect somehow, which detects when players have reached their healthy sustainable limit.
    - In the plateau period, pet can somehow adjust consumption to prevent leveling up.
    - Alternatively, have the level up require a consistent overproduction (a trial period).
    - Alternatively, have the user select if they would like to level up.
- Hiatus
  - A player needs the option to rest for vacations and other long breaks.
  - However, abusing the break system is not encouraged.
  - Perhaps entering the break costs 1 HP.
  - Perhaps the player must spend 1 raspberry to enter a hiatus.
  - During hiatus, the player cannot score tomatoes, juice, or raspberries.
  - The pomodoro setting could still function though, just with a warning.
