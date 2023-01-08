const GAME_CONFIG = `
# Part_None
Hardcoded first node internally but can appear anywhere, or nowhere, because there's no properties to this one

# Part_Trash
Hardcoded to be the second node internally but can appear anywhere. We can skin it here.
This is something that looks like a grey rock
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
Char is the Greek letter chi
- pattern: Part_Thread Part_Paper Part_GoldCoins Part_BookShield
- file: ./img/bigset.png
- x: 168
- y: 216
- w: 24
- h: 24

# Part_SilverCoin
Char is the Greek letter sigma
- pattern: Part_DirtWhite Part_DirtWhite
- file: ./img/parts/coin.png
- x: 0
- y: 0
- w: 160
- h: 160

# Part_SilverCoins
Char is the Greek letter sigma (upper)
- pattern: Part_SilverCoin Part_SilverCoin Part_SilverCoin
- file: ./img/bigset.png
- x: 144
- y: 2232
- w: 24
- h: 24

# Part_GoldCoin
Char is the Greek letter theta
- pattern: Part_SilverCoins Part_GoldDust
- file: ./img/bigset.png
- x: 0
- y: 2232
- w: 24
- h: 24

# Part_GoldCoins
Char is the Greek letter theta (upper)
- pattern: Part_GoldCoin Part_GoldCoin Part_GoldCoin
- file: ./img/bigset.png
- x: 48
- y: 2232
- w: 24
- h: 24

# Part_Cloth
Char is the Greek letter xi (upper)
- file: ./img/bigset.png
- x: 360
- y: 2376
- w: 24
- h: 24

# Part_GrayHat
Char is the Greek letter eta (upper)
- pattern: Part_Rope Part_Cloth
- file: ./img/bigset.png
- x: 48
- y: 10320
- w: 24
- h: 24

# Part_SantaHat
Char is the Greek letter eta (upper)
- pattern: Part_PotionRed Part_Rope Part_GrayHat
- file: ./img/bigset.png
- x: 169
- y: 5688
- w: 24
- h: 24

# Part_RedGift
Char is the Greek letter omega (upper)
- pattern: Part_Gift Part_SantaHat Part_PotionRed Part_PotionGreen Part_Sled
- file: ./img/bigset.png
- x: 193
- y: 5688
- w: 24
- h: 24

# Part_Wool
Char is the Greek letter psi (upper)
- file: ./img/bigset.png
- x: 120
- y: 15072
- w: 24
- h: 24

# Part_Thread
Char is the Greek letter tau (upper)
- pattern: Part_Wool
- file: ./img/parts/thread.png
- x: 0
- y: 0
- w: 160
- h: 160

# Part_FishingRod
Char is the Greek letter tau (upper)
- pattern: Part_Wood Part_Thread
- file: ./img/bigset.png
- x: 144
- y: 2448
- w: 24
- h: 24

# Part_Worm
Char is the Greek letter zeta (upper)
- file: ./img/roguelikeitems.png
- x: 176
- y: 176
- w: 16
- h: 16

# Part_Sled
Char is the Greek letter xi (upper)
- pattern: Part_Wood
- file: ./img/sled.png
- x: 0
- y: 0
- w: 512
- h: 512

# Quest_Start
- after: 
- parts: Part_DirtWhite, Part_IngotWhite
- targets: 10x Part_IngotWhite

# Quest_Shield
- after: Quest_Start
- parts: Part_Wood, Part_ShieldWood
- targets: 10x Part_ShieldWood

# Quest_BlueBottle
- after: Quest_Start
- parts: Part_Sapphire, Part_PotionBlue, Part_EmptyBottle
- targets: 10x Part_PotionBlue

# Quest_BlueShield
- after: Quest_Shield, Quest_BlueBottle
- parts: Part_ShieldBlue
- targets: 10x Part_ShieldBlue

# Quest_WhiteBook
- after: Quest_Start
- parts: Part_Rope, Part_Paper, Part_BookWhite
- targets: 10x Part_BookWhite

# Quest_BlueBook
- after: Quest_BlueBottle, Quest_WhiteBook
- parts: Part_BookBlue
- targets: 10x Part_BookBlue

# Quest_BookShield
- after: Quest_BlueBook, Quest_BlueShield
- parts: Part_BookShield
- targets: 10x Part_BookShield

# Quest_SilverCoin
- after:
- parts: Part_SilverCoin
- targets: 10x Part_SilverCoin

# Quest_SilverCoins
- after: Quest_SilverCoin
- parts: Part_SilverCoins
- targets: 10x Part_SilverCoins

# Quest_GoldCoin
- after: Quest_SilverCoins
- parts: Part_GoldCoin Part_GoldDust
- targets: 10x Part_GoldCoins

# Quest_GoldCoins
- after: Quest_GoldCoin
- parts: Part_GoldCoins Part_GoldDust
- targets: 10x Part_GoldCoins

# Quest_Gift
- after: Quest_GoldCoins Quest_BookShield
- parts: Part_Gift
- targets: 10x Part_Gift

# Quest_PotionRed
- after: Quest_Start
- parts: Part_Ruby, Part_PotionRed
- targets: 10x Part_PotionRed

# Quest_GreyHat
- after: Quest_Start
- parts: Part_GrayHat Part_Cloth
- targets: 10x Part_GrayHat

# Quest_SantaHat
- after: Quest_PotionRed Quest_WhiteBook Quest_GreyHat
- parts: Part_SantaHat
- targets: 10x Part_SantaHat

# Quest_Thread
- after:
- parts: Part_Wool Part_Thread
- targets: 10x Part_Thread

# Quest_FishingRod
- after: Quest_Start Quest_Thread
- parts: Part_FishingRod
- targets: 10x Part_FishingRod

# Quest_Sled
- after: Quest_FishingRod
- parts: Part_Sled Part_Worm
- targets: 10x Part_Sled

# Quest_Santa
- after: Quest_Gift Quest_SantaHat Quest_Sled
- parts: Part_PotionGreen Part_RedGift
- targets: 10x Part_RedGift






# Supply_Up
- frame_offset: 0
- frame_delay: 150
- loop_delay: 50
- frame: 1
  - file: ./img/supplier_up_anim_v2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Supply_Right
- frame_offset: 0
- frame_delay: 150
- loop_delay: 50
- frame: 1
  - file: ./img/supplier_right_anim_v2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Supply_Down
- frame_offset: 0
- frame_delay: 150
- loop_delay: 50
- frame: 1
  - file: ./img/supplier_down_anim_v2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Supply_Left
- frame_offset: 0
- frame_delay: 150
- loop_delay: 50
- frame: 1
  - file: ./img/supplier_left_anim_v2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224







# Demand_Up
- frame_offset: 0
- frame_delay: 100
- loop_delay: 50
- loop_backwards: true
- frame: 1
  - file: ./img/demander_anim_up.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Demand_Right
- frame_offset: 0
- frame_delay: 100
- loop_delay: 50
- loop_backwards: true
- frame: 1
  - file: ./img/demander_anim_right.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Demand_Down
- frame_offset: 0
- frame_delay: 100
- loop_backwards: true
- loop_delay: 50
- frame: 1
  - file: ./img/demander_anim_down.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Demand_Left
- frame_offset: 0
- frame_delay: 100
- loop_backwards: true
- loop_delay: 50
- frame: 1
  - file: ./img/demander_anim_left.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224






# Dock_Up
This is the edge area where suppliers and demanders can be placed
- file: ./img/dock.png
- x: 0
- y: 0
- w: 64
- h: 64

# Dock_Right
- file: ./img/dock.png
- x: 0
- y: 0
- w: 64
- h: 64

# Dock_Down
- file: ./img/dock.png
- x: 0
- y: 0
- w: 64
- h: 64

# Dock_Left
- file: ./img/dock.png
- x: 0
- y: 0
- w: 64
- h: 64




# Machine_3x3
- file: ./img/machine_1_1.png
- part_x: 5
- part_y: 5
- part_w: 20
- part_h: 20




# Belt_None
- file: ./img/belt/belt_none.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_Unknown
- file: ./img/belt/belt_unknown.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_Invalid
- file: ./img/belt/belt_invalid.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/l.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__L
- frame_offset: 6
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/_l.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt___L
- file: ./img/belt/__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/d.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DL_
- file: ./img/belt/dl_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_L
- frame_offset: 3
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/d_l2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_D__L
- file: ./img/belt/d__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__D
- frame_offset: 6
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/_d.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_L_D
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/l_d2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__DL
- file: ./img/belt/_dl.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__D_L
- file: ./img/belt/_d_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___D
- file: ./img/belt/__d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L__D
- file: ./img/belt/l__d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__L_D
- file: ./img/belt/_l_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___DL
- file: ./img/belt/__dl.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/r.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_LR_
- file: ./img/belt/lr_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_L
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/r_l.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_R__L
- file: ./img/belt/r__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR_
- file: ./img/belt/dr_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLR_
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dlr.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DR_L
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dr_l.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DR__L
- file: ./img/belt/dr__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_D
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/r_d2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_LR_D
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/lr_d.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_R_DL
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/r_dl.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_R_D_L
- file: ./img/belt/r_d_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R__D
- file: ./img/belt/r__d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR__D
- file: ./img/belt/lr__d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_L_D
- file: ./img/belt/r_l_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R__DL
- file: ./img/belt/r__dl.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__R
- frame_offset: 6
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/_r.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_L_R
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/l_r.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__LR
- file: ./img/belt/_lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__R_L
- file: ./img/belt/_r_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_R
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/d_r2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DL_R
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dl_r.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_D_LR
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/d_lr.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_D_R_L
- file: ./img/belt/d_r_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DR
- file: ./img/belt/_dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_DR
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/l_dr.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__DLR
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/_dlr.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__DR_L
- file: ./img/belt/_dr_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__R_D
- file: ./img/belt/_r_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_R_D
- file: ./img/belt/l_r_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__LR_D
- file: ./img/belt/_lr_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__R_DL
- file: ./img/belt/_r_dl.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___R
- file: ./img/belt/__r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L__R
- file: ./img/belt/l__r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__L_R
- file: ./img/belt/_l_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___LR
- file: ./img/belt/__lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D__R
- file: ./img/belt/d__r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL__R
- file: ./img/belt/dl__r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_L_R
- file: ./img/belt/d_l_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D__LR
- file: ./img/belt/d__lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__D_R
- file: ./img/belt/_d_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_D_R
- file: ./img/belt/l_d_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DL_R
- file: ./img/belt/_dl_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__D_LR
- file: ./img/belt/_d_lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___DR
- file: ./img/belt/__dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L__DR
- file: ./img/belt/l__dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__L_DR
- file: ./img/belt/_l_dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___DLR
- file: ./img/belt/__dlr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/u.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_LU_
- file: ./img/belt/lu_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_L
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/u_l2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_U__L
- file: ./img/belt/u__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU_
- file: ./img/belt/du_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLU_
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dlu.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DU_L
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/du_l.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DU__L
- file: ./img/belt/du__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_D
- frame_offset: 0
- frame_delay: 62
- frame: 0
  - file: ./img/belt/anim/u_d.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_LU_D
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/lu_d.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_U_DL
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/u_dl.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_U_D_L
- file: ./img/belt/u_d_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U__D
- file: ./img/belt/u__d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU__D
- file: ./img/belt/lu__d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_L_D
- file: ./img/belt/u_l_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U__DL
- file: ./img/belt/u__dl.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU_
- file: ./img/belt/ru_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LRU_
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/lru.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_RU_L
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/ru_l.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_RU__L
- file: ./img/belt/ru__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DRU_
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dru.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DLRU_
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dlru.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DRU_L
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dru_l.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DRU__L
- file: ./img/belt/dru__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU_D
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/ru_d.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_LRU_D
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/lru_d.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_RU_DL
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/ru_dl.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_RU_D_L
- file: ./img/belt/ru_d_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU__D
- file: ./img/belt/ru__d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LRU__D
- file: ./img/belt/lru__d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU_L_D
- file: ./img/belt/ru_l_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU__DL
- file: ./img/belt/ru__dl.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_R
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/u_r2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_LU_R
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/lu_r.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_U_LR
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/u_lr.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_U_R_L
- file: ./img/belt/u_r_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU_R
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/du_r.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DLU_R
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dlu_r.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DU_LR
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/du_lr.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DU_R_L
- file: ./img/belt/du_r_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_DR
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/u_dr.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_LU_DR
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/lu_dr.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_U_DLR
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/u_dlr.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_U_DR_L
- file: ./img/belt/u_dr_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_R_D
- file: ./img/belt/u_r_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU_R_D
- file: ./img/belt/lu_r_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_LR_D
- file: ./img/belt/u_lr_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_R_DL
- file: ./img/belt/u_r_dl.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U__R
- file: ./img/belt/u__r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU__R
- file: ./img/belt/lu__r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_L_R
- file: ./img/belt/u_l_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U__LR
- file: ./img/belt/u__lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU__R
- file: ./img/belt/du__r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLU__R
- file: ./img/belt/dlu__r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU_L_R
- file: ./img/belt/du_l_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU__LR
- file: ./img/belt/du__lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_D_R
- file: ./img/belt/u_d_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU_D_R
- file: ./img/belt/lu_d_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_DL_R
- file: ./img/belt/u_dl_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_D_LR
- file: ./img/belt/u_d_lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U__DR
- file: ./img/belt/u__dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU__DR
- file: ./img/belt/lu__dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_L_DR
- file: ./img/belt/u_l_dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U__DLR
- file: ./img/belt/u__dlr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__U
- frame_offset: 6
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/_u.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_L_U
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/l_u2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__LU
- file: ./img/belt/_lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__U_L
- file: ./img/belt/_u_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_U
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/d_u.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DL_U
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dl_u.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_D_LU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/d_lu.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_D_U_L
- file: ./img/belt/d_u_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DU
- file: ./img/belt/_du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_DU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/l_du.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__DLU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/_dlu.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__DU_L
- file: ./img/belt/_du_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__U_D
- file: ./img/belt/_u_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_U_D
- file: ./img/belt/l_u_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__LU_D
- file: ./img/belt/_lu_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__U_DL
- file: ./img/belt/_u_dl.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_U
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/r_u2.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_LR_U
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/lr_u.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_R_LU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/r_lu.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_R_U_L
- file: ./img/belt/r_u_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR_U
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dr_u.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DLR_U
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dlr_u.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DR_LU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dr_lu.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DR_U_L
- file: ./img/belt/dr_u_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_DU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/r_du.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_LR_DU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/lr_du.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_R_DLU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/r_dlu.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_R_DU_L
- file: ./img/belt/r_du_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_U_D
- file: ./img/belt/r_u_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR_U_D
- file: ./img/belt/lr_u_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_LU_D
- file: ./img/belt/r_lu_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_U_DL
- file: ./img/belt/r_u_dl.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__RU
- file: ./img/belt/_ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_RU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/l_ru.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__LRU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/_lru.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__RU_L
- file: ./img/belt/_ru_l.png
- x: 0
- y: 0 
- w: 160
- h: 160

# Belt_D_RU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/d_ru.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_DL_RU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/dl_ru.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_D_LRU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/d_lru.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt_D_RU_L
- file: ./img/belt/d_ru_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DRU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/_dru.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224
  
# Belt_L_DRU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/l_dru.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__DLRU
- frame_offset: 0
- frame_delay: 62
- frame: 1
  - file: ./img/belt/anim/_dlru.png
  - y: 0
  - x: 0
  - w: 32
  - h: 32
- frame: 2
  - x: 32
- frame: 3
  - x: 64
- frame: 4
  - x: 96
- frame: 5
  - x: 128
- frame: 6
  - x: 160
- frame: 7
  - x: 192
- frame: 8
  - x: 224

# Belt__DRU_L
- file: ./img/belt/_dru_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__RU_D
- file: ./img/belt/_ru_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_RU_D
- file: ./img/belt/l_ru_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__LRU_D
- file: ./img/belt/_lru_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__RU_DL
- file: ./img/belt/_ru_dl.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__U_R
- file: ./img/belt/_u_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_U_R
- file: ./img/belt/l_u_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__LU_R
- file: ./img/belt/_lu_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__U_LR
- file: ./img/belt/_u_lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_U_R
- file: ./img/belt/d_u_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL_U_R
- file: ./img/belt/dl_u_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_LU_R
- file: ./img/belt/d_lu_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_U_LR
- file: ./img/belt/d_u_lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DU_R
- file: ./img/belt/_du_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_DU_R
- file: ./img/belt/l_du_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DLU_R
- file: ./img/belt/_dlu_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DU_LR
- file: ./img/belt/_du_lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__U_DR
- file: ./img/belt/_u_dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_U_DR
- file: ./img/belt/l_u_dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__LU_DR
- file: ./img/belt/_lu_dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__U_DLR
- file: ./img/belt/_u_dlr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___U
- file: ./img/belt/__u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L__U
- file: ./img/belt/l__u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__L_U
- file: ./img/belt/_l_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___LU
- file: ./img/belt/__lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D__U
- file: ./img/belt/d__u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL__U
- file: ./img/belt/dl__u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_L_U
- file: ./img/belt/d_l_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D__LU
- file: ./img/belt/d__lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__D_U
- file: ./img/belt/_d_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_D_U
- file: ./img/belt/l_d_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DL_U
- file: ./img/belt/_dl_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__D_LU
- file: ./img/belt/_d_lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___DU
- file: ./img/belt/__du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L__DU
- file: ./img/belt/l__du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__L_DU
- file: ./img/belt/_l_du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___DLU
- file: ./img/belt/__dlu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R__U
- file: ./img/belt/r__u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR__U
- file: ./img/belt/lr__u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_L_U
- file: ./img/belt/r_l_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R__LU
- file: ./img/belt/r__lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR__U
- file: ./img/belt/dr__u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLR__U
- file: ./img/belt/dlr__u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR_L_U
- file: ./img/belt/dr_l_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR__LU
- file: ./img/belt/dr__lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_D_U
- file: ./img/belt/r_d_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR_D_U
- file: ./img/belt/lr_d_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_DL_U
- file: ./img/belt/r_dl_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_D_LU
- file: ./img/belt/r_d_lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R__DU
- file: ./img/belt/r__du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR__DU
- file: ./img/belt/lr__du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_L_DU
- file: ./img/belt/r_l_du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R__DLU
- file: ./img/belt/r__dlu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__R_U
- file: ./img/belt/_r_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_R_U
- file: ./img/belt/l_r_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__LR_U
- file: ./img/belt/_lr_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__R_LU
- file: ./img/belt/_r_lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_R_U
- file: ./img/belt/d_r_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL_R_U
- file: ./img/belt/dl_r_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_LR_U
- file: ./img/belt/d_lr_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_R_LU
- file: ./img/belt/d_r_lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DR_U
- file: ./img/belt/_dr_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_DR_U
- file: ./img/belt/l_dr_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DLR_U
- file: ./img/belt/_dlr_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DR_LU
- file: ./img/belt/_dr_lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__R_DU
- file: ./img/belt/_r_du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_R_DU
- file: ./img/belt/l_r_du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__LR_DU
- file: ./img/belt/_lr_du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__R_DLU
- file: ./img/belt/_r_dlu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___RU
- file: ./img/belt/__ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L__RU
- file: ./img/belt/l__ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__L_RU
- file: ./img/belt/_l_ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___LRU
- file: ./img/belt/__lru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D__RU
- file: ./img/belt/d__ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL__RU
- file: ./img/belt/dl__ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_L_RU
- file: ./img/belt/d_l_ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D__LRU
- file: ./img/belt/d__lru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__D_RU
- file: ./img/belt/_d_ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_D_RU
- file: ./img/belt/l_d_ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DL_RU
- file: ./img/belt/_dl_ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__D_LRU
- file: ./img/belt/_d_lru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___DRU
- file: ./img/belt/__dru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L__DRU
- file: ./img/belt/l__dru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__L_DRU
- file: ./img/belt/_l_dru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___DLRU
- file: ./img/belt/__dlru.png
- x: 0
- y: 0
- w: 160
- h: 160



`;
