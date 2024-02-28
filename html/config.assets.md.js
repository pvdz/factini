const GAME_CONFIG_ASSETS = `

===
Note: These asset names are hardcoded. Their order or even occurrence in this file do not matter. 
      They govern specific and  hardcoded parts of the game.
      Trying to use an Asset name that doesn't exist will lead to a runtime error while loading.
===

# Part_None
Can appear anywhere, or nowhere, because there's no properties to this one.
This is a transparent placeholder part for when its container does not have/need any part.
It may not even be painted, even if it could be.

# Part_Trash
Hardcoded node. Debug only. This definition can appear anywhere. This is the trash/crap/whatever part.
This is something that looks like a grey rock.
- char: t
- file: ./img/bag.png
- w: 512
- h: 512

# Asset_Logo
- file: ./img/logo75.png
- w: 210
- h: 75

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
- frame_delay: 200
- looping: true
- frame: 1
- file: ./img/woowie50.png
- x: 0
- y: 0
- w: 16
- h: 16

# Asset_MissingInputs
Indicator that a machine is missing an input belt
- frame_offset: 0
- frame_count: 2
- frame_direction: right
- frame_delay: 3000
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
- frame_delay: 3000
- looping: true
- frame: 1
- file: ./img/missing_outputs.png
- x: 0
- y: 0
- w: 50
- h: 50

# Asset_MissingPurpose
Indicator that a machine has no target part to create
Source of cog unclear: https://pixelartmaker.com/art/b1c49e7f345d87d
- drm
- frame_offset: 0
- frame_count: 2
- frame_direction: right
- frame_delay: 5000
- looping: true
- frame: 1
- file: ./img/missing_purpose.png
- x: 0
- y: 0
- w: 50
- h: 50

# Asset_Machine_1_2
- file: ./img/machines/machine_1_2.png
- w: 64
- h: 48
- file: ./img/machines/machine_orange.png
- w: 128
- h: 128

# Asset_Machine_2_1
- file: ./img/machines/machine_2_1.png
- w: 128
- h: 64
- file: ./img/machines/machine_orange.png
- w: 128
- h: 128

# Asset_Machine_2_2
- file: ./img/machines/machine_2_2.png
- w: 64
- h: 64
- file: ./img/machines/machine_orange.png
- w: 128
- h: 128

# Asset_Machine_3_3
- file: ./img/machines/machine_3_3.png
- w: 320
- h: 320
- file: ./img/machines/machine_orange.png
- w: 128
- h: 128

# Asset_Factory
- file: ./img/machines/machine_3_3.png
- w: 320
- h: 320

# Asset_DumpTruck
Public domain from https://opengameart.org/content/yellow-racing-car
- file: ./img/dumptruck.png
- x: 0
- y: 0
- w: 64
- h: 64

# Asset_Sand
- file: ./img/sand.png
- w: 128
- h: 128

# Asset_HelpBlack
Source: ikea.com via https://www.mentalfloss.com/article/58450/16-out-context-ikea-instructions-help-you-live-better-life
Used as parody
- file: ./img/help_black.png
- x: 0
- y: 0
- w: 50
- h: 41
# Asset_HelpWhite
Source: ikea.com via https://www.mentalfloss.com/article/58450/16-out-context-ikea-instructions-help-you-live-better-life
Used as parody
- file: ./img/help_white.png
- x: 0
- y: 0
- w: 50
- h: 41
# Asset_HelpGrey
Source: ikea.com via https://www.mentalfloss.com/article/58450/16-out-context-ikea-instructions-help-you-live-better-life
Used as parody
- file: ./img/help_grey.png
- x: 0
- y: 0
- w: 50
- h: 41
# Asset_HelpRed
Source: ikea.com via https://www.mentalfloss.com/article/58450/16-out-context-ikea-instructions-help-you-live-better-life
Used as parody
- file: ./img/help_red.png
- x: 0
- y: 0
- w: 50
- h: 41

# Asset_Manual
- file: ./img/manual.png
- w: 649
- h: 740

# Asset_Lmb
Source: https://www.flaticon.com/free-icon/mouse-left-button_32041
Free when with attribution
- file: ./img/lmb.png
- w: 50
- h: 50

# Asset_Rmb
Source: https://www.flaticon.com/free-icon/mouse-left-button_32041
Free when with attribution
- file: ./img/rmb.png
- w: 50
- h: 50

# Asset_SaveDark
- file: ./img/save_dark.png
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
- file: ./img/trash_dark_cb.png
- x: 0
- y: 0
- w: 136
- h: 136

# Asset_TrashLight
- frame_offset: 0
- frame_count: 1
- file: ./img/trash_light_cb.png
- x: 0
- y: 0
- w: 136
- h: 136

# Asset_TrashGrey
- frame_offset: 0
- frame_count: 1
- file: ./img/trash_grey_cb.png
- x: 0
- y: 0
- w: 136
- h: 136

# Asset_TrashRed
- frame_offset: 0
- frame_count: 1
- file: ./img/trash_red_cb.png
- x: 0
- y: 0
- w: 136
- h: 136

# Asset_TrashGreen
- frame_offset: 0
- frame_count: 1
- file: ./img/trash_green_cb.png
- x: 0
- y: 0
- w: 136
- h: 136

# Asset_BrushGrey
- frame_offset: 0
- frame_count: 1
- file: ./img/brush_grey_cb.png
- x: 0
- y: 0
- w: 128
- h: 128

# Asset_BrushDark
- frame_offset: 0
- frame_count: 1
- file: ./img/brush_dark_cb.png
- x: 0
- y: 0
- w: 128
- h: 128

# Asset_BrushLight
- frame_offset: 0
- frame_count: 1
- file: ./img/brush_light_cb.png
- x: 0
- y: 0
- w: 128
- h: 128

# Asset_BrushGreen
- frame_offset: 0
- frame_count: 1
- file: ./img/brush_green_cb.png
- x: 0
- y: 0
- w: 128
- h: 128

# Asset_BrushRed
- frame_offset: 0
- frame_count: 1
- file: ./img/brush_red_cb_x.png
- x: 0
- y: 0
- w: 128
- h: 128

# Asset_UndoLight
- frame_offset: 0
- frame_count: 1
- file: ./img/undo_light.png
- x: 0
- y: 0
- w: 128
- h: 128

# Asset_UndoGrey
- frame_offset: 0
- frame_count: 1
- file: ./img/undo_grey.png
- x: 0
- y: 0
- w: 128
- h: 128

# Asset_RedoLight
- frame_offset: 0
- frame_count: 1
- file: ./img/redo_light.png
- x: 0
- y: 0
- w: 128
- h: 128

# Asset_RedoGrey
- frame_offset: 0
- frame_count: 1
- file: ./img/redo_grey.png
- x: 0
- y: 0
- w: 128
- h: 128

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

# Asset_Pickaxe
- file: ./img/pickaxe.png
- x: 0
- y: 0
- w: 128
- h: 128

# Asset_Treasure
- file: ./img/treasure.png
- x: 0
- y: 0
- w: 160
- h: 160

# Asset_CopyWhite
- file: ./img/copy_white.png
- w: 48
- h: 48
# Asset_CopyGrey
- file: ./img/copy_grey.png
- w: 512
- h: 512
# Asset_CopyGreen
- file: ./img/copy_green.png
- w: 48
- h: 48

# Asset_PasteWhite
- file: ./img/download_white.png
- w: 512
- h: 512
# Asset_PasteGrey
- file: ./img/download_grey.png
- w: 512
- h: 512
# Asset_PasteGreen
- file: ./img/download_green.png
- w: 48
- h: 48

# Asset_DrmPlaceholder
See options.show_drm=false
- file: ./img/drm.png
- w: 16
- h: 16

# Asset_FullScreenBlack
- file: ./img/fullscreen_black.png
- w: 512
- h: 512

# Asset_FullScreenWhite
- file: ./img/fullscreen_white.png
- w: 512
- h: 512

# Asset_FullScreenGrey
- file: ./img/fullscreen_grey.png
- w: 512
- h: 512

# Asset_PlayBlack
- file: ./img/play_black.png
- w: 114
- h: 130
# Asset_PlayWhite
- file: ./img/play_white.png
- w: 114
- h: 130
# Asset_PlayGrey
- file: ./img/play_grey.png
- w: 114
- h: 130

# Asset_BwdBlack
- file: ./img/bwd_black.png
- w: 113
- h: 128
# Asset_BwdWhite
- file: ./img/bwd_white.png
- w: 113
- h: 128
# Asset_BwdGrey
- file: ./img/bwd_grey.png
- w: 113
- h: 128

# Asset_FastBwdBlack
- file: ./img/fast_bwd_black.png
- w: 183
- h: 128
# Asset_FastBwdWhite
- file: ./img/fast_bwd_white.png
- w: 183
- h: 128
# Asset_FastBwdGrey
- file: ./img/fast_bwd_grey.png
- w: 183
- h: 128

# Asset_FwdBlack
- file: ./img/fwd_black.png
- w: 113
- h: 128
# Asset_FwdWhite
- file: ./img/fwd_white.png
- w: 113
- h: 128
# Asset_FwdGrey
- file: ./img/fwd_grey.png
- w: 113
- h: 128

# Asset_FastFwdBlack
- file: ./img/fast_fwd_black.png
- w: 182
- h: 128
# Asset_FastFwdWhite
- file: ./img/fast_fwd_white.png
- w: 182
- h: 128
# Asset_FastFwdGrey
- file: ./img/fast_fwd_grey.png
- w: 182
- h: 128

# Asset_StopBlack
- file: ./img/stop_black.png
- w: 121
- h: 120
# Asset_StopWhite
- file: ./img/stop_white.png
- w: 121
- h: 120
# Asset_StopGrey
- file: ./img/stop_grey.png
- w: 121
- h: 120

# Asset_PauseBlack
- file: ./img/pause_black.png
- w: 119
- h: 120
# Asset_PauseWhite
- file: ./img/pause_white.png
- w: 119
- h: 120
# Asset_PauseGrey
- file: ./img/pause_grey.png
- w: 119
- h: 120

# Asset_Battery
- file: ./img/battery.png
- w: 70
- h: 128

`;
