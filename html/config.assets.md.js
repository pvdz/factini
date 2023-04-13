const GAME_CONFIG_ASSETS = `

===
Note: These asset names are hardcoded. Their order or even occurrence in this file do not matter. 
      They govern specific and  hardcoded parts of the game.
      Trying to use an Asset name that doesn't exist will lead to a runtime error while loading.
===


# Asset_ScreenLoader
This should be the cover screen. It should be at the top so it starts to load first.
  A placeholder is shown as long as the loader hasn't finished loading yet.
- file: ./img/screen_load.png
- w: 800
- h: 571

# Asset_ScreenMain
This should be the main menu screen.
- file: ./img/screen_main.png
- w: 800
- h: 571

# Asset_WeeWoo
This is an animated alarm (like the rotating light on a police car or ambulance)
- frame_offset: 0
- frame_count: 50
- frame_direction: right
- frame_delay: 80
- looping: true
- frame: 1
- file: ./img/weewoo.png
- x: 0
- y: 0
- w: 92
- h: 92

# Asset_MissingInputs
Indicator that a machine is missing an input belt
- frame_offset: 0
- frame_count: 2
- frame_direction: right
- frame_delay: 1500
- looping: true
- frame: 1
- file: ./img/missing_inputs.png
- x: 0
- y: 0
- w: 50
- h: 50

# Asset_MissingOutputs
Indicator that a machine is missing an input belt
- frame_offset: 0
- frame_count: 2
- frame_direction: right
- frame_delay: 1500
- looping: true
- frame: 1
- file: ./img/missing_outputs.png
- x: 0
- y: 0
- w: 50
- h: 50

# Asset_MissingPurpose
Indicator that a machine has no target part to create
- frame_offset: 0
- frame_count: 2
- frame_direction: right
- frame_delay: 2000
- looping: true
- frame: 1
- file: ./img/missing_purpose.png
- x: 0
- y: 0
- w: 50
- h: 50

# Asset_Machine1
- frame_offset: 0
- frame_count: 1
- file: ./img/machine1.png
- x: 0
- y: 0
- w: 64
- h: 64

# Asset_Machine2
- frame_offset: 0
- frame_count: 1
- file: ./img/machine2.png
- x: 0
- y: 0
- w: 64
- h: 64

# Asset_Machine3
- frame_offset: 0
- frame_count: 1
- file: ./img/machine3.png
- x: 0
- y: 0
- w: 64
- h: 64

# Asset_Machine4
- frame_offset: 0
- frame_count: 1
- file: ./img/machine4.png
- x: 0
- y: 0
- w: 64
- h: 64

# Asset_Machine_1_1
- frame_offset: 0
- frame_count: 1
- file: ./img/machine_1_1.png
- x: 0
- y: 0
- w: 320
- h: 320

# Asset_Machine_2_1
- frame_offset: 0
- frame_count: 1
- file: ./img/machine_2_2.png
- x: 0
- y: 0
- w: 128
- h: 64

# Asset_Machine_3_2
- frame_offset: 0
- frame_count: 1
- file: ./img/machine_3_2.png
- x: 0
- y: 0
- w: 192
- h: 128

# Asset_DumpTruck
- frame_offset: 0
- frame_count: 1
- file: ./img/dumptruck.png
- x: 0
- y: 0
- w: 64
- h: 64

# Asset_Sand
- frame_offset: 0
- frame_count: 1
- file: ./img/sand.png
- x: 0
- y: 0
- w: 128
- h: 128

# Asset_HelpBlack
- frame_offset: 0
- frame_count: 1
- file: ./img/help.png
- x: 0
- y: 0
- w: 50
- h: 41

# Asset_HelpRed
- frame_offset: 0
- frame_count: 1
- file: ./img/help_red.png
- x: 0
- y: 0
- w: 50
- h: 41

# Asset_Manual
- frame_offset: 0
- frame_count: 1
- file: ./img/manual.png
- x: 0
- y: 0
- w: 740
- h: 740

# Asset_Lmb
- frame_offset: 0
- frame_count: 1
- file: ./img/lmb.png
- x: 0
- y: 0
- w: 50
- h: 50

# Asset_Rmb
- frame_offset: 0
- frame_count: 1
- file: ./img/rmb.png
- x: 0
- y: 0
- w: 50
- h: 50

# Asset_SaveDark
- frame_offset: 0
- frame_count: 1
- file: ./img/save_dark.png
- x: 0
- y: 0
- w: 48
- h: 48

# Asset_SaveLight
- frame_offset: 0
- frame_count: 1
- file: ./img/save_light.png
- x: 0
- y: 0
- w: 48
- h: 48

# Asset_SaveGrey
- frame_offset: 0
- frame_count: 1
- file: ./img/save_grey.png
- x: 0
- y: 0
- w: 48
- h: 48

# Asset_TrashDark
- frame_offset: 0
- frame_count: 1
- file: ./img/trash_dark.png
- x: 0
- y: 0
- w: 43
- h: 43

# Asset_TrashLight
- frame_offset: 0
- frame_count: 1
- file: ./img/trash_light.png
- x: 0
- y: 0
- w: 43
- h: 43

# Asset_TrashGrey
- frame_offset: 0
- frame_count: 1
- file: ./img/trash_grey.png
- x: 0
- y: 0
- w: 43
- h: 43

# Asset_TrashRed
- frame_offset: 0
- frame_count: 1
- file: ./img/trash_red.png
- x: 0
- y: 0
- w: 43
- h: 43

# Asset_TrashGreen
- frame_offset: 0
- frame_count: 1
- file: ./img/trash_green.png
- x: 0
- y: 0
- w: 43
- h: 43

# Asset_QuestFrame
- frame_offset: 0
- frame_count: 1
- file: ./img/quest_frame.png
- x: 0
- y: 0
- w: 490
- h: 175

# Asset_DoubleArrowRight
- frame_offset: 0
- frame_count: 1
- file: ./img/double_arrow_right.png
- x: 0
- y: 0
- w: 13
- h: 38

# Asset_SingleArrowDown
- frame_offset: 0
- frame_count: 1
- file: ./img/single_arrow_down.png
- x: 0
- y: 0
- w: 18
- h: 13

# Asset_SingleArrowRight
- frame_offset: 0
- frame_count: 1
- file: ./img/single_arrow_right.png
- x: 0
- y: 0
- w: 13
- h: 18

# Asset_ButtonDown1
- file: ./img/button/down/button_down_1.png
- w: 65
- h: 55

# Asset_ButtonDown2
- file: ./img/button/down/button_down_2.png
- w: 1
- h: 55

# Asset_ButtonDown3
- file: ./img/button/down/button_down_3.png
- w: 65
- h: 55

# Asset_ButtonDown4
- file: ./img/button/down/button_down_4.png
- w: 70
- h: 1

# Asset_ButtonDown6
- file: ./img/button/down/button_down_6.png
- w: 70
- h: 1

# Asset_ButtonDown7
- file: ./img/button/down/button_down_7.png
- w: 75
- h: 60

# Asset_ButtonDown8
- file: ./img/button/down/button_down_8.png
- w: 1
- h: 60

# Asset_ButtonDown9
- file: ./img/button/down/button_down_9.png
- w: 75
- h: 60

# Asset_ButtonUp1
- file: ./img/button/up/button_up_1.png
- w: 70
- h: 50

# Asset_ButtonUp2
- file: ./img/button/up/button_up_2.png
- w: 1
- h: 50

# Asset_ButtonUp3
- file: ./img/button/up/button_up_3.png
- w: 70
- h: 50

# Asset_ButtonUp4
- file: ./img/button/up/button_up_4.png
- w: 70
- h: 1

# Asset_ButtonUp6
- file: ./img/button/up/button_up_6.png
- w: 70
- h: 1

# Asset_ButtonUp7
- file: ./img/button/up/button_up_7.png
- w: 70
- h: 55

# Asset_ButtonUp8
- file: ./img/button/up/button_up_8.png
- w: 1
- h: 55

# Asset_ButtonUp9
- file: ./img/button/up/button_up_9.png
- w: 70
- h: 55

`;
