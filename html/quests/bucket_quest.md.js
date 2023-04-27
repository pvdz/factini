const CONFIG_BUCKET_QUEST = `

# Story_Buckets
- title: bucket coloring sotry
Author: Peter van der Zee
Desc: Simple example story
- active

===
  Raw materials
===

# Part_BucketRed
- file: ./img/parts/bucket_simple_red.png
- w: 32
- h: 32

# Part_BucketBlue
- file: ./img/parts/bucket_simple_blue.png
- w: 32
- h: 32

# Part_BucketYellow
- file: ./img/parts/bucket_simple_yellow.png
- w: 32
- h: 32

# Part_BucketBlack
- file: ./img/parts/bucket_simple_black.png
- w: 32
- h: 32

# Part_BucketWhite
- file: ./img/parts/bucket_simple_white.png
- w: 32
- h: 32


===
  Compositions
===


# Part_BucketGrey
- special: p 1
- pattern: Part_BucketWhite Part_BucketBlack
- file: ./img/parts/bucket_simple_grey.png
- w: 32
- h: 32

# Part_BucketGreen
- special: e 1
- pattern: Part_BucketBlue Part_BucketYellow
- file: ./img/parts/bucket_simple_green.png
- w: 32
- h: 32

# Part_BucketPurple
- special: v 1
- pattern: Part_BucketBlue Part_BucketRed
- file: ./img/parts/bucket_simple_purple.png
- w: 32
- h: 32

# Part_BucketOrange
- special: s 1
- pattern: Part_BucketYellow Part_BucketRed
- file: ./img/parts/bucket_simple_orange.png
- w: 32
- h: 32

# Part_BucketPink
- pattern: Part_BucketWhite Part_BucketRed
- file: ./img/parts/bucket_simple_pink.png
- w: 32
- h: 32

# Part_BucketRainbow
- pattern: Part_BucketWhite Part_BucketBlack Part_BucketRed Part_BucketBlue Part_BucketYellow Part_BucketGrey Part_BucketGreen Part_BucketPurple Part_BucketOrange
- file: ./img/parts/bucket_simple_rainbow.png
- w: 32
- h: 32


===
  Quests
===

# Quest_Green
- after:
- parts: Part_BucketYellow, Part_BucketBlue
- targets: 15x Part_BucketGreen

# Quest_Orange
- after: 
- parts: Part_BucketYellow, Part_BucketRed
- targets: 25x Part_BucketOrange

# Quest_Purple
- after: 
- parts: Part_BucketBlue, Part_BucketRed
- targets: 23x Part_BucketPurple

# Quest_Grey
- after: Quest_Green
- parts: Part_BucketWhite, Part_BucketBlack, Part_BucketGrey
- targets: 30x Part_BucketGrey

# Quest_Pink
- after: Quest_Orange
- parts: Part_BucketWhite, Part_BucketPink
- targets: 18x Part_BucketPink

# Quest_Rainbow
- after: Quest_Grey, Quest_Pink
- parts: Part_BucketRainbow Part_BucketWhite Part_BucketBlack Part_BucketRed Part_BucketBlue Part_BucketYellow Part_BucketGrey Part_BucketGreen Part_BucketPurple Part_BucketOrange Part_BucketPink
- targets: 50x Part_BucketRainbow


`;
