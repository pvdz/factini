const GAME_CONFIG_PRELUDE = `
This is a set of game config node.

## Dynamic declarative definition

The config files are dynamically loaded and define the game you are going to play. No need to recompile Rust to update the game looks or unlock tree. Everything is moddable.

Their syntax is bespoke and somewhat in the style of markdown with the goal if looking pretty as an .md file. Too bad these are .md.js files but I guess oh well.

## Syntax

Configs are cut in lines and each line is trimmed from spacing and then parsed as follows:

* If a line starts with a dash and a space and does not contain a recognized property then an error is thrown.
* A trimmed line that does not start with a hash+space or dash+space is ignored as being a comment.
* A new config node starts with a line with a single hash (#) followed by the full qualified name of the node, like \`# Asset_FirstThing\`, for an asset named FirstThing.
  * The part before the underscore is the category, this a fixed short hardcoded list and it will be an error for unknown category names.
  * The part after can be strict or freeform depending on the category. Generally speaking, Parts are fully freeform, Quests are fully freeform except the parts they refer to must be defined, other category names only allow a hardcoded list of names.
* Text that does not start with a single hashtag (#) or dash (-) is considered a comment. That is why this list is with a star ;)
* Lines starting with a single dash are modifiers / properties for the current open config node, like \`- file: ./img/food.png\`
  * The syntax is \`- {property_name}: {value}\`, where the property name has no spaces, the colon is mandatory, and the value is an integer most of the time, with certain exceptions
  * In theory all properties are optional and can appear in any order, although they always get scoped to the current open node or the current open frame of the animation of the current open node
  * Repeating the same property means the previous one gets replaced with the current one (unless a new node/frame scope was opened of course)
* Config node animations
  * Frame specific overrides like offset, size, and file, are all going to replace the last opened frame index. Frame names are irrelevant and frames cannot be referred to by index.
  * By default, each config node opens its first frame implicitly which will start with default values (like x=0, y=0, etc). You can open it explicitly but that is optional.
  * Subsequent frames will start by cloning the previous frame. This means it will copy the size and file etc of the previous frame so you don't have to repeat yourself.
  * If your node has only one frame, or defaults to apply to all frames, you can define them at the toplevel (and omit "frame"), or inside the frame section (which has the same effect).
* The "parser" is hand written and not very flexible. Sorry-not-sorry. 

## Known nodes

These are the known node kinds:

* Asset: hardcoded parts of the game, so you can skin them (update their looks/animation)
* Quest: the set of unlocks
* Part: the parts which you are to combine to fulfill quests
* Demand: the demander cell animation (where parts should end up), one for each orientation. Special kind of Asset.
* Supply: the supplier cell animation (where raw materials are given), one for each orientation. Special kind of Asset.
* Dock: the backdrop for the edge, one for each orientation. Special kind of Asset.
* Machine: machine cells. Only a handful are defined and currently only one is actually ever used at all.
* Belt: animation for belt cells.
  * The game maintains a hardcoded list of brute force variations for all port states of a belt cell
  * A port is the in/out state for one side of a belt and the four ports define the look of the belt.
  * The name of the belt is basically three segments: belt_in_out_unknown, and if a direction is not mentioned then it is "none".
  * The four directions are consistently used as "up" ("U"), "right" ("R"), "down" ("D"), "left" ("L"). When the "in" part is empty the underscore is not omitted. Only the second one is omitted if both the "out" and "unknown" parts are empty.
  * The directions are ordered alphabetically in the belt name for the sake of consistency
  * Example: Belt_D_RU -> this is a T-shaped belt, incoming from down, outgoing to the right and upward. Left has no connection. 

## Generic properties

* "drm": For anything where the animation is not owned. Can be used in conjunction with options.show_drm=false to create public share-able media with placeholders

## Quest properties

* "after": A list of zero or more quests that are required to unlock this quest
* "parts": A list of zero or more parts that unlock when this quest unlocks
* "targets": One or more pairs of counts and parts, the requirements to finish this quest
* "state": Only valid stats are "active", "finished", and "waiting". Generally you omit this.
* "active": No value. Mark the current story (whatever it is) as the active one. Will crash hard if more than one story is active.

## Part properties

* "pattern": If set then this part must be constructed in a machine with these input parts. If empty or omitted then it is a raw supplier part and cannot be created by a machine.
* "char": Ascii character that represents this part in the serialized map. Note that this is ascii bound. The parser does not do unicode. When omitted an available character is auto-assigned. Not all characters are available.
* "special": Marker that identifies this part as a special part for the maze. The property is to be followed by 'n', 'e', 's', 'p', or 'v' (special kind) and a digit (special level), like "- special: e 3". By default parts are "n 0"

## Animation definitions

Each node property starts with a dash and a space. Most update the current node or the current frame of the node.

Supported animation properties:

* "frame_offset": This defines which of the defined frames frame is considered the start of the animation. Offset zero.
* "frame_count": This defines how many frames the animation will have, even if fewer are defined. If fewer are defined, that many will be cloned from the last and appended, anyways.
* "frame_direction": Defines whether an animation goes forwards or backwards.
* "frame_delay": Expected animation frame delay in Factory ticks. Cannot be set per frame. 
* "looping": Does the animation repeat at all?
* "loop_delay": Pause at the end of each animation before starting the next loop, in Factory ticks.
* "loop_backwards": Basically defines whether the loop starts from the first or "reverses" the animation until the start, like a bounce. This must be either "true" or anything else to mean "false". Irrelevant of "looping" state, despite the name.
* "frame": This starts the definition of the next frame of the animation.
  * The text that follows "frame" will be its raw name, although it's not actually used.
  * The name is not an index and you cannot refer to specific frame indexes at all.
  * Note: The first time "frame" is used in a node it will only update the name of the current open frame
  * Note: Subsequent usage of "frame" opens the next frame which first fully clones the previous frame config
  * You can not refer to frames by their index

### Animation frame properties:

* "file": for animation frames. This defines the file to draw from for the current frame and future frames until overridden
* "part_x": See "x"
* "x": for animation frames. The x-coordinate of the sprite in the image
* "part_y": See "y"
* "y": for animation frames. The y-coordinate of the sprite in the image
* "part_w": See "w"
* "w": for animation frames. The width of the sprite in the image
* "part_h": See "h"
* "h": for animation frames. The height of the sprite in the image


`;
