const DEV_QUEST_PARTS = `

===
Note: Almost all parts are dynamically created and can have any (legal) name.
      The None and Trash parts are hardcoded, you can override them if you must.
      Order and appearance is arbitrary although quest nodes do require all referred parts to be defined.
===

# Story_Dev
- title: dev
Author: Peter van der Zee
Desc: Just dev mode stuff


# Part_None
Can appear anywhere, or nowhere, because there's no properties to this one.
This is a transparent placeholder part for when its container does not have/need any part.
It may not even be painted, even if it could be.

# Part_Trash
Hardcoded node. Can appear anywhere. This is the trash/crap/whatever part. 
This is something that looks like a grey rock.
- char: t
- file: ./img/roguelikeitems.png
- x: 176
- y: 160
- w: 16
- h: 16

# Part_BlueWand
This is the slightly bigger blue wand
- char: b
- file: ./img/roguelikeitems.png
- x: 32
- y: 176
- w: 16
- h: 16

# Part_GoldDust
Kinda like gold dust?
- char: d
- file: ./img/roguelikeitems.png
- x: 128
- y: 48
- w: 16
- h: 16

# Part_GoldenBlueWand
This is the golden blue wand
- char: g
- file: ./img/roguelikeitems.png
- x: 64
- y: 176
- w: 16
- h: 16

# Part_Sapphire
This is a sapphire
- char: s
- file: ./img/roguelikeitems.png
- x: 16
- y: 48
- w: 16
- h: 16

# Part_WoodenStick
This is a club? Piece of wood I guess? From which wands are formed.
- char: w
- file: ./img/roguelikeitems.png
- x: 0
- y: 176
- w: 16
- h: 16

# Part_Paper
The clean paper
- char: p
- file: ./img/roguelikeitems.png
- x: 192
- y: 160
- w: 16
- h: 16

# Part_Parchment
The old paper
- char: q
- file: ./img/roguelikeitems.png
- x: 144
- y: 64
- w: 16
- h: 16

# Part_DirtWhite
- char: n
- file: ./img/roguelikeitems.png
- x: 32
- y: 64
- w: 16
- h: 16

# Part_DirtTurquoise
- char: h
- file: ./img/roguelikeitems.png
- x: 80
- y: 64
- w: 16
- h: 16

# Part_DirtBlue
- char: r
- file: ./img/roguelikeitems.png
- x: 64
- y: 64
- w: 16
- h: 16

# Part_Rope
- char: i
- file: ./img/roguelikeitems.png
- x: 48
- y: 96
- w: 16
- h: 16

# Part_Ruby
- char: j
- file: ./img/roguelikeitems.png
- x: 48
- y: 48
- w: 16
- h: 16

# Part_Wood
- char: k
- file: ./img/roguelikeitems.png
- x: 32
- y: 96
- w: 16
- h: 16

# Part_ShieldWood
- char: l
- pattern: _   Q   _   k   k   k   k   k   k
- file: ./img/roguelikeitems.png
- x: 80 
- y: 176
- w: 16 
- h: 16

# Part_ShieldBlue
- char: o
- pattern: _   _   _   W   l   W   _  _  _
- file: ./img/roguelikeitems.png
- x: 144
- y: 176
- w: 16
- h: 16

# Part_BookGreen
- char: A
- file: ./img/roguelikeitems.png
- x: 112
- y: 192
- w: 16
- h: 16

# Part_BookRed
- char: B
- file: ./img/parts/book_red.png
- x: 0
- y: 0
- w: 160
- h: 160

# Part_BookBlue
- char: C
- pattern: .   W   .   W   D   W   .   W   .
- file: ./img/roguelikeitems.png
- x: 144
- y: 192
- w: 16
- h: 16

# Part_BookWhite
- char: D
- pattern: i   p   p   i   p   p   i   p   p
- file: ./img/roguelikeitems.png
- x: 160
- y: 192
- w: 16
- h: 16

# Part_BookBrown
- char: E
- file: ./img/roguelikeitems.png
- x: 176
- y: 192
- w: 16
- h: 16

# Part_BookHeart
- char: F
- file: ./img/roguelikeitems.png
- x: 192
- y: 192
- w: 16
- h: 16

# Part_BookPurple
- char: G
- file: ./img/roguelikeitems.png
- x: 112
- y: 208
- w: 16
- h: 16

# Part_BookYellow
- char: H
- file: ./img/roguelikeitems.png
- x: 128
- y: 208
- w: 16
- h: 16

# Part_BookBlack
- char: I
- file: ./img/roguelikeitems.png
- x: 160
- y: 208
- w: 16
- h: 16

# Part_BookSkull
- char: J
- file: ./img/roguelikeitems.png
- x: 176
- y: 208
- w: 16
- h: 16

# Part_BookShield
- char: K
- pattern: .   .   .   C   o   .   .   .   .
- file: ./img/roguelikeitems.png
- x: 192
- y: 208
- w: 16
- h: 16

# Part_MaterialTurquoise
- char: m
- file: ./img/roguelikeitems.png
- x: 80
- y: 64
- w: 16
- h: 16

# Part_IngotBabyBlue
- char: L
- file: ./img/roguelikeitems.png
- x: 80
- y: 80
- w: 16
- h: 16

# Part_IngotGrey
- char: M
- file: ./img/parts/ingot_silver.png
- x: 0
- y: 0
- w: 160
- h: 160

# Part_IngotLawnGreen
- char: N
- file: ./img/roguelikeitems.png
- x: 48
- y: 80
- w: 16
- h: 16

# Part_IngotOrange
- char: O
- file: ./img/roguelikeitems.png
- x: 16
- y: 80
- w: 16
- h: 16

# Part_IngotTurquoise
- char: P
- file: ./img/roguelikeitems.png
- x: 80
- y: 80
- w: 16
- h: 16

# Part_IngotWhite
- char: Q
- pattern: .   .   .   n   n   n   n   n   n
- file: ./img/parts/ingot_silver.png
- x: 0
- y: 0
- w: 160
- h: 160

# Part_PotionWhite
- char: R
- file: ./img/roguelikeitems.png
- x: 112
- y: 64
- w: 16
- h: 16

# Part_PotionBlack
- char: S
- file: ./img/roguelikeitems.png
- x: 128
- y: 64
- w: 16
- h: 16

# Part_PotionPurple
- char: T
- file: ./img/roguelikeitems.png
- x: 144
- y: 64
- w: 16
- h: 16

# Part_PotionGreen
- char: U
- file: ./img/roguelikeitems.png
- x: 160
- y: 64
- w: 16
- h: 16

# Part_PotionRed
- char: V
- pattern: Part_Ruby Part_EmptyBottle
- file: ./img/parts/bottle_red.png
- x: 0
- y: 0
- w: 160
- h: 160

# Part_PotionBlue
- char: W
- pattern: .   r   .   .   e   .   .   r   .
- file: ./img/roguelikeitems.png
- x: 192
- y: 64
- w: 16
- h: 16

# Part_PotionBrown
- char: X
- file: ./img/roguelikeitems.png
- x: 112
- y: 80
- w: 16
- h: 16

# Part_PotionTurquoise
- char: Y
- file: ./img/roguelikeitems.png
- x: 128
- y: 80
- w: 16
- h: 16

# Part_PotionYellow
- char: Z
- file: ./img/roguelikeitems.png
- x: 144
- y: 80
- w: 16
- h: 16

# Part_PotionOrange
- char: x
- file: ./img/roguelikeitems.png
- x: 160
- y: 80
- w: 16
- h: 16

# Part_EmptyBottle
- char: e
- file: ./img/parts/bottle_empty.png
- x: 0
- y: 0
- w: 160
- h: 160

# Part_Gift
- pattern: Part_Thread Part_Paper Part_GoldCoins Part_BookShield
- file: ./img/bigset.png
- x: 168
- y: 216
- w: 24
- h: 24

# Part_SilverCoin
- pattern: Part_DirtWhite Part_DirtWhite
- file: ./img/parts/coin.png
- x: 0
- y: 0
- w: 160
- h: 160

# Part_SilverCoins
- pattern: Part_SilverCoin Part_SilverCoin Part_SilverCoin
- file: ./img/bigset.png
- x: 144
- y: 2232
- w: 24
- h: 24

# Part_GoldCoin
- pattern: Part_SilverCoins Part_GoldDust Part_DirtWhite
- file: ./img/bigset.png
- x: 0
- y: 2232
- w: 24
- h: 24

# Part_GoldCoins
- pattern: Part_GoldCoin Part_GoldCoin Part_GoldCoin
- file: ./img/bigset.png
- x: 48
- y: 2232
- w: 24
- h: 24

# Part_Cloth
- file: ./img/bigset.png
- x: 360
- y: 2376
- w: 24
- h: 24

# Part_WizardHat
- pattern: Part_Rope Part_Cloth
- file: ./img/parts/hat.png
- x: 0
- y: 0
- w: 160
- h: 160

# Part_SantaHat
- pattern: Part_PotionRed Part_Rope Part_WizardHat
Testing:
- pattern: Part_Wood Part_Thread Part_SilverCoin Part_SilverCoins Part_GoldCoin Part_Sapphire Part_ShieldWood Part_IngotWhite Part_BookWhite
- file: ./img/bigset.png
- x: 169
- y: 5688
- w: 24
- h: 24

# Part_RedGift
- pattern: Part_Gift Part_SantaHat Part_PotionRed Part_PotionGreen Part_Sled
- file: ./img/bigset.png
- x: 193
- y: 5688
- w: 24
- h: 24

# Part_Wool
- file: ./img/bigset.png
- x: 120
- y: 15072
- w: 24
- h: 24

# Part_Thread
- pattern: Part_Wool
- file: ./img/parts/thread.png
- x: 0
- y: 0
- w: 160
- h: 160

# Part_FishingRod
- pattern: Part_Wood Part_Thread
Temporarily 7 input parts for debugging
- pattern: Part_Wood Part_Thread Part_SilverCoin Part_SilverCoins Part_GoldCoin Part_Sapphire Part_ShieldWood Part_IngotWhite Part_BookWhite
- file: ./img/bigset.png
- x: 144
- y: 2448
- w: 24
- h: 24

# Part_Worm
- file: ./img/roguelikeitems.png
- x: 176
- y: 176
- w: 16
- h: 16

# Part_Sled
- pattern: Part_Wood
- file: ./img/sled.png
- x: 0
- y: 0
- w: 512
- h: 512

`;
