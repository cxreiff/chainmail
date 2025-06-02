# Chainmail

_If You Do Not Send This Letter To Ten Recipients You Will Be Visited By The Wraith_

## Main Idea

### Theming
A game about delivering chain letters. Each level or challenge is introduced in
the form of a chain letter, in the language of a chain letter. For example, if
the chain letter is "Deliver This Letter To Ten Recipients Or You Will Be
Visited By The Wraith" the level will have ten targets that must be visited
while you are running away from a "wraith" entity.

### Mechanics
- Perhaps a game like starfox, 3D but constant movement down a linear corridor,
  with obstacles and power-ups.
- Each level has specific goals described in trickier and trickier language by
  the chain letters.
- Each level ends with a new chain letter to kick off the next level, but there
  might be other chain letters in the level that you can pick up to unlock
  "bonus" levels.
- If you take too long, you get chased by an unstoppable malevolent entity,
  sort of like the SkiFree yeti.

### Aesthetics
- Each letter will appear character-by-character or word-by-word like a typical
  character dialogue window, within a border that looks like a letter. Random
  sound effect per revealed character, like animal crossing or chulip.
- Some form of vector graphics? Perhaps using the Canvas and Line widgets from
  Ratatui.
- When taking damage from an obstacle, use Ratatui widgets and
  bevy_ratatui_camera's world-space conversion methods to create "particle
  effects" that are actually just characters drawn to the buffer.
- Stretch goal: when taking damage, can we create a "glitch" effect by actually
  messing with how the characters are drawn to the buffer for a few frames?

### Other
- Use other jam participants' usernames as senders, recipients, and/or quoted
  testimonials, i.e. "@example met their true love 9 days after forwarding!" or
  similar.

## Alternative Idea (tighter scope)

Could fall back to a 2D game where you are chasing targets (like Snake) which
are chain letters, but collecting each target causes ten more to appear. There
could be some penalty for leaving one uncollected for too long (the Wraith
appears?) so as more and more letters appear you will eventually be overwhelmed
and lose.
