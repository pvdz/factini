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


# Belt_D_U
- file: ./img/d_u_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_D
- file: ./img/u_d_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU
- file: ./img/du_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_R
- file: ./img/l_r_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_L
- file: ./img/r_l_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR
- file: ./img/lr_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_U
- file: ./img/l_u_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_L
- file: ./img/u_l_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU
- file: ./img/lu_cb2.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_U
- file: ./img/r_u_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_R
- file: ./img/u_r_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU
- file: ./img/ru_cb2.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_R
- file: ./img/d_r_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_D
- file: ./img/r_d_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR
- file: ./img/dr_cb2.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_L
- file: ./img/d_l_cb2.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_D
- file: ./img/l_d_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL
- file: ./img/dl_cb2.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU_R
- file: ./img/du_r_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR_U
- file: ./img/dr_u_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_RU
- file: ./img/d_ru_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU_D
- file: ./img/ru_d_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_DU
- file: ./img/r_du_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_DR
- file: ./img/u_dr_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DRU
- file: ./img/dru_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU_R
- file: ./img/lu_r.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR_U
- file: ./img/lr_u_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_RU
- file: ./img/l_ru_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU_L
- file: ./img/ru_l_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_LU
- file: ./img/r_lu_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_LR
- file: ./img/u_lr_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LRU
- file: ./img/lru_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL_R
- file: ./img/dl_r_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR_L
- file: ./img/dr_l_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_LR
- file: ./img/d_lr_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR_D
- file: ./img/lr_d_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_DL
- file: ./img/r_dl_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_DR
- file: ./img/l_dr_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLR
- file: ./img/dlr_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL_U
- file: ./img/dl_u_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU_L
- file: ./img/du_l_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_LU
- file: ./img/d_lu_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU_D
- file: ./img/lu_d_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_DL
- file: ./img/u_dl_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_DU
- file: ./img/l_du_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLU
- file: ./img/dlu_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLR_U
- file: ./img/dlr_u_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLU_R
- file: ./img/dlu_r_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DRU_L
- file: ./img/dru_l_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LRU_D
- file: ./img/lru_d_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DL_RU
- file: ./img/dl_ru_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DR_LU
- file: ./img/dr_lu_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DU_LR
- file: ./img/du_lr_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LR_DU
- file: ./img/lr_du_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_LU_DR
- file: ./img/lu_dr_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_RU_DL
- file: ./img/ru_dl_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_D_LRU
- file: ./img/d_lru_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_L_DRU
- file: ./img/l_dru_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_R_DLU
- file: ./img/r_dlu_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_U_DLR
- file: ./img/u_dlr_cb.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_DLRU
- file: ./img/dlru_cb1.png
- x: 0
- y: 0
- w: 160
- h: 160

# Belt_None
- file: ./img/belt_none.png
- x: 0
- y: 0
- w: 64
- h: 64

# Belt_Unknown
- file: ./img/belt_unknown.png
- x: 0
- y: 0
- w: 64
- h: 64

# Belt_Invalid
- file: ./img/belt_invalid.png
- x: 0
- y: 0
- w: 64
- h: 64

`;
