const DEV_QUEST_QUESTS = `

===
Note: Quest names are arbitrary and not hardcoded.
      The parts they refer must be defined in the config or otherwise runtime 
      errors are thrown when loading the config.
      The game will try to figure out which Quests are available by looking
      at the available parts and the required parts for construction.
===

# Story_Dev

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
- targets: 2x Part_PotionBlue

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
- targets: 10x Part_GoldCoin

# Quest_GoldCoins
- after: Quest_GoldCoin
- parts: Part_GoldCoins
- targets: 10x Part_GoldCoins

# Quest_Gift
- after: Quest_GoldCoins Quest_BookShield
- parts: Part_Gift
- targets: 10x Part_Gift

# Quest_PotionRed
- after: Quest_Start
- parts: Part_Ruby, Part_PotionRed
- targets: 10x Part_PotionRed

# Quest_WizardHat
- after: Quest_Start
- parts: Part_WizardHat Part_Cloth
- targets: 10x Part_WizardHat

# Quest_SantaHat
- after: Quest_PotionRed Quest_WhiteBook Quest_WizardHat
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

`;
