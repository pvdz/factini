const MAZE_STORY = `

# Story_Maze
- title: maze story line
Author: Peter van der Zee
Desc: Real game nao?

===
Raw materials
===

# Part_BatteryBlack
- file: ./img/parts/bucket_v3_black.png
- w: 32
- h: 32

# Part_BatteryOrange
- file: ./img/roguelikeitems.png
- drm
- x: 160
- y: 80
- w: 16
- h: 16

# Part_Energy1
- special: e 1
- pattern: Part_BatteryBlack Part_BatteryOrange
- file: ./img/battery.png
- drm
- w: 16
- h: 16


# Part_Speed1
- special: s 1
- pattern: Part_WheelAxeWood Part_Rims
- file: ./img/roguelikeitems.png
- drm
- x: 112
- y: 176
- w: 16
- h: 16

# Part_WheelAxeWood
- file: ./img/roguelikeitems.png
- drm
- x: 32
- y: 96
- w: 16
- h: 16

# Part_WheelAxeIron
- file: ./img/parts/ingot_silver.png
- x: 0
- y: 0
- w: 160
- h: 160


# Part_Volume1
- special: p 1
- pattern: Part_VolumeWool Part_VolumeGreen
- file: ./img/backpack.png
- drm
- w: 59
- h: 64

# Part_VolumeWool
- file: ./img/parts/thread.png
- x: 0
- y: 0
- w: 160
- h: 160

# Part_VolumeGreen
- file: ./img/roguelikeitems.png
- drm
- x: 160
- y: 64
- w: 16
- h: 16


# Part_Power1
- special: v 1
- pattern: Part_WheelAxeWood Part_WheelAxeIron
- file: ./img/roguelikeitems.png
- drm
- x: 80
- y: 96
- w: 16
- h: 16

# Part_Rims
- file: ./img/parts/coin.png
- x: 0
- y: 0
- w: 160
- h: 160



===
Compositions
===


===
Quests
===

# Quest_Energy
- after:
- targets: 10x Part_Energy1

# Quest_Speed
- after:
- targets: 10x Part_Speed1

# Quest_Volume
- after:
- targets: 10x Part_Volume1

# Quest_Power
- after:
- targets: 10x Part_Power1

`;
