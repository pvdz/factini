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
  - file: ./img/parts/bucket_v2_red.png
  - w: 32
  - h: 32
  
  # Part_BucketBlue
  - file: ./img/parts/bucket_simple_blue.png
  - file: ./img/parts/bucket_v2_blue.png
  - w: 32
  - h: 32
  
  # Part_BucketYellow
  - file: ./img/parts/bucket_simple_yellow.png
  - file: ./img/parts/bucket_v2_yellow.png
  - w: 32
  - h: 32
  
  # Part_BucketBlack
  - file: ./img/parts/bucket_simple_black.png
  - file: ./img/parts/bucket_v2_black.png
  - w: 32
  - h: 32
  
  # Part_BucketWhite
  - file: ./img/parts/bucket_simple_white.png
  - file: ./img/parts/bucket_v2_white.png
  - w: 32
  - h: 32

===
  Compositions
===

  ===
    Level 1.1
    Combine raw materials to unlock mixed materials (and black and white)
  ===

    # Part_BucketGrey
    - pattern: Part_BucketWhite Part_BucketBlack
    - file: ./img/parts/bucket_simple_grey.png
    - file: ./img/parts/bucket_v2_grey.png
    - w: 32
    - h: 32
    
    # Part_BucketGreen
    - special: e 1
    - pattern: Part_BucketBlue Part_BucketYellow
    - file: ./img/parts/bucket_simple_green.png
    - file: ./img/parts/bucket_v2_green.png
    - w: 32
    - h: 32
    
    # Part_BucketPurple
    - special: v 1
    - pattern: Part_BucketBlue Part_BucketRed
    - file: ./img/parts/bucket_simple_purple.png
    - file: ./img/parts/bucket_v2_purple.png
    - w: 32
    - h: 32
    
    # Part_BucketOrange
    - special: s 1
    - pattern: Part_BucketYellow Part_BucketRed
    - file: ./img/parts/bucket_simple_orange.png
    - file: ./img/parts/bucket_v2_orange.png
    - w: 32
    - h: 32
    
    # Part_BucketPink
    - special: p 1
    - pattern: Part_BucketWhite Part_BucketRed
    - file: ./img/parts/bucket_simple_pink.png
    - file: ./img/parts/bucket_v2_pink.png
    - w: 32
    - h: 32

  ===
    Level 1.2
    Combine all available colors for the rainbow bucket
  ===
    
    # Part_BucketRainbow
    - pattern: Part_BucketWhite Part_BucketBlack Part_BucketRed Part_BucketBlue Part_BucketYellow Part_BucketGrey Part_BucketGreen Part_BucketPurple Part_BucketOrange
    - file: ./img/parts/bucket_simple_rainbow.png
    - file: ./img/parts/bucket_v2_rainbow.png
    - w: 32
    - h: 32

  ===
    Level 1.3
    Convert rainbows to upgrades
  ===

    # Part_Plus
    - pattern: Part_BucketRainbow
    - file: ./img/parts/plus.png
    - w: 32
    - h: 32
    - file: ./img/parts/badge.v4.png
    - w: 64
    - h: 64
    
  ===
    Level 2.1
    Created 5 upgraded basic colors
  ===

    # Part_BucketRedPlus
    - pattern: Part_BucketRed Part_Plus
    - file: ./img/parts/bucket_simple_red_plus.png
    - file: ./img/parts/bucket_v2_red_plus.png
    - w: 32
    - h: 32
    
    # Part_BucketBluePlus
    - pattern: Part_BucketBlue Part_Plus
    - file: ./img/parts/bucket_simple_blue_plus.png
    - file: ./img/parts/bucket_v2_blue_plus.png
    - w: 32
    - h: 32
    
    # Part_BucketYellowPlus
    - pattern: Part_BucketYellow Part_Plus
    - file: ./img/parts/bucket_simple_yellow_plus.png
    - file: ./img/parts/bucket_v2_yellow_plus.png
    - w: 32
    - h: 32
    
    # Part_BucketBlackPlus
    - pattern: Part_BucketBlack Part_Plus
    - file: ./img/parts/bucket_simple_black_plus.png
    - file: ./img/parts/bucket_v2_black_plus.png
    - w: 32
    - h: 32
    
    # Part_BucketWhitePlus
    - pattern: Part_BucketWhite Part_Plus
    - file: ./img/parts/bucket_simple_white_plus.png
    - file: ./img/parts/bucket_v2_white_plus.png
    - w: 32
    - h: 32

  ===
    Level 2.2
    Combine upgraded basic colors into upgraded mixed colors
  ===

    # Part_BucketGreenPlus
    - special: e 2
    - pattern: Part_BucketBluePlus Part_BucketYellowPlus
    - file: ./img/parts/bucket_simple_green_plus.png
    - file: ./img/parts/bucket_v2_green_plus.png
    - w: 32
    - h: 32
    
    # Part_BucketGreyPlus
    - pattern: Part_BucketBlackPlus Part_BucketWhitePlus
    - file: ./img/parts/bucket_simple_grey_plus.png
    - file: ./img/parts/bucket_v2_grey_plus.png
    - w: 32
    - h: 32
    
    # Part_BucketPurplePlus
    - special: v 2
    - pattern: Part_BucketBluePlus Part_BucketRedPlus
    - file: ./img/parts/bucket_simple_purple_plus.png
    - file: ./img/parts/bucket_v2_purple_plus.png
    - w: 32
    - h: 32
    
    # Part_BucketOrangePlus
    - special: s 2
    - pattern: Part_BucketRedPlus Part_BucketYellowPlus
    - file: ./img/parts/bucket_simple_orange_plus.png
    - file: ./img/parts/bucket_v2_orange_plus.png
    - w: 32
    - h: 32
    
    # Part_BucketPinkPlus
    - special: p 2
    - pattern: Part_BucketRedPlus Part_BucketWhitePlus
    - file: ./img/parts/bucket_simple_pink_plus.png
    - file: ./img/parts/bucket_v2_pink_plus.png
    - w: 32
    - h: 32

  ===
    Level 3.1
    Combine mixed colors into double upgraded basic colors
  ===

    # Part_BucketRedPlusPlus
    - pattern: Part_BucketRedPlus Part_Plus
    - file: ./img/parts/bucket_simple_red_plusplus.png
    - file: ./img/parts/bucket_v2_red_plusplus.png
    - w: 32
    - h: 32
    
    # Part_BucketBluePlusPlus
    - pattern: Part_BucketBluePlus Part_Plus
    - file: ./img/parts/bucket_simple_blue_plusplus.png
    - file: ./img/parts/bucket_v2_blue_plusplus.png
    - w: 32
    - h: 32
    
    # Part_BucketYellowPlusPlus
    - pattern: Part_BucketYellowPlus Part_Plus
    - file: ./img/parts/bucket_simple_yellow_plusplus.png
    - file: ./img/parts/bucket_v2_yellow_plusplus.png
    - w: 32
    - h: 32
    
    # Part_BucketBlackPlusPlus
    - pattern: Part_BucketBlackPlus Part_Plus
    - file: ./img/parts/bucket_simple_black_plusplus.png
    - file: ./img/parts/bucket_v2_black_plusplus.png
    - w: 32
    - h: 32
    
    # Part_BucketWhitePlusPlus
    - pattern: Part_BucketWhitePlus Part_Plus
    - file: ./img/parts/bucket_simple_white_plusplus.png
    - file: ./img/parts/bucket_v2_white_plusplus.png
    - w: 32
    - h: 32

  ===
    Level 3.2
    Combined double upgraded basic colors into mixed double upgraded colors
  ===

    # Part_BucketGreyPlusPlus
    - pattern: Part_BucketBlackPlusPlus Part_BucketWhitePlusPlus
    - file: ./img/parts/bucket_simple_grey_plusplus.png
    - file: ./img/parts/bucket_v2_grey_plusplus.png
    - w: 32
    - h: 32
    
    # Part_BucketGreenPlusPlus
    - special: e 3
    - pattern: Part_BucketBluePlusPlus Part_BucketYellowPlusPlus
    - file: ./img/parts/bucket_simple_green_plusplus.png
    - file: ./img/parts/bucket_v2_green_plusplus.png
    - w: 32
    - h: 32
    
    # Part_BucketPurplePlusPlus
    - special: v 3
    - pattern: Part_BucketBluePlusPlus Part_BucketRedPlusPlus
    - file: ./img/parts/bucket_simple_purple_plusplus.png
    - file: ./img/parts/bucket_v2_purple_plusplus.png
    - w: 32
    - h: 32
    
    # Part_BucketOrangePlusPlus
    - special: s 3
    - pattern: Part_BucketRedPlusPlus Part_BucketYellowPlusPlus
    - file: ./img/parts/bucket_simple_orange_plusplus.png
    - file: ./img/parts/bucket_v2_orange_plusplus.png
    - w: 32
    - h: 32
    
    # Part_BucketPinkPlusPlus
    - special: p 3
    - pattern: Part_BucketRedPlusPlus Part_BucketWhitePlusPlus
    - file: ./img/parts/bucket_simple_pink_plusplus.png
    - file: ./img/parts/bucket_v2_pink_plusplus.png
    - w: 32
    - h: 32

===
  Quests
===

  ===
    Phase 1
    Create the mixed colors
  ===
  
    # Quest_Green
    - after:
    - targets: 150x Part_BucketGreen
    
    # Quest_Orange
    - after: 
    - targets: 25x Part_BucketOrange
    
    # Quest_Purple
    - after: 
    - targets: 23x Part_BucketPurple
    
    # Quest_Grey
    - after: Quest_Green Quest_Purple
    - targets: 30x Part_BucketGrey
    
    # Quest_Pink
    - after: Quest_Orange
    - targets: 18x Part_BucketPink
  
  ===
    Phase 2
    Create Rainbow and upgrade
  ===
      
    # Quest_Rainbow
    - after: Quest_Grey, Quest_Pink
    - targets: 50x Part_BucketRainbow
    
    # Quest_Plus
    - after: Quest_Rainbow
    - targets: 50x Part_Plus
    
  ===
    Phase 3
    Create upgraded basic colors
  ===
  
    # Quest_BlackPlus
    - after: Quest_Plus
    - targets: 30x Part_BucketBlackPlus
  
    # Quest_RedPlus
    - after: Quest_Plus
    - targets: 30x Part_BucketRedPlus
  
    # Quest_BluePlus
    - after: Quest_Plus
    - targets: 30x Part_BucketBluePlus
  
    # Quest_YellowPlus
    - after: Quest_Plus
    - targets: 30x Part_BucketYellowPlus
  
    # Quest_WhitePlus
    - after: Quest_Plus
    - targets: 30x Part_BucketWhitePlus
    
  ===
    Phase 4
    Create upgraded mixed colors
  ===
    
    # Quest_GreenPlus
    - after: Quest_BluePlus Quest_YellowPlus
    - targets: 10x Part_BucketGreenPlus
    
    # Quest_OrangePlus
    - after: Quest_RedPlus Quest_YellowPlus
    - targets: 25x Part_BucketOrangePlus
    
    # Quest_PurplePlus
    - after: Quest_RedPlus Quest_BluePlus 
    - targets: 23x Part_BucketPurplePlus
    
    # Quest_GreyPlus
    - after: Quest_WhitePlus Quest_BlackPlus
    - targets: 30x Part_BucketGreyPlus
    
    # Quest_PinkPlus
    - after: Quest_WhitePlus Quest_RedPlus
    - targets: 18x Part_BucketPinkPlus
    
  ===
    Phase 5
    Create double upgraded basic colors
  ===
  
    # Quest_BlackPlusPlus
    - after: Quest_GreenPlus Quest_OrangePlus Quest_PurplePlus Quest_GreyPlus Quest_PinkPlus
    - targets: 30x Part_BucketBlackPlusPlus
  
    # Quest_RedPlusPlus
    - after: Quest_GreenPlus Quest_OrangePlus Quest_PurplePlus Quest_GreyPlus Quest_PinkPlus
    - targets: 30x Part_BucketRedPlusPlus
  
    # Quest_BluePlusPlus
    - after: Quest_GreenPlus Quest_OrangePlus Quest_PurplePlus Quest_GreyPlus Quest_PinkPlus
    - targets: 30x Part_BucketBluePlusPlus
  
    # Quest_YellowPlusPlus
    - after: Quest_GreenPlus Quest_OrangePlus Quest_PurplePlus Quest_GreyPlus Quest_PinkPlus
    - targets: 30x Part_BucketYellowPlusPlus
  
    # Quest_WhitePlusPlus
    - after: Quest_GreenPlus Quest_OrangePlus Quest_PurplePlus Quest_GreyPlus Quest_PinkPlus
    - targets: 30x Part_BucketWhitePlusPlus
    
  ===
    Phase 6
    Create double upgraded mixed colors
  ===
    
    # Quest_GreenPlusPlus
    - after: Quest_BluePlusPlus Quest_YellowPlusPlus
    - targets: 10x Part_BucketGreenPlusPlus
    
    # Quest_OrangePlusPlus
    - after: Quest_RedPlusPlus Quest_YellowPlusPlus
    - targets: 25x Part_BucketOrangePlusPlus
    
    # Quest_PurplePlusPlus
    - after: Quest_RedPlusPlus Quest_BluePlusPlus 
    - targets: 23x Part_BucketPurplePlusPlus
    
    # Quest_GreyPlusPlus
    - after: Quest_WhitePlusPlus Quest_BlackPlusPlus
    - targets: 30x Part_BucketGreyPlusPlus
    
    # Quest_PinkPlusPlus
    - after: Quest_WhitePlusPlus Quest_RedPlusPlus
    - targets: 18x Part_BucketPinkPlusPlus
    
`;
