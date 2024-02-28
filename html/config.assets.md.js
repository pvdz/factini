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

# Asset_ScreenPlay
This is the "play" part for the main loading screen. We paint it over the "loading" part.
- file: ./img/screen_play.png
- w: 138
- h: 61

# Asset_SupplyUp
- frame_offset: 0
- frame_delay: 150
- loop_delay: 50
- looping: false
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

# Asset_SupplyRight
- frame_offset: 0
- frame_delay: 150
- loop_delay: 50
- looping: false
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

# Asset_SupplyDown
- frame_offset: 0
- frame_delay: 150
- loop_delay: 50
- looping: false
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

# Asset_SupplyLeft
- frame_offset: 0
- frame_delay: 150
- loop_delay: 50
- looping: false
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





# Asset_DemandUp
- frame_offset: 0
- frame_delay: 100
- loop_delay: 50
- loop_backwards: true
- looping: false
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

# Asset_DemandRight
- frame_offset: 0
- frame_delay: 100
- loop_delay: 50
- loop_backwards: true
- looping: false
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

# Asset_DemandDown
- frame_offset: 0
- frame_delay: 100
- loop_backwards: true
- loop_delay: 50
- looping: false
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

# Asset_DemandLeft
- frame_offset: 0
- frame_delay: 200
- loop_backwards: true
- loop_delay: 50
- looping: false
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




# Asset_DockUp
This is the edge area where suppliers and demanders can be placed
- file: ./img/dock.png
- x: 0
- y: 0
- w: 64
- h: 64

# Asset_DockRight
- file: ./img/dock.png
- x: 0
- y: 0
- w: 64
- h: 64

# Asset_DockDown
- file: ./img/dock.png
- x: 0
- y: 0
- w: 64
- h: 64

# Asset_DockLeft
- file: ./img/dock.png
- x: 0
- y: 0
- w: 64
- h: 64



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

# Asset_Machine_1_1
- unused

# Asset_Machine_1_2
- file: ./img/machines/machine_1_2.png
- w: 64
- h: 48
- file: ./img/machines/machine_orange.png
- w: 128
- h: 128

# Asset_Machine_1_3
- unused

# Asset_Machine_1_4
- unused

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

# Asset_Machine_2_3
- unused

# Asset_Machine_2_4
- unused

# Asset_Machine_3_1
- unused

# Asset_Machine_3_2
- unused

# Asset_Machine_3_3
- file: ./img/machines/machine_3_3.png
- w: 320
- h: 320
- file: ./img/machines/machine_orange.png
- w: 128
- h: 128

# Asset_Machine_3_4
- unused

# Asset_Machine_4_1
- unused

# Asset_Machine_4_2
- unused

# Asset_Machine_4_3
- unused

# Asset_Machine_4_4
- unused

# Asset_Machine_Fallback
- unused

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
(This is the background image of the canvas so not used inside rust)
- unused
- file: ./img/sand.png
- w: 128
- h: 128

# Asset_HelpBlack
Source: ikea.com via https://www.mentalfloss.com/article/58450/16-out-context-ikea-instructions-help-you-live-better-life
Used as parody
- unused
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
- unused
Source: https://www.flaticon.com/free-icon/mouse-left-button_32041
Free when with attribution
- file: ./img/lmb.png
- w: 50
- h: 50

# Asset_Rmb
- unused
Source: https://www.flaticon.com/free-icon/mouse-left-button_32041
Free when with attribution
- file: ./img/rmb.png
- w: 50
- h: 50

# Asset_SaveDark
- unused
- file: ./img/save_dark.png
- w: 48
- h: 48
# Asset_SaveLight
- file: ./img/save_light.png
- w: 48
- h: 48
# Asset_SaveGrey
- file: ./img/save_grey.png
- w: 48
- h: 48

# Asset_TrashDark
- unused
- file: ./img/trash_dark_cb.png
- w: 136
- h: 136
# Asset_TrashLight
- file: ./img/trash_light_cb.png
- w: 136
- h: 136
# Asset_TrashGrey
- file: ./img/trash_grey_cb.png
- w: 136
- h: 136
# Asset_TrashRed
- file: ./img/trash_red_cb.png
- w: 136
- h: 136
# Asset_TrashGreen
- file: ./img/trash_green_cb.png
- w: 136
- h: 136

# Asset_BrushGrey
- file: ./img/brush_grey_cb.png
- w: 128
- h: 128
# Asset_BrushDark
- unused
- file: ./img/brush_dark_cb.png
- w: 128
- h: 128
# Asset_BrushLight
- file: ./img/brush_light_cb.png
- w: 128
- h: 128
# Asset_BrushGreen
- file: ./img/brush_green_cb.png
- w: 128
- h: 128
# Asset_BrushRed
- file: ./img/brush_red_cb_x.png
- w: 128
- h: 128

# Asset_UndoLight
- file: ./img/undo_light.png
- w: 128
- h: 128
# Asset_UndoGrey
- file: ./img/undo_grey.png
- w: 128
- h: 128

# Asset_RedoLight
- file: ./img/redo_light.png
- w: 128
- h: 128
# Asset_RedoGrey
- file: ./img/redo_grey.png
- w: 128
- h: 128

# Asset_QuestFrame
- file: ./img/quest_frame.png
- w: 490
- h: 175

# Asset_DoubleArrowRight
- file: ./img/double_arrow_right.png
- w: 13
- h: 38
# Asset_SingleArrowDown
- unused
- file: ./img/single_arrow_down.png
- w: 18
- h: 13
# Asset_SingleArrowRight
- file: ./img/single_arrow_right.png
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
- file: ./img/copy_white_128.png
- w: 128
- h: 128
# Asset_CopyGrey
- file: ./img/copy_grey_128.png
- w: 128
- h: 128
# Asset_CopyGreen
- file: ./img/copy_green_128.png
- w: 128
- h: 128

# Asset_PasteWhite
- file: ./img/paste_white_128.png
- w: 128
- h: 128
# Asset_PasteGrey
- file: ./img/paste_grey_128.png
- w: 128
- h: 128
# Asset_PasteGreen
- file: ./img/paste_green_128.png
- w: 128
- h: 128

# Asset_DrmPlaceholder
See options.show_drm=false
- file: ./img/drm.png
- w: 16
- h: 16

# Asset_FullScreenBlack
- unused
- file: ./img/fullscreen_black_128.png
- w: 128
- h: 128
# Asset_FullScreenWhite
- file: ./img/fullscreen_white_128.png
- w: 128
- h: 128
# Asset_FullScreenGrey
- file: ./img/fullscreen_grey_128.png
- w: 128
- h: 128

# Asset_PlayBlack
- unused
- file: ./img/play_black.png
- w: 114
- h: 130
# Asset_PlayWhite
- file: ./img/play_white.png
- w: 114
- h: 130
# Asset_PlayGrey
- unused
- file: ./img/play_grey.png
- w: 114
- h: 130

# Asset_BwdBlack
- unused
- file: ./img/bwd_black.png
- w: 113
- h: 128
# Asset_BwdWhite
- file: ./img/bwd_white.png
- w: 113
- h: 128
# Asset_BwdGrey
- unused
- file: ./img/bwd_grey.png
- w: 113
- h: 128

# Asset_FastBwdBlack
- unused
- file: ./img/fast_bwd_black.png
- w: 183
- h: 128
# Asset_FastBwdWhite
- file: ./img/fast_bwd_white.png
- w: 183
- h: 128
# Asset_FastBwdGrey
- unused
- file: ./img/fast_bwd_grey.png
- w: 183
- h: 128

# Asset_FwdBlack
- unused
- file: ./img/fwd_black.png
- w: 113
- h: 128
# Asset_FwdWhite
- file: ./img/fwd_white.png
- w: 113
- h: 128
# Asset_FwdGrey
- unused
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
- unused
- file: ./img/fast_fwd_grey.png
- w: 182
- h: 128

# Asset_StopBlack
- unused
- file: ./img/stop_black.png
- w: 121
- h: 120
# Asset_StopWhite
- unused
- file: ./img/stop_white.png
- w: 121
- h: 120
# Asset_StopGrey
- unused
- file: ./img/stop_grey.png
- w: 121
- h: 120

# Asset_PauseBlack
- unused
- file: ./img/pause_black.png
- w: 119
- h: 120
# Asset_PauseWhite
- unused
- file: ./img/pause_white.png
- w: 119
- h: 120
# Asset_PauseGrey
- unused
- file: ./img/pause_grey.png
- w: 119
- h: 120

# Asset_Battery
- file: ./img/battery.png
- w: 70
- h: 128

`;
