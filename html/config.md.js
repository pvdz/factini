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
- file: ./img/roguelikeitems.png
- x: 128
- y: 192
- w: 16
- h: 16

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
- file: ./img/roguelikeitems.png
- x: 96
- y: 80
- w: 16
- h: 16

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
- file: ./img/roguelikeitems.png
- x: 32
- y: 80
- w: 16
- h: 16

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
- file: ./img/roguelikeitems.png
- x: 176
- y: 64
- w: 16
- h: 16

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
- file: ./img/roguelikeitems.png
- x: 176
- y: 80
- w: 16
- h: 16

# Part_Gift
Char is the Greek letter chi
- char: χ
- pattern: Part_Thread Part_Paper Part_GoldCoins Part_BookShield
- file: ./img/bigset.png
- x: 168
- y: 216
- w: 24
- h: 24

# Part_SilverCoin
Char is the Greek letter sigma
- char: ς
- pattern: Part_DirtWhite Part_DirtWhite
- file: ./img/bigset.png
- x: 96
- y: 2232
- w: 24
- h: 24

# Part_SilverCoins
Char is the Greek letter sigma (upper)
- char: Σ
- pattern: Part_SilverCoin Part_SilverCoin Part_SilverCoin
- file: ./img/bigset.png
- x: 144
- y: 2232
- w: 24
- h: 24

# Part_GoldCoin
Char is the Greek letter theta
- char: θ
- pattern: Part_SilverCoins Part_GoldDust
- file: ./img/bigset.png
- x: 0
- y: 2232
- w: 24
- h: 24

# Part_GoldCoins
Char is the Greek letter theta (upper)
- char: Θ
- pattern: Part_GoldCoin Part_GoldCoin Part_GoldCoin
- file: ./img/bigset.png
- x: 48
- y: 2232
- w: 24
- h: 24

# Part_Cloth
Char is the Greek letter xi (upper)
- char: ξ
- file: ./img/bigset.png
- x: 360
- y: 2376
- w: 24
- h: 24

# Part_GrayHat
Char is the Greek letter eta (upper)
- char: ε
- pattern: Part_Rope Part_Cloth
- file: ./img/bigset.png
- x: 48
- y: 10320
- w: 24
- h: 24

# Part_SantaHat
Char is the Greek letter eta (upper)
- char: Η
- pattern: Part_PotionRed Part_Rope Part_GrayHat
- file: ./img/bigset.png
- x: 169
- y: 5688
- w: 24
- h: 24

# Part_RedGift
Char is the Greek letter omega (upper)
- char: Ω
- pattern: Part_Gift Part_SantaHat Part_PotionRed Part_PotionGreen Part_Sled
- file: ./img/bigset.png
- x: 193
- y: 5688
- w: 24
- h: 24

# Part_Wool
Char is the Greek letter psi (upper)
- char: ψ
- file: ./img/bigset.png
- x: 120
- y: 15072
- w: 24
- h: 24

# Part_Thread
Char is the Greek letter tau (upper)
- char: Τ
- pattern: Part_Wool
- file: ./img/bigset.png
- x: 48
- y: 2448
- w: 24
- h: 24

# Part_FishingRod
Char is the Greek letter tau (upper)
- char: Τ
- pattern: Part_Wood Part_Thread
- file: ./img/bigset.png
- x: 144
- y: 2448
- w: 24
- h: 24

# Part_Worm
Char is the Greek letter zeta (upper)
- char: ζ
- file: ./img/roguelikeitems.png
- x: 176
- y: 176
- w: 16
- h: 16

# Part_Sled
Char is the Greek letter xi (upper)
- char: Ξ
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
- parts: Part_Sapphire, Part_PotionBlue
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

# Quest_GoldCoins
- after: Quest_SilverCoins
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
- file: ./img/supply.png
- x: 0
- y: 0
- w: 32
- h: 32

# Supply_Right
- file: ./img/supply.png
- x: 96
- y: 0
- w: 32
- h: 32

# Supply_Down
- file: ./img/supply.png
- x: 64
- y: 0
- w: 32
- h: 32

# Supply_Left
- file: ./img/supply.png
- x: 32
- y: 0
- w: 32
- h: 32

# Demand_Up
- file: ./img/demand.png
- x: 0
- y: 0
- w: 32
- h: 32

# Demand_Right
- file: ./img/demand.png
- x: 96
- y: 0
- w: 32
- h: 32

# Demand_Down
- file: ./img/demand.png
- x: 64
- y: 0
- w: 32
- h: 32

# Demand_Left
- file: ./img/demand.png
- x: 32
- y: 0
- w: 32
- h: 32

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
- file: ./img/belt/l_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__L
- file: ./img/belt/_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt___L
- file: ./img/belt/__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_
- file: ./img/belt/d_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL_
- file: ./img/belt/dl_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_L
- file: ./img/belt/d_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D__L
- file: ./img/belt/d__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__D
- file: ./img/belt/_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_D
- file: ./img/belt/l_d.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/r_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR_
- file: ./img/belt/lr_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_L
- file: ./img/belt/r_l.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/dlr_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR_L
- file: ./img/belt/dr_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR__L
- file: ./img/belt/dr__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_D
- file: ./img/belt/r_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR_D
- file: ./img/belt/lr_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_DL
- file: ./img/belt/r_dl.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_R
- file: ./img/belt/l_r.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/d_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL_R
- file: ./img/belt/dl_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_LR
- file: ./img/belt/d_lr.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/l_dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DLR
- file: ./img/belt/_dlr.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/u_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU_
- file: ./img/belt/lu_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_L
- file: ./img/belt/u_l.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/dlu_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU_L
- file: ./img/belt/du_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU__L
- file: ./img/belt/du__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_D
- file: ./img/belt/u_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU_D
- file: ./img/belt/lu_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_DL
- file: ./img/belt/u_dl.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/lru_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU_L
- file: ./img/belt/ru_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU__L
- file: ./img/belt/ru__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DRU_
- file: ./img/belt/dru_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLRU_
- file: ./img/belt/dlru_.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DRU_L
- file: ./img/belt/dru_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DRU__L
- file: ./img/belt/dru__l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU_D
- file: ./img/belt/ru_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LRU_D
- file: ./img/belt/lru_d.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU_DL
- file: ./img/belt/ru_dl.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/u_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU_R
- file: ./img/belt/lu_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_LR
- file: ./img/belt/u_lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_R_L
- file: ./img/belt/u_r_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU_R
- file: ./img/belt/du_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLU_R
- file: ./img/belt/dlu_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU_LR
- file: ./img/belt/du_lr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU_R_L
- file: ./img/belt/du_r_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_DR
- file: ./img/belt/u_dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU_DR
- file: ./img/belt/lu_dr.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_DLR
- file: ./img/belt/u_dlr.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_U
- file: ./img/belt/l_u.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/d_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL_U
- file: ./img/belt/dl_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_LU
- file: ./img/belt/d_lu.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/l_du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DLU
- file: ./img/belt/_dlu.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/r_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR_U
- file: ./img/belt/lr_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_LU
- file: ./img/belt/r_lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_U_L
- file: ./img/belt/r_u_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR_U
- file: ./img/belt/dr_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLR_U
- file: ./img/belt/dlr_u.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR_LU
- file: ./img/belt/dr_lu.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR_U_L
- file: ./img/belt/dr_u_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_DU
- file: ./img/belt/r_du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR_DU
- file: ./img/belt/lr_du.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_DLU
- file: ./img/belt/r_dlu.png
- x: 0
- y: 0
- w: 160
- h: 160

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
- file: ./img/belt/l_ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__LRU
- file: ./img/belt/_lru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__RU_L
- file: ./img/belt/_ru_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_RU
- file: ./img/belt/d_ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL_RU
- file: ./img/belt/dl_ru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_LRU
- file: ./img/belt/d_lru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_RU_L
- file: ./img/belt/d_ru_l.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DRU
- file: ./img/belt/_dru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_DRU
- file: ./img/belt/l_dru.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt__DLRU
- file: ./img/belt/_dlru.png
- x: 0
- y: 0
- w: 160
- h: 160

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
