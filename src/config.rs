use std::collections::HashMap;
use std::collections::HashSet;

use wasm_bindgen::{JsValue};
use js_sys;

use super::belt::*;
use super::belt_codes::*;
use super::belt_frame::*;
use super::belt_meta::*;
use super::belt_type::*;
use super::belt_type::*;
use super::machine::*;
use super::options::*;
use super::part::*;
use super::quest::*;
use super::quest_state::*;
use super::sprite_config::*;
use super::sprite_frame::*;
use super::state::*;
use super::story::*;
use super::utils::*;
use super::log;

// These index directly to the config.nodes vec and BELT_CODES (for belts)

pub const CONFIG_NODE_PART_NONE: PartKind = 0;
pub const CONFIG_NODE_PART_TRASH: PartKind = 1;
pub const CONFIG_NODE_ASSET_SUPPLY_UP: usize = 2;
pub const CONFIG_NODE_ASSET_SUPPLY_RIGHT: usize = 3;
pub const CONFIG_NODE_ASSET_SUPPLY_DOWN: usize = 4;
pub const CONFIG_NODE_ASSET_SUPPLY_LEFT: usize = 5;
pub const CONFIG_NODE_ASSET_DEMAND_UP: usize = 6;
pub const CONFIG_NODE_ASSET_DEMAND_RIGHT: usize = 7;
pub const CONFIG_NODE_ASSET_DEMAND_DOWN: usize = 8;
pub const CONFIG_NODE_ASSET_DEMAND_LEFT: usize = 9;
pub const CONFIG_NODE_ASSET_DOCK_UP: usize = 10;
pub const CONFIG_NODE_ASSET_DOCK_RIGHT: usize = 11;
pub const CONFIG_NODE_ASSET_DOCK_DOWN: usize = 12;
pub const CONFIG_NODE_ASSET_DOCK_LEFT: usize = 13;
pub const CONFIG_NODE_MACHINE_1X1: usize = 14; // unused
pub const CONFIG_NODE_MACHINE_2X2: usize = 15; // unused
pub const CONFIG_NODE_MACHINE_3X3: usize = 16; // unused
pub const CONFIG_NODE_BELT_NONE: usize = 17;
pub const CONFIG_NODE_BELT_UNKNOWN: usize = 18;
pub const CONFIG_NODE_BELT_INVALID: usize = 19;
pub const CONFIG_NODE_BELT_L_: usize = 20;
pub const CONFIG_NODE_BELT__L: usize = 21;
pub const CONFIG_NODE_BELT___L: usize = 22;
pub const CONFIG_NODE_BELT_D_: usize = 23;
pub const CONFIG_NODE_BELT_DL_: usize = 24;
pub const CONFIG_NODE_BELT_D_L: usize = 25;
pub const CONFIG_NODE_BELT_D__L: usize = 26;
pub const CONFIG_NODE_BELT__D: usize = 27;
pub const CONFIG_NODE_BELT_L_D: usize = 28;
pub const CONFIG_NODE_BELT__DL: usize = 29;
pub const CONFIG_NODE_BELT__D_L: usize = 30;
pub const CONFIG_NODE_BELT___D: usize = 31;
pub const CONFIG_NODE_BELT_L__D: usize = 32;
pub const CONFIG_NODE_BELT__L_D: usize = 33;
pub const CONFIG_NODE_BELT___DL: usize = 34;
pub const CONFIG_NODE_BELT_R_: usize = 35;
pub const CONFIG_NODE_BELT_LR_: usize = 36;
pub const CONFIG_NODE_BELT_R_L: usize = 37;
pub const CONFIG_NODE_BELT_R__L: usize = 38;
pub const CONFIG_NODE_BELT_DR_: usize = 39;
pub const CONFIG_NODE_BELT_DLR_: usize = 40;
pub const CONFIG_NODE_BELT_DR_L: usize = 41;
pub const CONFIG_NODE_BELT_DR__L: usize = 42;
pub const CONFIG_NODE_BELT_R_D: usize = 43;
pub const CONFIG_NODE_BELT_LR_D: usize = 44;
pub const CONFIG_NODE_BELT_R_DL: usize = 45;
pub const CONFIG_NODE_BELT_R_D_L: usize = 46;
pub const CONFIG_NODE_BELT_R__D: usize = 47;
pub const CONFIG_NODE_BELT_LR__D: usize = 48;
pub const CONFIG_NODE_BELT_R_L_D: usize = 49;
pub const CONFIG_NODE_BELT_R__DL: usize = 50;
pub const CONFIG_NODE_BELT__R: usize = 51;
pub const CONFIG_NODE_BELT_L_R: usize = 52;
pub const CONFIG_NODE_BELT__LR: usize = 53;
pub const CONFIG_NODE_BELT__R_L: usize = 54;
pub const CONFIG_NODE_BELT_D_R: usize = 55;
pub const CONFIG_NODE_BELT_DL_R: usize = 56;
pub const CONFIG_NODE_BELT_D_LR: usize = 57;
pub const CONFIG_NODE_BELT_D_R_L: usize = 58;
pub const CONFIG_NODE_BELT__DR: usize = 59;
pub const CONFIG_NODE_BELT_L_DR: usize = 60;
pub const CONFIG_NODE_BELT__DLR: usize = 61;
pub const CONFIG_NODE_BELT__DR_L: usize = 62;
pub const CONFIG_NODE_BELT__R_D: usize = 63;
pub const CONFIG_NODE_BELT_L_R_D: usize = 64;
pub const CONFIG_NODE_BELT__LR_D: usize = 65;
pub const CONFIG_NODE_BELT__R_DL: usize = 66;
pub const CONFIG_NODE_BELT___R: usize = 67;
pub const CONFIG_NODE_BELT_L__R: usize = 68;
pub const CONFIG_NODE_BELT__L_R: usize = 69;
pub const CONFIG_NODE_BELT___LR: usize = 70;
pub const CONFIG_NODE_BELT_D__R: usize = 71;
pub const CONFIG_NODE_BELT_DL__R: usize = 72;
pub const CONFIG_NODE_BELT_D_L_R: usize = 73;
pub const CONFIG_NODE_BELT_D__LR: usize = 74;
pub const CONFIG_NODE_BELT__D_R: usize = 75;
pub const CONFIG_NODE_BELT_L_D_R: usize = 76;
pub const CONFIG_NODE_BELT__DL_R: usize = 77;
pub const CONFIG_NODE_BELT__D_LR: usize = 78;
pub const CONFIG_NODE_BELT___DR: usize = 79;
pub const CONFIG_NODE_BELT_L__DR: usize = 80;
pub const CONFIG_NODE_BELT__L_DR: usize = 81;
pub const CONFIG_NODE_BELT___DLR: usize = 82;
pub const CONFIG_NODE_BELT_U_: usize = 83;
pub const CONFIG_NODE_BELT_LU_: usize = 84;
pub const CONFIG_NODE_BELT_U_L: usize = 85;
pub const CONFIG_NODE_BELT_U__L: usize = 86;
pub const CONFIG_NODE_BELT_DU_: usize = 87;
pub const CONFIG_NODE_BELT_DLU_: usize = 88;
pub const CONFIG_NODE_BELT_DU_L: usize = 89;
pub const CONFIG_NODE_BELT_DU__L: usize = 90;
pub const CONFIG_NODE_BELT_U_D: usize = 91;
pub const CONFIG_NODE_BELT_LU_D: usize = 92;
pub const CONFIG_NODE_BELT_U_DL: usize = 93;
pub const CONFIG_NODE_BELT_U_D_L: usize = 94;
pub const CONFIG_NODE_BELT_U__D: usize = 95;
pub const CONFIG_NODE_BELT_LU__D: usize = 96;
pub const CONFIG_NODE_BELT_U_L_D: usize = 97;
pub const CONFIG_NODE_BELT_U__DL: usize = 98;
pub const CONFIG_NODE_BELT_RU_: usize = 99;
pub const CONFIG_NODE_BELT_LRU_: usize = 100;
pub const CONFIG_NODE_BELT_RU_L: usize = 101;
pub const CONFIG_NODE_BELT_RU__L: usize = 102;
pub const CONFIG_NODE_BELT_DRU_: usize = 103;
pub const CONFIG_NODE_BELT_DLRU_: usize = 104;
pub const CONFIG_NODE_BELT_DRU_L: usize = 105;
pub const CONFIG_NODE_BELT_DRU__L: usize = 106;
pub const CONFIG_NODE_BELT_RU_D: usize = 107;
pub const CONFIG_NODE_BELT_LRU_D: usize = 108;
pub const CONFIG_NODE_BELT_RU_DL: usize = 109;
pub const CONFIG_NODE_BELT_RU_D_L: usize = 110;
pub const CONFIG_NODE_BELT_RU__D: usize = 111;
pub const CONFIG_NODE_BELT_LRU__D: usize = 112;
pub const CONFIG_NODE_BELT_RU_L_D: usize = 113;
pub const CONFIG_NODE_BELT_RU__DL: usize = 114;
pub const CONFIG_NODE_BELT_U_R: usize = 115;
pub const CONFIG_NODE_BELT_LU_R: usize = 116;
pub const CONFIG_NODE_BELT_U_LR: usize = 117;
pub const CONFIG_NODE_BELT_U_R_L: usize = 118;
pub const CONFIG_NODE_BELT_DU_R: usize = 119;
pub const CONFIG_NODE_BELT_DLU_R: usize = 120;
pub const CONFIG_NODE_BELT_DU_LR: usize = 121;
pub const CONFIG_NODE_BELT_DU_R_L: usize = 122;
pub const CONFIG_NODE_BELT_U_DR: usize = 123;
pub const CONFIG_NODE_BELT_LU_DR: usize = 124;
pub const CONFIG_NODE_BELT_U_DLR: usize = 125;
pub const CONFIG_NODE_BELT_U_DR_L: usize = 126;
pub const CONFIG_NODE_BELT_U_R_D: usize = 127;
pub const CONFIG_NODE_BELT_LU_R_D: usize = 128;
pub const CONFIG_NODE_BELT_U_LR_D: usize = 129;
pub const CONFIG_NODE_BELT_U_R_DL: usize = 130;
pub const CONFIG_NODE_BELT_U__R: usize = 131;
pub const CONFIG_NODE_BELT_LU__R: usize = 132;
pub const CONFIG_NODE_BELT_U_L_R: usize = 133;
pub const CONFIG_NODE_BELT_U__LR: usize = 134;
pub const CONFIG_NODE_BELT_DU__R: usize = 135;
pub const CONFIG_NODE_BELT_DLU__R: usize = 136;
pub const CONFIG_NODE_BELT_DU_L_R: usize = 137;
pub const CONFIG_NODE_BELT_DU__LR: usize = 138;
pub const CONFIG_NODE_BELT_U_D_R: usize = 139;
pub const CONFIG_NODE_BELT_LU_D_R: usize = 140;
pub const CONFIG_NODE_BELT_U_DL_R: usize = 141;
pub const CONFIG_NODE_BELT_U_D_LR: usize = 142;
pub const CONFIG_NODE_BELT_U__DR: usize = 143;
pub const CONFIG_NODE_BELT_LU__DR: usize = 144;
pub const CONFIG_NODE_BELT_U_L_DR: usize = 145;
pub const CONFIG_NODE_BELT_U__DLR: usize = 146;
pub const CONFIG_NODE_BELT__U: usize = 147;
pub const CONFIG_NODE_BELT_L_U: usize = 148;
pub const CONFIG_NODE_BELT__LU: usize = 149;
pub const CONFIG_NODE_BELT__U_L: usize = 150;
pub const CONFIG_NODE_BELT_D_U: usize = 151;
pub const CONFIG_NODE_BELT_DL_U: usize = 152;
pub const CONFIG_NODE_BELT_D_LU: usize = 153;
pub const CONFIG_NODE_BELT_D_U_L: usize = 154;
pub const CONFIG_NODE_BELT__DU: usize = 155;
pub const CONFIG_NODE_BELT_L_DU: usize = 156;
pub const CONFIG_NODE_BELT__DLU: usize = 157;
pub const CONFIG_NODE_BELT__DU_L: usize = 158;
pub const CONFIG_NODE_BELT__U_D: usize = 159;
pub const CONFIG_NODE_BELT_L_U_D: usize = 160;
pub const CONFIG_NODE_BELT__LU_D: usize = 161;
pub const CONFIG_NODE_BELT__U_DL: usize = 162;
pub const CONFIG_NODE_BELT_R_U: usize = 163;
pub const CONFIG_NODE_BELT_LR_U: usize = 164;
pub const CONFIG_NODE_BELT_R_LU: usize = 165;
pub const CONFIG_NODE_BELT_R_U_L: usize = 166;
pub const CONFIG_NODE_BELT_DR_U: usize = 167;
pub const CONFIG_NODE_BELT_DLR_U: usize = 168;
pub const CONFIG_NODE_BELT_DR_LU: usize = 169;
pub const CONFIG_NODE_BELT_DR_U_L: usize = 170;
pub const CONFIG_NODE_BELT_R_DU: usize = 171;
pub const CONFIG_NODE_BELT_LR_DU: usize = 172;
pub const CONFIG_NODE_BELT_R_DLU: usize = 173;
pub const CONFIG_NODE_BELT_R_DU_L: usize = 174;
pub const CONFIG_NODE_BELT_R_U_D: usize = 175;
pub const CONFIG_NODE_BELT_LR_U_D: usize = 176;
pub const CONFIG_NODE_BELT_R_LU_D: usize = 177;
pub const CONFIG_NODE_BELT_R_U_DL: usize = 178;
pub const CONFIG_NODE_BELT__RU: usize = 179;
pub const CONFIG_NODE_BELT_L_RU: usize = 180;
pub const CONFIG_NODE_BELT__LRU: usize = 181;
pub const CONFIG_NODE_BELT__RU_L: usize = 182;
pub const CONFIG_NODE_BELT_D_RU: usize = 183;
pub const CONFIG_NODE_BELT_DL_RU: usize = 184;
pub const CONFIG_NODE_BELT_D_LRU: usize = 185;
pub const CONFIG_NODE_BELT_D_RU_L: usize = 186;
pub const CONFIG_NODE_BELT__DRU: usize = 187;
pub const CONFIG_NODE_BELT_L_DRU: usize = 188;
pub const CONFIG_NODE_BELT__DLRU: usize = 189;
pub const CONFIG_NODE_BELT__DRU_L: usize = 190;
pub const CONFIG_NODE_BELT__RU_D: usize = 191;
pub const CONFIG_NODE_BELT_L_RU_D: usize = 192;
pub const CONFIG_NODE_BELT__LRU_D: usize = 193;
pub const CONFIG_NODE_BELT__RU_DL: usize = 194;
pub const CONFIG_NODE_BELT__U_R: usize = 195;
pub const CONFIG_NODE_BELT_L_U_R: usize = 196;
pub const CONFIG_NODE_BELT__LU_R: usize = 197;
pub const CONFIG_NODE_BELT__U_LR: usize = 198;
pub const CONFIG_NODE_BELT_D_U_R: usize = 199;
pub const CONFIG_NODE_BELT_DL_U_R: usize = 200;
pub const CONFIG_NODE_BELT_D_LU_R: usize = 201;
pub const CONFIG_NODE_BELT_D_U_LR: usize = 202;
pub const CONFIG_NODE_BELT__DU_R: usize = 203;
pub const CONFIG_NODE_BELT_L_DU_R: usize = 204;
pub const CONFIG_NODE_BELT__DLU_R: usize = 205;
pub const CONFIG_NODE_BELT__DU_LR: usize = 206;
pub const CONFIG_NODE_BELT__U_DR: usize = 207;
pub const CONFIG_NODE_BELT_L_U_DR: usize = 208;
pub const CONFIG_NODE_BELT__LU_DR: usize = 209;
pub const CONFIG_NODE_BELT__U_DLR: usize = 210;
pub const CONFIG_NODE_BELT___U: usize = 211;
pub const CONFIG_NODE_BELT_L__U: usize = 212;
pub const CONFIG_NODE_BELT__L_U: usize = 213;
pub const CONFIG_NODE_BELT___LU: usize = 214;
pub const CONFIG_NODE_BELT_D__U: usize = 215;
pub const CONFIG_NODE_BELT_DL__U: usize = 216;
pub const CONFIG_NODE_BELT_D_L_U: usize = 217;
pub const CONFIG_NODE_BELT_D__LU: usize = 218;
pub const CONFIG_NODE_BELT__D_U: usize = 219;
pub const CONFIG_NODE_BELT_L_D_U: usize = 220;
pub const CONFIG_NODE_BELT__DL_U: usize = 221;
pub const CONFIG_NODE_BELT__D_LU: usize = 222;
pub const CONFIG_NODE_BELT___DU: usize = 223;
pub const CONFIG_NODE_BELT_L__DU: usize = 224;
pub const CONFIG_NODE_BELT__L_DU: usize = 225;
pub const CONFIG_NODE_BELT___DLU: usize = 226;
pub const CONFIG_NODE_BELT_R__U: usize = 227;
pub const CONFIG_NODE_BELT_LR__U: usize = 228;
pub const CONFIG_NODE_BELT_R_L_U: usize = 229;
pub const CONFIG_NODE_BELT_R__LU: usize = 230;
pub const CONFIG_NODE_BELT_DR__U: usize = 231;
pub const CONFIG_NODE_BELT_DLR__U: usize = 232;
pub const CONFIG_NODE_BELT_DR_L_U: usize = 233;
pub const CONFIG_NODE_BELT_DR__LU: usize = 234;
pub const CONFIG_NODE_BELT_R_D_U: usize = 235;
pub const CONFIG_NODE_BELT_LR_D_U: usize = 236;
pub const CONFIG_NODE_BELT_R_DL_U: usize = 237;
pub const CONFIG_NODE_BELT_R_D_LU: usize = 238;
pub const CONFIG_NODE_BELT_R__DU: usize = 239;
pub const CONFIG_NODE_BELT_LR__DU: usize = 240;
pub const CONFIG_NODE_BELT_R_L_DU: usize = 241;
pub const CONFIG_NODE_BELT_R__DLU: usize = 242;
pub const CONFIG_NODE_BELT__R_U: usize = 243;
pub const CONFIG_NODE_BELT_L_R_U: usize = 244;
pub const CONFIG_NODE_BELT__LR_U: usize = 245;
pub const CONFIG_NODE_BELT__R_LU: usize = 246;
pub const CONFIG_NODE_BELT_D_R_U: usize = 247;
pub const CONFIG_NODE_BELT_DL_R_U: usize = 248;
pub const CONFIG_NODE_BELT_D_LR_U: usize = 249;
pub const CONFIG_NODE_BELT_D_R_LU: usize = 250;
pub const CONFIG_NODE_BELT__DR_U: usize = 251;
pub const CONFIG_NODE_BELT_L_DR_U: usize = 252;
pub const CONFIG_NODE_BELT__DLR_U: usize = 253;
pub const CONFIG_NODE_BELT__DR_LU: usize = 254;
pub const CONFIG_NODE_BELT__R_DU: usize = 255;
pub const CONFIG_NODE_BELT_L_R_DU: usize = 256;
pub const CONFIG_NODE_BELT__LR_DU: usize = 257;
pub const CONFIG_NODE_BELT__R_DLU: usize = 258;
pub const CONFIG_NODE_BELT___RU: usize = 259;
pub const CONFIG_NODE_BELT_L__RU: usize = 260;
pub const CONFIG_NODE_BELT__L_RU: usize = 261;
pub const CONFIG_NODE_BELT___LRU: usize = 262;
pub const CONFIG_NODE_BELT_D__RU: usize = 263;
pub const CONFIG_NODE_BELT_DL__RU: usize = 264;
pub const CONFIG_NODE_BELT_D_L_RU: usize = 265;
pub const CONFIG_NODE_BELT_D__LRU: usize = 266;
pub const CONFIG_NODE_BELT__D_RU: usize = 267;
pub const CONFIG_NODE_BELT_L_D_RU: usize = 268;
pub const CONFIG_NODE_BELT__DL_RU: usize = 269;
pub const CONFIG_NODE_BELT__D_LRU: usize = 270;
pub const CONFIG_NODE_BELT___DRU: usize = 271;
pub const CONFIG_NODE_BELT_L__DRU: usize = 272;
pub const CONFIG_NODE_BELT__L_DRU: usize = 273;
pub const CONFIG_NODE_BELT___DLRU: usize = 274;
pub const CONFIG_NODE_ASSET_WEE_WOO: usize = 275;
pub const CONFIG_NODE_ASSET_MISSING_INPUTS: usize = 276;
pub const CONFIG_NODE_ASSET_MISSING_OUTPUTS: usize = 277;
pub const CONFIG_NODE_ASSET_MISSING_PURPOSE: usize = 278; // obsoleted, no longer used
pub const CONFIG_NODE_ASSET_MACHINE_1_1: usize = 279;
pub const CONFIG_NODE_ASSET_MACHINE_1_2: usize = 280;
pub const CONFIG_NODE_ASSET_MACHINE_1_3: usize = 281;
pub const CONFIG_NODE_ASSET_MACHINE_1_4: usize = 282;
pub const CONFIG_NODE_ASSET_MACHINE_2_1: usize = 283;
pub const CONFIG_NODE_ASSET_MACHINE_2_2: usize = 284;
pub const CONFIG_NODE_ASSET_MACHINE_2_3: usize = 285;
pub const CONFIG_NODE_ASSET_MACHINE_2_4: usize = 286;
pub const CONFIG_NODE_ASSET_MACHINE_3_1: usize = 287;
pub const CONFIG_NODE_ASSET_MACHINE_3_2: usize = 288;
pub const CONFIG_NODE_ASSET_MACHINE_3_3: usize = 289;
pub const CONFIG_NODE_ASSET_MACHINE_3_4: usize = 290;
pub const CONFIG_NODE_ASSET_MACHINE_4_1: usize = 291;
pub const CONFIG_NODE_ASSET_MACHINE_4_2: usize = 292;
pub const CONFIG_NODE_ASSET_MACHINE_4_3: usize = 293;
pub const CONFIG_NODE_ASSET_MACHINE_4_4: usize = 294;
pub const CONFIG_NODE_ASSET_MACHINE_FALLBACK: usize = 295;
pub const CONFIG_NODE_ASSET_DUMP_TRUCK: usize = 296;
pub const CONFIG_NODE_ASSET_SAND: usize = 297;
pub const CONFIG_NODE_ASSET_HELP_BLACK: usize = 298;
pub const CONFIG_NODE_ASSET_HELP_RED: usize = 299;
pub const CONFIG_NODE_ASSET_MANUAL: usize = 300;
pub const CONFIG_NODE_ASSET_LMB: usize = 301;
pub const CONFIG_NODE_ASSET_RMB: usize = 302;
pub const CONFIG_NODE_ASSET_SAVE_DARK: usize = 303;
pub const CONFIG_NODE_ASSET_QUEST_FRAME: usize = 304;
pub const CONFIG_NODE_ASSET_DOUBLE_ARROW_RIGHT: usize = 305;
pub const CONFIG_NODE_ASSET_SINGLE_ARROW_DOWN: usize = 306;
pub const CONFIG_NODE_ASSET_SINGLE_ARROW_RIGHT: usize = 307;
pub const CONFIG_NODE_ASSET_SCREEN_LOADER: usize = 308;
pub const CONFIG_NODE_ASSET_SCREEN_PLAY: usize = 309;
pub const CONFIG_NODE_ASSET_BUTTON_UP_1: usize = 310; // 9-slice, in order
pub const CONFIG_NODE_ASSET_BUTTON_UP_2: usize = 311;
pub const CONFIG_NODE_ASSET_BUTTON_UP_3: usize = 312;
pub const CONFIG_NODE_ASSET_BUTTON_UP_4: usize = 313;
pub const CONFIG_NODE_ASSET_BUTTON_UP_6: usize = 314;
pub const CONFIG_NODE_ASSET_BUTTON_UP_7: usize = 315;
pub const CONFIG_NODE_ASSET_BUTTON_UP_8: usize = 316;
pub const CONFIG_NODE_ASSET_BUTTON_UP_9: usize = 317;
pub const CONFIG_NODE_ASSET_BUTTON_DOWN_1: usize = 318; // 9-slice, in order
pub const CONFIG_NODE_ASSET_BUTTON_DOWN_2: usize = 319;
pub const CONFIG_NODE_ASSET_BUTTON_DOWN_3: usize = 320;
pub const CONFIG_NODE_ASSET_BUTTON_DOWN_4: usize = 321;
pub const CONFIG_NODE_ASSET_BUTTON_DOWN_6: usize = 322;
pub const CONFIG_NODE_ASSET_BUTTON_DOWN_7: usize = 323;
pub const CONFIG_NODE_ASSET_BUTTON_DOWN_8: usize = 324;
pub const CONFIG_NODE_ASSET_BUTTON_DOWN_9: usize = 325;
pub const CONFIG_NODE_ASSET_SAVE_LIGHT: usize = 326;
pub const CONFIG_NODE_ASSET_SAVE_GREY: usize = 327;
pub const CONFIG_NODE_ASSET_TRASH_DARK: usize = 328;
pub const CONFIG_NODE_ASSET_TRASH_LIGHT: usize = 329;
pub const CONFIG_NODE_ASSET_TRASH_GREY: usize = 330;
pub const CONFIG_NODE_ASSET_TRASH_RED: usize = 331;
pub const CONFIG_NODE_ASSET_TRASH_GREEN: usize = 332;
pub const CONFIG_NODE_STORY_DEFAULT: usize = 333;
pub const CONFIG_NDOE_ASSET_TREASURE: usize = 334;
pub const CONFIG_NODE_ASSET_PICKAXE: usize = 335;
pub const CONFIG_NODE_ASSET_DRM_PLACEHOLDER: usize = 336;
pub const CONFIG_NODE_ASSET_BRUSH_DARK: usize = 337;
pub const CONFIG_NODE_ASSET_BRUSH_LIGHT: usize = 338;
pub const CONFIG_NODE_ASSET_BRUSH_RED: usize = 339;
pub const CONFIG_NODE_ASSET_BRUSH_GREEN: usize = 340;
pub const CONFIG_NODE_ASSET_BRUSH_GREY: usize = 341;
pub const CONFIG_NODE_ASSET_UNDO_LIGHT: usize = 342;
pub const CONFIG_NODE_ASSET_UNDO_GREY: usize = 343;
pub const CONFIG_NODE_ASSET_REDO_LIGHT: usize = 344;
pub const CONFIG_NODE_ASSET_REDO_GREY: usize = 345;
pub const CONFIG_NODE_ASSET_LOGO: usize = 346;
pub const CONFIG_NODE_ASSET_COPY_GREY: usize = 347;
pub const CONFIG_NODE_ASSET_PASTE_GREY: usize = 348;
pub const CONFIG_NODE_ASSET_COPY_GREEN: usize = 349;
pub const CONFIG_NODE_ASSET_PASTE_GREEN: usize = 350;
pub const CONFIG_NODE_ASSET_FACTORY: usize = 351;
pub const CONFIG_NODE_ASSET_FULLSCREEN_BLACK: usize = 352;
pub const CONFIG_NODE_ASSET_FULLSCREEN_WHITE: usize = 353;
pub const CONFIG_NODE_ASSET_FULLSCREEN_GREY: usize = 354;
pub const CONFIG_NODE_ASSET_PLAY_BLACK: usize = 355;
pub const CONFIG_NODE_ASSET_PLAY_WHITE: usize = 356;
pub const CONFIG_NODE_ASSET_PLAY_GREY: usize = 357;
pub const CONFIG_NODE_ASSET_FWD_BLACK: usize = 358;
pub const CONFIG_NODE_ASSET_FWD_WHITE: usize = 359;
pub const CONFIG_NODE_ASSET_FWD_GREY: usize = 360;
pub const CONFIG_NODE_ASSET_FAST_FWD_BLACK: usize = 361;
pub const CONFIG_NODE_ASSET_FAST_FWD_WHITE: usize = 362;
pub const CONFIG_NODE_ASSET_FAST_FWD_GREY: usize = 363;
pub const CONFIG_NODE_ASSET_BWD_BLACK: usize = 364;
pub const CONFIG_NODE_ASSET_BWD_WHITE: usize = 365;
pub const CONFIG_NODE_ASSET_BWD_GREY: usize = 366;
pub const CONFIG_NODE_ASSET_FAST_BWD_BLACK: usize = 367;
pub const CONFIG_NODE_ASSET_FAST_BWD_WHITE: usize = 368;
pub const CONFIG_NODE_ASSET_FAST_BWD_GREY: usize = 369;
pub const CONFIG_NODE_ASSET_STOP_BLACK: usize = 370;
pub const CONFIG_NODE_ASSET_STOP_WHITE: usize = 371;
pub const CONFIG_NODE_ASSET_STOP_GREY: usize = 372;
pub const CONFIG_NODE_ASSET_PAUSE_BLACK: usize = 373;
pub const CONFIG_NODE_ASSET_PAUSE_WHITE: usize = 374;
pub const CONFIG_NODE_ASSET_PAUSE_GREY: usize = 375;
pub const CONFIG_NODE_ASSET_COPY_WHITE: usize = 376;
pub const CONFIG_NODE_ASSET_PASTE_WHITE: usize = 377;
pub const CONFIG_NODE_ASSET_HELP_WHITE: usize = 378;
pub const CONFIG_NODE_ASSET_HELP_GREY: usize = 379;
pub const CONFIG_NODE_ASSET_BATTERY: usize = 380;

#[derive(Debug)]
pub struct Config {
  pub nodes: Vec<ConfigNode>,
  pub stories: Vec<Story>,
  pub initial_active_story_index: usize, // Maps to config.stories[], see state.active_story_index
  pub node_name_to_index: HashMap<String, PartKind>,
  pub sprite_cache_lookup: HashMap<String, usize>, // indexes into sprite_cache_canvas
  pub sprite_cache_order: Vec<String>, // srcs by index.
  pub sprite_cache_canvas: Vec<web_sys::HtmlImageElement>,
  pub sprite_cache_loading: bool,
}

#[derive(Debug)]
pub struct ConfigNode {
  pub index: usize, // own index in the config.nodes vec
  pub kind: ConfigNodeKind,
  pub name: String,
  pub raw_name: String,
  pub unused: bool,

  // Story
  pub story_index: usize, // Index on config.stories[]. If not a Story then this node belongs to the Story pointed to.

  // Quest
  // A quest becomes available as soon as all of its starting parts are available.
  pub quest_index: usize, // Index on the quest lists. Zero for non-quests.
  pub quest_init_status: QuestStatus,
  pub starting_part_by_name: Vec<String>, // Fully qualified name. These parts are available when this quest becomes available
  pub starting_part_by_index: Vec<usize>, // These parts are available when this quest becomes available
  pub production_target_by_name: Vec<(u32, String)>, // Fully qualified name. count,name pairs, you need this to finish the quest
  pub production_target_by_index: Vec<(u32, PartKind)>, // count,index pairs, you need this to finish the quest
  pub required_by_quest_indexes: Vec<usize>, // List of quests that depend on this quest before becoming available. Computed after parsing config.
  pub unlocks_after_by_name: Vec<String>, // Fully qualified name. Becomes available when these quests are finished.
  pub unlocks_after_by_index: Vec<usize>, // Becomes available when these quests are finished
  pub unlocks_todo_by_index: Vec<usize>, // Which quests still need to be unlocked before this one unlocks? Note: this is only to hash out the final state for the (static) config object. The "runtime" value is QuestState#unlocks_todo

  // Part
  pub pattern_by_index: Vec<PartKind>, // Machine pattern that generates this part (part_kind)
  pub pattern_by_name: Vec<String>, // Actual names. Used while parsing. Should only be used for debugging afterwards
  pub pattern_by_icon: Vec<char>, // Char icons. Should only be used for debugging
  pub pattern: String, // pattern_by_icon as a string cached (or "prerendered")
  pub pattern_unique_kinds: Vec<PartKind>, // Unique non-empty part kinds. We can use this to quickly find machines that have received these parts.
  pub icon: char, // Single (unique) character that also represents this part internally
  pub special: (char, u8), // (special kind, special level). for the maze runner.
  pub machine_width: usize, // When creating a machine for this part, this is the dimension
  pub machine_height: usize, // When creating a machine for this part, this is the dimension
  pub machine_asset_name: String, // Asset to use when painting this machine. Only used at config parse time. See machine_asset_index for actual usage.
  pub machine_asset_index: usize, // Asset (config.nodes index) to use when painting this machine.

  // Graphics
  pub drm: bool, // When true, the art for this node is not owned. Use with options.show_drm=false to create safe public media with placeholders
  pub sprite_config: SpriteConfig,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConfigNodeKind {
  Asset,
  Part,
  Quest,
  Belt,
  Story,
}

pub fn parse_config_md(trace_parse_config_md: bool, config: String) -> Config {
  // Parse Fake MD config
  log!("parse_config_md(trace_parse_config_md={})", trace_parse_config_md);

  // This pseudo-md config file is a series of node definitions.
  // Names are case insensitive.
  // Each node starts with an md header (`# Abc_Def`) where the name is split between
  // a type and a name, separated by one underscore. (The underscore is basically to support
  // double-click-to-select the whole name, otherwise I would have picked a colon.)
  // The body of a node is zero or more list elements which are each key value pairs, separated
  // by a colon.
  // There are a few fields currently. For example: `parents` and `requires`.
  // - `parents`: a comma separated list of zero or more (fully qualified) parent node names.
  // - `requirements`: a comma separated list of zero or more, spacing separated counts and
  //   (fully qualified) node name pairs.
  // - `pattern`: input parts for the machine to generate this part. spacing separated. Underscore
  //   for empty spot. Single line. (Parsing multi-line pattern is just too much work.) Can use
  //   the char icon name instead of the full qualified name.
  // - `char`: CLI printing code. In some places, the char icon can be used instead of full name.
  // - `file`: source of the sprite image file
  // - `x`: offset of sprite for this part
  // - `y`: offset of sprite for this part
  // - `w`: width of the sprite for this part
  // - `h`: heigth of the sprite for this part
  // All fields are optional and default to an empty list, string, or 0.
  // All spacing is optional.
  // Lines that do not start with a `#` or `-` are ignored.
  // Only the space is considered for spacing, not tabs.
  // The idea is to create a tree/graph structure this way without too many syntactical limits.
  // Parsing can be simple on a high level, albeit is less efficient. Which should be fine.
  // The internal structure should be dumpable to a "dot" file, which can be consumed by open source
  // tooling to generate a proper graph.
  // Example:
  // # Quest_Foo
  // - parents: Quest_A, Quest_B
  // - requires: 10 Part_A, 200 Part_B

  let mut current_story_index = 0; // Start with the first one. Maps to config.stories[]

  let mut nodes: Vec<ConfigNode> = get_system_nodes();
  let mut stories: Vec<Story> = vec!(Story {
    story_node_index: CONFIG_NODE_STORY_DEFAULT,
    part_nodes: vec!(),
    quest_nodes: vec!(),
  });

  let mut first_frame = true;
  let mut seen_header = false;
  let mut current_node_index = 0;

  let mut full_name_to_node_index: HashMap<String, usize> = HashMap::new();
  // Add the system nodes into it
  nodes.iter().for_each(|node| {
    full_name_to_node_index.insert(node.raw_name.clone(), node.index);
  });

  let mut active_story_index = 0;
  let mut story_activated = false;

  config.lines().for_each(
    |line| {
      let trimmed = line.trim();
      match trimmed.chars().nth(0) {
        Some('#') => {
          match trimmed.chars().nth(1) {
            Some(' ') => {
              // Okay, proceed parsing a header
            },
            _ => {
              // Not conforming a header. Consider this a comment.
              if trace_parse_config_md { log!("This line is considered comment because it starts with a hash but not a space: {:?}", trimmed); }
              return;
            },
          }

          seen_header = true;
          first_frame = true;

          let rest = trimmed[1..].trim();
          if trace_parse_config_md { log!("Next # header: {:?}. Previous node was: {:?}", rest, nodes[nodes.len()-1].raw_name); }
          let mut split = rest.split('_');
          let kind = split.next().or(Some("UnknownPrefix")).unwrap().trim(); // first

          let name = split.collect::<Vec<&str>>();
          let name = name.join("_");
          let name = name.trim();
          // let mut name = split.next_back().or(Some("MissingName")).unwrap().trim(); // last
          let icon = if rest == "Part_None" { ' ' } else { '?' };
          let node_index: usize = config_full_node_name_to_target_index(rest, kind, nodes.len());
          let node_index =
            if node_index == nodes.len() {
              if let Some(&node_index) = full_name_to_node_index.get(rest.clone()) {
                if trace_parse_config_md { log!("Amending to index {} for node {}", node_index, rest); }
                node_index
              } else {
                if trace_parse_config_md { log!("Creating new index {} for {}", rest, node_index); }
                full_name_to_node_index.insert(rest.to_string(), node_index);
                node_index
              }
            } else {
              if trace_parse_config_md { log!("System {} node index {}", rest, node_index); }
              node_index
            };
          if trace_parse_config_md { log!("- raw: `{}`, kind: `{}`, name: `{}`, index: {}", rest, kind, name, node_index); }
          let is_fresh_node = node_index == nodes.len();
          if is_fresh_node {
            let current_node = ConfigNode {
              index: node_index,
              kind: match kind {
                "Asset" => ConfigNodeKind::Asset,
                "Quest" => ConfigNodeKind::Quest,
                "Part" => ConfigNodeKind::Part,
                "Belt" => ConfigNodeKind::Belt,
                "Story" => ConfigNodeKind::Story,
                _ => panic!("Unsupported node kind. Node headers should be composed like Kind_Name and the kind can only be Quest, Part, or Belt. But it was {:?} (`{}`)", kind, rest),
              },
              name: name.to_string(),
              raw_name: rest.to_string(),
              unused: false,
              story_index: active_story_index,
              quest_index: 0,
              quest_init_status: QuestStatus::Waiting,
              unlocks_after_by_name: vec!(),
              unlocks_after_by_index: vec!(),
              unlocks_todo_by_index: vec!(),
              starting_part_by_name: vec!(),
              starting_part_by_index: vec!(),
              production_target_by_name: vec!(),
              production_target_by_index: vec!(),
              required_by_quest_indexes: vec!(),
              pattern_by_index: vec!(),
              pattern_by_name: vec!(),
              pattern_by_icon: vec!(),
              pattern: "".to_string(),
              pattern_unique_kinds: vec!(),
              icon,
              machine_width: 0,
              machine_height: 0,
              machine_asset_name: "Asset_Machine_3_3".to_string(),
              machine_asset_index: CONFIG_NODE_ASSET_MACHINE_3_3,
              special: ('n', 0),
              drm: false,
              sprite_config: SpriteConfig {
                frame_offset: 0,
                frame_count: 1,
                frame_direction: SpriteConfigDirection::Right,
                initial_delay: 0,
                frame_delay: 0,
                looping: false,
                loop_delay: 0,
                loop_backwards: false,
                frames: vec![SpriteFrame {
                  file: "".to_string(),
                  name: "untitled frame".to_string(),
                  file_canvas_cache_index: 0,
                  x: 0.0,
                  y: 0.0,
                  w: 0.0,
                  h: 0.0
                }]
              },
            };
            nodes.push(current_node);
          }
          current_node_index = node_index;
          match kind {
            "Asset" => {}
            "Quest" => {
              // quest_nodes_by_index.push(node_index);
              // Register as node for the current story
              if trace_parse_config_md { log!("Adding quest_node_index {} to story_index {}", node_index, current_story_index); }
              let story = &mut stories[current_story_index];
              nodes[node_index].quest_index = story.quest_nodes.len();
              nodes[node_index].story_index = current_story_index;
              story.quest_nodes.push(node_index);
            },
            "Part" => {
              // Register as node for the current story
              if trace_parse_config_md { log!("Adding part {} ({}) to stories {} / {}", node_index, nodes[node_index].raw_name, current_story_index, stories.len()); }
              stories[current_story_index].part_nodes.push(node_index);
              nodes[node_index].story_index = current_story_index;
            },
            "Supply" => {}
            "Demand" => {}
            "Dock" => {}
            "Machine" => {}
            "Belt" => {},
            "Story" => {
              let mut next_story_index = stories.len();
              // This story node may override an existing one so we search for that first
              for (story_index, story) in stories.iter().enumerate() {
                if story.story_node_index == node_index {
                  next_story_index = story_index;
                }
              }
              current_story_index = next_story_index;
              // Note: if two nodes with the same name appear the later one overrides the first one, hence this check
              if next_story_index == stories.len() {
                if trace_parse_config_md { log!("Added new Story, index {}, name {}", next_story_index, nodes[node_index].raw_name); }
                stories.push(Story {
                  story_node_index: node_index,
                  part_nodes: vec!(),
                  quest_nodes: vec!(),
                });
                let len = nodes.len();
                nodes[len - 1].story_index = next_story_index;
              }
              if trace_parse_config_md { log!("Changing to quest_index {} ({})", current_story_index, nodes[node_index].raw_name); }
            },
            _ => panic!("Unsupported node kind. Node headers should be composed like Kind_Name and the kind can only be Quest, Part, Supply, Demand, Machine, Belt, or Dock. But it was {:?}", kind),
          }
        }
        Some('-') => {
          match trimmed.chars().nth(1) {
            Some(' ') => {
              // Okay, proceed parsing a header
            },
            _ => {
              // Not conforming a header. Consider this a comment.
              if trace_parse_config_md { log!("This line is considered comment because it starts with a hash but not a space: {:?}", trimmed); }
              return;
            },
          }

          if !seen_header {
            // Could ignore this with a warning ...
            panic!("Invalid config; found line starting with `-` before seeing a line starting with `#`");
          }

          let rest = trimmed[1..].trim();
          let mut split = rest.split(':');
          let label = split.next().or(Some("_")).unwrap().trim(); // first
          let value_raw = split.next_back().or(Some("")).unwrap().trim(); // last

          match label {
            "active" => {
              // This means the current story should be considered the active story
              // Reject if there was already a story that was marked active.
              if story_activated {
                panic!("Should not activate more than one story. Find \"- active\" and make sure there is only one");
              }
              active_story_index = current_story_index;
              story_activated = true;
              if trace_parse_config_md { log!("Marked {} as the active_story_index", current_story_index); }
            }
            "after" => {
              // This should be a list of zero or more quests that are required to unlock this quest
              let pairs = value_raw.split(',');
              for s in pairs {
                let pairs = s.split(' ');
                for name_untrimmed in pairs {
                  let name = name_untrimmed.trim();
                  if name != "" {
                    nodes[current_node_index].unlocks_after_by_name.push(name.trim().to_string());
                  }
                }
              }
            }
            "parts" => {
              // Note: this was an early way of unlocking parts for the next step. But later
              //       that process automated and the game will automatically release any
              //       part that is necessary to construct the target(s). Not recursively though.
              //       I've left it in here so you can still release other parts, even if you
              //       won't immediately use them. Maybe there's a use case so why not.

              // This is a list of zero or more parts that unlock when this quest unlocks
              // So it's not _after_ this quest completes, but parts that unlock when starting this quest
              let pairs = value_raw.split(',');
              for s in pairs {
                let pairs = s.split(' ');
                for name_untrimmed in pairs {
                  let name = name_untrimmed.trim();
                  if name != "" {
                    nodes[current_node_index].starting_part_by_name.push(name.to_string());
                  }
                }
              }
            }
            "targets" => {
              // One or more pairs of counts and parts, the requirements to finish this quest.
              let pairs = value_raw.split(',');
              for pair_untrimmed in pairs {
                let pair = pair_untrimmed.trim();
                if pair != "" {
                  let split = pair.trim().split(' ').collect::<Vec<&str>>();
                  let count_str = split[0].trim();
                  // The count can end with an `x`, strip it
                  let count_str2 =
                    if count_str.ends_with('x') {
                      count_str[..count_str.len()-1].trim()
                    } else {
                      count_str
                    };
                  let count = count_str2.parse::<u32>().or::<Result<u32, &str>>(Ok(0u32)).unwrap();
                  if count == 0 {
                    panic!("Config error: count for {} is zero or otherwise invalid. String found is `{}` (config value = `{}`, after x: `{}`)", nodes[nodes.len() - 1].raw_name, count_str, pair_untrimmed, count_str2);
                  }
                  let name = split[split.len() - 1]; // Ignore multiple spaces between, :shrug:
                  let name = if name == "" { "MissingName" } else { name };
                  if trace_parse_config_md { log!("Parsing counts: `{}` into `{:?}` -> `{}` and `{}`", pair, split, count, name); }
                  nodes[current_node_index].production_target_by_name.push((count, name.to_string()));
                }
              }
            }
            "pattern" => {
              // Optional pattern for creating this part in a machine
              // If there's no pattern then the part can be used as a supplier. Otherwise it cannot.
              let pairs = value_raw.split(' ');
              for s in pairs {
                let pairs = s.split(',');
                for name_untrimmed in pairs {
                  let name = name_untrimmed.trim();
                  if name != "" {
                    nodes[current_node_index].pattern_by_name.push(name.trim().to_string());
                  }
                }
              }
            }
            "machine" => {
              // Machine specification for a woop part
              // The format is `- machine: [\dx\d] [name]`, like `- machine 3x3 Asset_Machine`
              // Otherwise it defaults to the smallest dimension that fits the input count (later)
              // Each dim will have its own default asset too, in case the name is not given

              let mut split = value_raw.split(' ');

              let mut chunk = split.next().or(Some("")).unwrap().trim();
              let mut bytes = chunk.bytes();
              let w = bytes.next().or(Some('?' as u8)).unwrap() as char;
              let x = bytes.next().or(Some('?' as u8)).unwrap() as char;
              let h = bytes.next().or(Some('?' as u8)).unwrap() as char;

              if (w >= '0' && w <= '9') && x == 'x' && (h >= '0' && h <= '9') {
                chunk = split.next_back().or(Some("")).unwrap().trim();

                nodes[current_node_index].machine_width = (w as u8 - '0' as u8) as usize;
                nodes[current_node_index].machine_height = (h as u8 - '0' as u8) as usize;
              }

              // Either
              // - There were two chunks. Then `chunk` will contain the second chunk now
              // - There was one chunk and it was a dimension, `chunk` will be the empty string now
              // - There was one chunk and it was not a dimension, `chunk` will be the first chunk

              if chunk != "" {
                nodes[current_node_index].machine_asset_name = chunk.to_string(); // Validate later, when we parsed all the configs, otherwise the node name may not bbe available yet.
              }
            }
            "char" => {
              // The icon
              // Only accept a-zA-Z and nothing else
              // For entries that have no "char" field (but need it), we'll auto-assign an icon
              let c = value_raw.bytes().next().or(Some('?' as u8)).unwrap() as char;
              nodes[current_node_index].icon = c;
              if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) {
                panic!("Attempted to parse a char icon that was not a-zA-Z: name = {}, icon = `{}` ({})", nodes[current_node_index].raw_name, c, c as u8);
              }
            }
            "file" => {
              // The sprite file
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].file = value_raw.trim().to_string();
            }
            "drm" => {
              // Mark the art for this node as "not owned", use with options.show_drm=false to create public media
              nodes[current_node_index].drm = true;
            }
            "special" => {
              // This part is one of the four special kinds. The "special" should be followed by
              // a colon, one of the four special types (espv) and a digit indicating the power level
              // By default parts have a power level of zero, which means they are not special.
              // Their symbol will be n (none). We don't need to validate the character here :shrug:

              let pair = value_raw.trim();
              if pair != "" {
                let split = pair.trim().split(' ').collect::<Vec<&str>>();

                let kind = split[0]; // Ignore multiple spaces between, :shrug:
                let mut kind =
                  if kind == "" { 'n' }
                  else if kind == "n" { 'n' }
                  else if kind == "N" { 'n' }
                  else if kind == "e" { 'e' }
                  else if kind == "E" { 'e' }
                  else if kind == "s" { 's' }
                  else if kind == "S" { 's' }
                  else if kind == "p" { 'p' }
                  else if kind == "P" { 'p' }
                  else if kind == "v" { 'v' }
                  else if kind == "V" { 'v' }
                  else { 'n' };
                if kind == 'n' {
                  if trace_parse_config_md { log!("Parsed a special kind `n`, which means none. Might be intentional, might be a parse error... Current node: {}, input value: `{}`", nodes[current_node_index].raw_name, value_raw); }
                } else if kind != 'e' && kind != 's' && kind != 'p' && kind != 'v' {
                  if trace_parse_config_md { log!("Parsed an unknown special kind, which defaults to none. Might be intentional, might be a parse error... Current node: {}, input value: `{}`", nodes[current_node_index].raw_name, value_raw); }
                  kind = 'n';
                }

                let power_level_str = split[split.len() - 1].trim();
                let power_level = power_level_str.parse::<u8>().or::<Result<u8, &str>>(Ok(0u8)).unwrap();
                if power_level == 0 {
                  if trace_parse_config_md { log!("Parsed a special level of zero. Maybe this was a bug or a parse error... Current node: {}, input value: `{}`", nodes[current_node_index].raw_name, value_raw); }
                }

                if trace_parse_config_md { log!("Parsing special: `{}` into `{:?}` -> `{}` and `{}`", pair, split, kind, power_level); }

                nodes[current_node_index].special = ( kind, power_level );
              }
            }
            | "part_x"
            | "x"
            => {
              // x coord in the sprite file where this sprite begins
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].x = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            | "part_y"
            | "y"
            => {
              // y coord in the sprite file where this sprite begins
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].y = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            | "part_w"
            | "w"
            => {
              // width in the sprite file of this sprite
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].w = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            | "part_h"
            | "h"
            => {
              // height in the sprite file of this sprite
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].h = value_raw.parse::<f64>().or::<Result<u32, &str>>(Ok(0.0)).unwrap();
            }
            "quest_status" => {
              if trace_parse_config_md { log!("Forcing quest status for `{}` to {}", nodes[current_node_index].name, value_raw); }
              match value_raw {
                "active" => nodes[current_node_index].quest_init_status = QuestStatus::Active,
                "bouncing" => nodes[current_node_index].quest_init_status = QuestStatus::Bouncing,
                "fading_and_bouncing" => nodes[current_node_index].quest_init_status = QuestStatus::FadingAndBouncing,
                "finished" => nodes[current_node_index].quest_init_status = QuestStatus::Finished,
                "waiting" => nodes[current_node_index].quest_init_status = QuestStatus::Waiting,
                _ => panic!("Only valid quest status allowed; Expecting one if 'active', 'bouncing', 'fading_and_bouncing', 'finished', or 'waiting', got: {}", value_raw),
              }
            }
            "frame" => {
              if first_frame {
                // Ignore. Keep the only frame that's already in this sprite and overwrite parts of it.
                first_frame = false;
              } else {
                // Clone the last frame and push it as a new frame.
                let last = nodes[current_node_index].sprite_config.frames.len() - 1;
                let c = nodes[current_node_index].sprite_config.frames[last].clone();
                nodes[current_node_index].sprite_config.frames.push(c);
              }
              let last = nodes[current_node_index].sprite_config.frames.len() - 1;
              nodes[current_node_index].sprite_config.frames[last].name = value_raw.to_string();
            }
            "frame_offset" => {
              nodes[current_node_index].sprite_config.frame_offset = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap() as usize;
            }
            "frame_count" => {
              // The total number of frames this sprite is expected to have
              let count = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
              nodes[current_node_index].sprite_config.frame_count = count;
            }
            "frame_direction" => {
              // The direction towards which the frames should iterate from the first (up, right, down, left)
              let dir = match value_raw.trim() {
                "up" => SpriteConfigDirection::Up,
                "right" => SpriteConfigDirection::Right,
                "down" => SpriteConfigDirection::Down,
                "left" => SpriteConfigDirection::Left,
                _ => panic!("The config value for `frame_direction` should be one of up, right, down, or left. It was: `{}`", value_raw.trim()),
              };
              nodes[current_node_index].sprite_config.frame_direction = dir;
            }
            "frame_delay" => {
              nodes[current_node_index].sprite_config.frame_delay = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
            }
            "looping" => {
              nodes[current_node_index].sprite_config.looping = value_raw == "true";
            }
            "loop_delay" => {
              nodes[current_node_index].sprite_config.loop_delay = value_raw.parse::<u64>().or::<Result<u32, &str>>(Ok(0)).unwrap();
            }
            "loop_backwards" => {
              nodes[current_node_index].sprite_config.loop_backwards = value_raw == "true";
            }
            "title" => {
              // For story lines
            },
            "author" => {
              // For story lines
            },
            "desc" => {
              // For story lines
            },
            "unused" => {
              // Do not load this image. Do not include in sprite map when running spriter.
              nodes[current_node_index].unused = true;
            }
            "gen" => {
              // This is generated (by the Spriter, presumably).
              // Remove the currently parsed frames.
              // TODO: keep them around for debugging purposes, like viewing "raw" frames

              first_frame = true;
              // Reset frames to initial state
              nodes[current_node_index].sprite_config.frames.clear();
              nodes[current_node_index].sprite_config.frames.push(SpriteFrame {
                file: "".to_string(),
                name: "untitled frame".to_string(),
                file_canvas_cache_index: 0,
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0
              });
            }
            _ => panic!("Unsupported node option. Node options must be one of a hard coded set but was `{:?}`", label),
          }
        }
        _ => {
          // comment
          if trace_parse_config_md { log!("This line is considered comment because it does not start with a hash or dash: {:?}", trimmed); }
        }
      }
    }
  );
  if trace_parse_config_md {
    log!("Last node was: {:?}", nodes[nodes.len()-1]);

    log!("+ Have {} stories:", stories.len());
    stories.iter().enumerate().for_each(|(story_index, story)| {
      log!("  - Story {} ({}), has {} quests", story_index, nodes[story.story_node_index].name, story.quest_nodes.len());
      story.quest_nodes.iter().enumerate().for_each(|(i, &quest_node_index)| {
        log!("    - Quest node {}: {:?}", i, nodes[quest_node_index]);
      });
    });

    log!("Stories: {:?}", stories);
  }

  // So now we have a serial list of nodes but we need to create a hierarchical tree from them
  // We create two models; one is a tree and the other a hashmap

  // Extrapolate SpriteConfig frames to match specified frame count
  if trace_parse_config_md { log!("+ Extrapolate all sprite animations"); }
  nodes.iter_mut().enumerate().for_each(|(i, node)| {
    let len = node.sprite_config.frames.len();
    let to_add = node.sprite_config.frame_count as i32 - node.sprite_config.frames.len() as i32;
    if to_add > 0 {
      // Take details from last frame and add more frames with same details except their offset
      // updated to match the frame_direction.

      let delta_x =
        if node.sprite_config.frame_direction == SpriteConfigDirection::Left { -node.sprite_config.frames[len - 1].w }
        else if node.sprite_config.frame_direction == SpriteConfigDirection::Right { node.sprite_config.frames[len - 1].w }
        else { 0.0 };
      let delta_y =
        if node.sprite_config.frame_direction == SpriteConfigDirection::Up { -node.sprite_config.frames[len - 1].h }
        else if node.sprite_config.frame_direction == SpriteConfigDirection::Down { node.sprite_config.frames[len - 1].h }
        else { 0.0 };

      if trace_parse_config_md { log!("Extrapolating {} more frames for node {} from {} to {} with delta x: {}, y: {}", to_add, i, len, node.sprite_config.frame_count, delta_x, delta_y); }

      for i in 0..to_add {
        let len = node.sprite_config.frames.len();
        let mut c = node.sprite_config.frames[len - 1].clone();
        c.x += delta_x;
        c.y += delta_y;
        node.sprite_config.frames.push(c);
      }
    }
  });

  // Map (fully qualified) name to index on config nodes. Also maps icon chars and &ord icons.
  let mut node_name_to_index = HashMap::new();
  // Create unique index for each unique sprite map url
  let mut sprite_cache_lookup = HashMap::new();
  let mut sprite_cache_order = vec!();

  // Generate lookup table name -> node_index
  nodes.iter_mut().enumerate().for_each(|(i, node)| {
    node_name_to_index.insert(node.raw_name.clone(), i);
    // Add mapping to icon char, too
    if node.icon != '?' {
      node_name_to_index.insert(node.icon.to_string(), i);
      // And a mapping to the &ord notation
      node_name_to_index.insert(format!("&{}", node.icon as u8), i);
    }
  });
  // Now assign the nodes without an icon
  // Downside is that this assignment isn't stable and any changes to config order may screw this up
  // In particular with respects to maps with initial starting configs
  // We could use raw names rather than icons for that tho
  if trace_parse_config_md { log!("+ Assign icons to parts without icons"); }
  nodes.iter_mut().enumerate().for_each(|(i, node)| {
    if node.kind != ConfigNodeKind::Part { return; } // Only relevant for parts
    if node.icon != '?' { return; } // Only assign unassigned ones
    for icon in 'a' as u8 ..= 'z' as u8 {
      let cstring = format!("{}", icon as char);
      if !node_name_to_index.contains_key(&cstring) {
        node.icon = icon as char;
        node_name_to_index.insert(cstring, i);
        if trace_parse_config_md { log!("  - config node {} was assigned icon `{}`", node.name, node.icon); }
        return; // -> continue with next node
      }
    }
    for icon in 'A' as u8 ..= 'Z' as u8 {
      let cstring = format!("{}", icon as char);
      if !node_name_to_index.contains_key(&cstring) {
        node.icon = icon as char;
        node_name_to_index.insert(cstring, i);
        if trace_parse_config_md { log!("  - config node {} was assigned icon `{}`", node.name, node.icon); }
        return; // -> continue with next node
      }
    }
    for icon in 128 as u8 ..= 253 as u8 {
      let cstring = format!("{}", icon as char);
      if !node_name_to_index.contains_key(&cstring) {
        node.icon = icon as char;
        node_name_to_index.insert(cstring, i);
        if trace_parse_config_md { log!("  - config node {} was assigned icon `{}` ({})", node.name, node.icon, icon); }
        return; // -> continue with next node
      }
    }

    // Mostly an icon limitation. But sadly an important one.
    panic!("oh no, ran out of space :'( can define up to 170 parts");
  });

  if trace_parse_config_md { log!("+ Create part pattern_by_index tables"); }
  for i in 0..nodes.len() {
    let node = &mut nodes[i];

    if trace_parse_config_md { log!("{}: {}: pattern_by_name= {:?}", i, nodes[i].raw_name, nodes[i].pattern_by_name); }

    // First resolve the target index, regardless of whether the name or icon was used
    let pattern_by_index =
      nodes[i].pattern_by_name.iter()
        .map(|name| {
          let mut t = name.as_str().clone();
          if t == "." || t == "_"{
            // In patterns we use . or _ to represent nothing, which translates to the empty/none part
            t = " ";
          }
          return *node_name_to_index.get(t).unwrap_or_else(| | panic!("pattern_by_name to index: what happened here: unlock name=`{}` of names=`{:?}`", name, node_name_to_index.keys()))
        })
        .collect::<Vec<PartKind>>();
    let pattern_by_index = machine_normalize_wants(&pattern_by_index);

    // Clean up the pattern by name (should only be used for parsing and debugging)
    nodes[i].pattern_by_name = pattern_by_index.iter().map(|&kind| nodes[kind].name.clone()).collect::<Vec<String>>();
    // Should only be used for debugging and serialization
    nodes[i].pattern_by_icon = pattern_by_index.iter().map(|&index| nodes[index].icon).collect::<Vec<char>>();
    nodes[i].pattern_by_icon.sort();

    // Sort by char to normalize it
    let pattern_str = nodes[i].pattern_by_icon.iter().collect::<String>();
    let pattern_str = str::replace(&pattern_str, " ", "");
    let pattern_str = str::replace(&pattern_str, ".", "");
    nodes[i].pattern = pattern_str;

    // Get all unique required parts, convert them to their icon, order them, create a string
    // If we do the same for the machines then we can do string comparisons.
    let mut kinds = pattern_by_index.clone();
    kinds.dedup();
    if trace_parse_config_md { log!("{}: {}: len={} kinds= {:?}", i, nodes[i].raw_name, pattern_by_index.len(), kinds); }
    nodes[i].pattern_unique_kinds = kinds;
    nodes[i].pattern_by_index = pattern_by_index;
  }

  if trace_parse_config_md { log!("+ Resolving part machine customizations"); }
  for i in 0..nodes.len() {
    let node = &mut nodes[i];

    assert!(node.machine_width <= 9);
    assert!(node.machine_height <= 9);

    let len = node.pattern_unique_kinds.len();
    if len > 0 && (node.machine_width == 0 || node.machine_height == 0) {
      // Auto discover
      if len <= 6 { // 1x2 2x1
        node.machine_width = 1;
        node.machine_height = 2;
      } else if len <= 8 { // 1x3 2x2 3x1
        node.machine_width = 2;
        node.machine_height = 2;
      } else if len <= 10 { // 2x3 3x2 1x4 4x1
        node.machine_width = 2;
        node.machine_height = 3;
      } else if len <= 12 { // 2x4 4x2 3x3 1x5 5x1
        node.machine_width = 3;
        node.machine_height = 3;
      } else if len <= 14 { // 1x6 6x1 2x5 5x2 3x4 4x3
        node.machine_width = 3;
        node.machine_height = 4;
      } else if len <= 16 { // 1x7 7x1 2x6 6x2 3x5 5x3 4x4
        node.machine_width = 4;
        node.machine_height = 4;
      } else if len <= 18 { // 1x8 8x1 2x7 7x2 3x6 6x3 4x5 5x4
        node.machine_width = 4;
        node.machine_height = 5;
      } else if len <= 20 { // 1x9 9x1 2x8 8x2 3x7 7x3 4x6 6x4 5x5
        node.machine_width = 5;
        node.machine_height = 5;
      } else {
        // Biggest machine :shrug: You'll have to share inputs.
        node.machine_width = 5;
        node.machine_height = 5;
      }

      if trace_parse_config_md { log!("  - {} defined the machine size to {}x{}", node.raw_name, node.machine_width, node.machine_height); }
    }

    if node.machine_asset_name == "" {
      log!("  - Warning: {} had an empty machine asset name, defaulting to 3x3 asset", node.raw_name);
      node.machine_asset_index = CONFIG_NODE_ASSET_MACHINE_3_3;
    } else if node.machine_asset_name == "Asset_Machine_3_3" {
      node.machine_asset_index = CONFIG_NODE_ASSET_MACHINE_3_3;
    } else if let Some(&node_index) = full_name_to_node_index.get(&node.machine_asset_name) {
      if trace_parse_config_md { log!("  - {} had {} which resolves to index {}", node.raw_name, node.machine_asset_name, node.machine_asset_index); }
      node.machine_asset_index = node_index;
    } else {
      log!("  - Warning: {} had `{}` which could not be resolved, using defaults [{:?}] [{:?}]", node.raw_name, node.machine_asset_name, node_name_to_index.get(&node.machine_asset_name.clone()), node_name_to_index.get(&"Asset_Machine_2_2".to_string()));
      node.machine_asset_name = "Asset_Machine_3_3".to_string();
      node.machine_asset_index = CONFIG_NODE_ASSET_MACHINE_3_3;
    }
  }

  if trace_parse_config_md { log!("+ Create quest unlocks_after_by_index and starting_part_by_index pointers"); }
  stories.iter().for_each(|story| {
    if trace_parse_config_md { log!("  - Story: {}", nodes[story.story_node_index].name); }

    story.quest_nodes.iter().for_each(|&node_index| {
      if trace_parse_config_md { log!("    ++ quest node index = {}, name = {}, unlocks after = `{:?}`", node_index, nodes[node_index].name, nodes[node_index].unlocks_after_by_name); }

      // Note: the indicdes must be ordered for binary_search later
      let mut indices_ordered: Vec<usize> = vec!();
      nodes[node_index].unlocks_after_by_name.iter().for_each(|name| {
        indices_ordered.push(*node_name_to_index.get(name.as_str().clone()).unwrap_or_else(| | panic!("parent_quest_name to index: what happened here: unlock name=`{}` of names=`{:?}`", name, node_name_to_index.keys())));
      });

      // Make sure the set is ordered
      indices_ordered.sort();

      nodes[node_index].unlocks_after_by_index = indices_ordered.clone(); // Never depletes
      // Note: this set may be reduced below when the quest initial status is determined
      nodes[node_index].unlocks_todo_by_index = indices_ordered; // This one depletes as quests are configured to start as finished (at parse time, not runtime). When this vec is empty, this quest starts available (if not finished).

      let mut indices: Vec<usize> = vec!();
      nodes[node_index].starting_part_by_name.iter().for_each(|name| {
        indices.push(*node_name_to_index.get(name.as_str().clone()).unwrap_or_else(| | panic!("starting_part_name to index: what happened here: part name=`{} of names=`{:?}`", name, node_name_to_index.keys())));
      });
      nodes[node_index].starting_part_by_index = indices;

      let mut indices: Vec<(u32, PartKind)> = vec!();
      nodes[node_index].production_target_by_name.iter().for_each(|(count, name)| {
        let index = *node_name_to_index.get(name.as_str().clone()).unwrap_or_else(| | panic!("production_target_name to index: what happened here: unlock name=`{} of names=`{:?}`", name, node_name_to_index.keys()));
        indices.push((*count, index));
      });
      nodes[node_index].production_target_by_index = indices;
    });
  });

  if trace_parse_config_md {
    log!("+ Have a total of {} config nodes:", nodes.len());
    nodes.iter_mut().enumerate().for_each(|(i, node)| {
      log!("- node {} is {}, kind={:?}", i, node.raw_name, node.kind);
    });
  }

  if trace_parse_config_md { log!("+ prepare unique sprite map pointers"); }
  nodes.iter_mut().enumerate().for_each(|(i, node)| {
    if i == 0 || node.name == "None" {
      // Do not add a sprite map for the None part; we should never be painting it.
      return;
    }
    if node.unused {
      // Do not load frames marked as unused
      return;
    }

    // Create sprite image index pointers. There will be an array with a canvas loaded with
    // that image and it will sit in a vector at the position of that index.
    node.sprite_config.frames.iter_mut().for_each(|frame| {
      let f = frame.file.as_str();
      match sprite_cache_lookup.get(f) {
        Some(&index) => {
          frame.file_canvas_cache_index = index;
        },
        None => {
          let index = sprite_cache_lookup.len();
          let file = frame.file.clone();
          frame.file_canvas_cache_index = index;
          sprite_cache_lookup.insert(file.clone(), index);
          sprite_cache_order.push(file);
        }
      }
    });
  });

  if trace_parse_config_md { log!("+ Initialize the quest node statuses and quest todos"); }
  stories.iter().for_each(|story| {
    if trace_parse_config_md { log!("- Story: {}", nodes[story.story_node_index].raw_name); }

    let mut again= true;
    let mut loop_i = 0;
    while again {
      again = false;
      loop_i += 1;

      if trace_parse_config_md { log!("  + {} #{}; Initialize the quest node statuses", nodes[story.story_node_index].raw_name, loop_i); }
      story.quest_nodes.iter().for_each(|&quest_index| {
        if trace_parse_config_md { log!("    - state loop"); }
        // If the config specified an initial state then just roll with that
        let mut changed = true;
        while changed {
          if trace_parse_config_md { log!("      - state inner loop"); }
          // Repeat the process until there's no further changes. This loop is guaranteed to halt.
          changed = false;
          if
            nodes[quest_index].quest_init_status == QuestStatus::Waiting &&
            nodes[quest_index].unlocks_after_by_index.iter().all(|&other_index| nodes[other_index].quest_init_status == QuestStatus::Finished)
          {
            if trace_parse_config_md { log!("        - Quest `{}` is available because `{:?}` are all finished", nodes[quest_index].raw_name, nodes[quest_index].unlocks_after_by_name); }
            nodes[quest_index].unlocks_todo_by_index.clear();
            nodes[quest_index].quest_init_status = QuestStatus::Active;
            changed = true;
          }
        }
      });

      if trace_parse_config_md { log!("  + {} #{}; Remove immediately finished quests from any quest todo that depends on it", loop_i, nodes[story.story_node_index].raw_name); }
      for quest_index in 0..story.quest_nodes.len() {
        let quest_node_index = story.quest_nodes[quest_index];
        if nodes[quest_node_index].quest_init_status != QuestStatus::Waiting && nodes[quest_node_index].quest_init_status != QuestStatus::Active {
          if trace_parse_config_md { log!("    - Find quests blocked by quest {} / node {} ({}) because init status is {:?}", quest_index, quest_node_index, nodes[quest_node_index].raw_name, nodes[quest_node_index].quest_init_status); }
          for other_quest_index in 0..story.quest_nodes.len() {
            let other_quest_node_index = story.quest_nodes[other_quest_index];
            // if trace_parse_config_md { log!("      - Quest node {}, node index {} ({}): {:?} {:?}", other_quest_index, other_quest_node_index, nodes[other_quest_node_index].raw_name, nodes[other_quest_node_index].unlocks_after_by_name, nodes[other_quest_node_index].unlocks_after_by_index); }
            if trace_parse_config_md { if nodes[other_quest_node_index].unlocks_after_by_index.contains(&quest_node_index) { log!("      - Quest {} ({}) did depend on this quest... current todos: {:?}", other_quest_node_index, nodes[other_quest_node_index].raw_name, nodes[other_quest_node_index].unlocks_todo_by_index); } }
            // If the finished quest was still part of the todo list, remove it now. It may already have been removed.
            let pos = nodes[other_quest_node_index].unlocks_todo_by_index.binary_search(&quest_node_index);
            if let Ok(unlock_index) = pos {
              if trace_parse_config_md { log!("        - Removed {} from the quests todo for {} ({}), todos: {:?}", quest_node_index, other_quest_node_index, nodes[other_quest_node_index].raw_name, nodes[other_quest_node_index].unlocks_todo_by_index); }
              nodes[other_quest_node_index].unlocks_todo_by_index.remove(unlock_index);
              if trace_parse_config_md { log!("        - After removal: {:?}", nodes[other_quest_node_index].unlocks_todo_by_index); }
              if nodes[other_quest_node_index].unlocks_todo_by_index.len() == 0 {
                if trace_parse_config_md { log!("        - This means {} ({}) is no longer blocked. Initial status was {:?}", other_quest_node_index, nodes[other_quest_node_index].raw_name, nodes[other_quest_node_index].quest_init_status); }
                if nodes[other_quest_node_index].quest_init_status == QuestStatus::Waiting {
                  if trace_parse_config_md { log!("        - Quest {} ({}) was still waiting with nothing left to wait for so setting it to Active...", other_quest_node_index, nodes[other_quest_node_index].raw_name); }
                  nodes[other_quest_node_index].quest_init_status = QuestStatus::Active;
                  again = true; // Need to reevaluate the initial status now
                }
              }
            }
          }
        }
      }
    }
  });

  if trace_parse_config_md {
    log!("+ Collected available Quests and Parts from the start:");
    nodes.iter().for_each(|node| {
      if node.quest_init_status != QuestStatus::Waiting {
        match node.kind {
          ConfigNodeKind::Asset => {}
          ConfigNodeKind::Part => {}
          ConfigNodeKind::Quest => log!("  - Quest {}, quest init status: {:?}", node.raw_name, node.quest_init_status),
          ConfigNodeKind::Belt => {}
          ConfigNodeKind::Story => {}
        }
      }
    });
  }

  if trace_parse_config_md { log!("+ Determine unlock requirements for each quest"); }
  stories.iter_mut().for_each(|story| {
    if trace_parse_config_md {  log!("  - Story: {}", nodes[story.story_node_index].name); }
    story.quest_nodes.iter().for_each(|&quest_node_index| {
      // For all quests go through list of dependency quests and tag them as being their "unlock parent"
      let mut found = vec!(); // Work around chicken egg problem -> mutability of nodes
      nodes[quest_node_index].unlocks_after_by_index.iter().for_each(|pindex| found.push(*pindex));
      found.iter().for_each(|pindex| nodes[*pindex].required_by_quest_indexes.push(quest_node_index));
    });
    // story.2.iter().for_each(|&index| {
    //   log!("quest {} depends on {:?}", nodes[index].raw_name.clone(), nodes[index].required_by_quest_indexes.iter().map(|pindex| nodes[*pindex].raw_name.clone()).collect::<Vec<String>>());
    // });
  });

  let mut assets = 0;
  let mut parts = 0;
  let mut quests = 0;
  let mut belts = 0;
  let mut stories_count = 0;
  nodes.iter().for_each(|node| {
    match node.kind {
      ConfigNodeKind::Asset => assets += 1,
      ConfigNodeKind::Part => parts += 1,
      ConfigNodeKind::Quest => quests += 1,
      ConfigNodeKind::Belt => belts += 1,
      ConfigNodeKind::Story => stories_count += 1,
    }
  });

  log!("Config has {} nodes with: {} stories, {} assets, {} parts, {} quests, and {} belts", nodes.len(), stories_count, assets, parts, quests, belts);

  // log!("parsed nodes: {:?}", &nodes[1..]);
  if trace_parse_config_md { log!("parsed map: {:?}", node_name_to_index); }
  if trace_parse_config_md { log!("active_story_index final => {}", active_story_index); }

  return Config {
    nodes,
    stories,
    initial_active_story_index: active_story_index,
    node_name_to_index,
    sprite_cache_lookup,
    sprite_cache_order,
    sprite_cache_canvas: vec!(),
    sprite_cache_loading: false,
  };
}

fn config_full_node_name_to_target_index(name: &str, kind: &str, def_index: usize) -> usize {
  return match name {
    "Asset_WeeWoo" => CONFIG_NODE_ASSET_WEE_WOO,
    "Asset_MissingInputs" => CONFIG_NODE_ASSET_MISSING_INPUTS,
    "Asset_MissingOutputs" => CONFIG_NODE_ASSET_MISSING_OUTPUTS,
    "Asset_MissingPurpose" => CONFIG_NODE_ASSET_MISSING_PURPOSE,
    "Asset_Machine_1_1" => CONFIG_NODE_ASSET_MACHINE_1_1,
    "Asset_Machine_1_2" => CONFIG_NODE_ASSET_MACHINE_1_2,
    "Asset_Machine_1_3" => CONFIG_NODE_ASSET_MACHINE_1_3,
    "Asset_Machine_1_4" => CONFIG_NODE_ASSET_MACHINE_1_4,
    "Asset_Machine_2_1" => CONFIG_NODE_ASSET_MACHINE_2_1,
    "Asset_Machine_2_2" => CONFIG_NODE_ASSET_MACHINE_2_2,
    "Asset_Machine_2_3" => CONFIG_NODE_ASSET_MACHINE_2_3,
    "Asset_Machine_2_4" => CONFIG_NODE_ASSET_MACHINE_2_4,
    "Asset_Machine_3_1" => CONFIG_NODE_ASSET_MACHINE_3_1,
    "Asset_Machine_3_2" => CONFIG_NODE_ASSET_MACHINE_3_2,
    "Asset_Machine_3_3" => CONFIG_NODE_ASSET_MACHINE_3_3,
    "Asset_Machine_3_4" => CONFIG_NODE_ASSET_MACHINE_3_4,
    "Asset_Machine_4_1" => CONFIG_NODE_ASSET_MACHINE_4_1,
    "Asset_Machine_4_2" => CONFIG_NODE_ASSET_MACHINE_4_2,
    "Asset_Machine_4_3" => CONFIG_NODE_ASSET_MACHINE_4_3,
    "Asset_Machine_4_4" => CONFIG_NODE_ASSET_MACHINE_4_4,
    "Asset_Factory" => CONFIG_NODE_ASSET_FACTORY,
    "Asset_Machine_Fallback" => CONFIG_NODE_ASSET_MACHINE_FALLBACK,
    "Asset_DumpTruck" => CONFIG_NODE_ASSET_DUMP_TRUCK,
    "Asset_Sand" => CONFIG_NODE_ASSET_SAND,
    "Asset_HelpBlack" => CONFIG_NODE_ASSET_HELP_BLACK,
    "Asset_HelpWhite" => CONFIG_NODE_ASSET_HELP_WHITE,
    "Asset_HelpGrey" => CONFIG_NODE_ASSET_HELP_GREY,
    "Asset_HelpRed" => CONFIG_NODE_ASSET_HELP_RED,
    "Asset_Manual" => CONFIG_NODE_ASSET_MANUAL,
    "Asset_Lmb" => CONFIG_NODE_ASSET_LMB,
    "Asset_Rmb" => CONFIG_NODE_ASSET_RMB,
    "Asset_SaveDark" => CONFIG_NODE_ASSET_SAVE_DARK,
    "Asset_SaveLight" => CONFIG_NODE_ASSET_SAVE_LIGHT,
    "Asset_SaveGrey" => CONFIG_NODE_ASSET_SAVE_GREY,
    "Asset_TrashDark" => CONFIG_NODE_ASSET_TRASH_DARK,
    "Asset_TrashLight" => CONFIG_NODE_ASSET_TRASH_LIGHT,
    "Asset_TrashGrey" => CONFIG_NODE_ASSET_TRASH_GREY,
    "Asset_TrashRed" => CONFIG_NODE_ASSET_TRASH_RED,
    "Asset_TrashGreen" => CONFIG_NODE_ASSET_TRASH_GREEN,
    "Asset_QuestFrame" => CONFIG_NODE_ASSET_QUEST_FRAME,
    "Asset_DoubleArrowRight" => CONFIG_NODE_ASSET_DOUBLE_ARROW_RIGHT,
    "Asset_ScreenLoader" => CONFIG_NODE_ASSET_SCREEN_LOADER,
    "Asset_ScreenPlay" => CONFIG_NODE_ASSET_SCREEN_PLAY,
    "Asset_ButtonUp1" => CONFIG_NODE_ASSET_BUTTON_UP_1,
    "Asset_ButtonUp2" => CONFIG_NODE_ASSET_BUTTON_UP_2,
    "Asset_ButtonUp3" => CONFIG_NODE_ASSET_BUTTON_UP_3,
    "Asset_ButtonUp4" => CONFIG_NODE_ASSET_BUTTON_UP_4,
    "Asset_ButtonUp6" => CONFIG_NODE_ASSET_BUTTON_UP_6,
    "Asset_ButtonUp7" => CONFIG_NODE_ASSET_BUTTON_UP_7,
    "Asset_ButtonUp8" => CONFIG_NODE_ASSET_BUTTON_UP_8,
    "Asset_ButtonUp9" => CONFIG_NODE_ASSET_BUTTON_UP_9,
    "Asset_ButtonDown1" => CONFIG_NODE_ASSET_BUTTON_DOWN_1,
    "Asset_ButtonDown2" => CONFIG_NODE_ASSET_BUTTON_DOWN_2,
    "Asset_ButtonDown3" => CONFIG_NODE_ASSET_BUTTON_DOWN_3,
    "Asset_ButtonDown4" => CONFIG_NODE_ASSET_BUTTON_DOWN_4,
    "Asset_ButtonDown6" => CONFIG_NODE_ASSET_BUTTON_DOWN_6,
    "Asset_ButtonDown7" => CONFIG_NODE_ASSET_BUTTON_DOWN_7,
    "Asset_ButtonDown8" => CONFIG_NODE_ASSET_BUTTON_DOWN_8,
    "Asset_ButtonDown9" => CONFIG_NODE_ASSET_BUTTON_DOWN_9,
    "Asset_SingleArrowDown" => CONFIG_NODE_ASSET_SINGLE_ARROW_DOWN,
    "Asset_SingleArrowRight" => CONFIG_NODE_ASSET_SINGLE_ARROW_RIGHT,
    "Asset_Treasure" => CONFIG_NDOE_ASSET_TREASURE,
    "Asset_Pickaxe" => CONFIG_NODE_ASSET_PICKAXE,
    "Asset_DrmPlaceholder" => CONFIG_NODE_ASSET_DRM_PLACEHOLDER,
    "Asset_BrushDark" => CONFIG_NODE_ASSET_BRUSH_DARK,
    "Asset_BrushLight" => CONFIG_NODE_ASSET_BRUSH_LIGHT,
    "Asset_BrushRed" => CONFIG_NODE_ASSET_BRUSH_RED,
    "Asset_BrushGreen" => CONFIG_NODE_ASSET_BRUSH_GREEN,
    "Asset_BrushGrey" => CONFIG_NODE_ASSET_BRUSH_GREY,
    "Asset_UndoLight" => CONFIG_NODE_ASSET_UNDO_LIGHT,
    "Asset_UndoGrey" => CONFIG_NODE_ASSET_UNDO_GREY,
    "Asset_RedoLight" => CONFIG_NODE_ASSET_REDO_LIGHT,
    "Asset_RedoGrey" => CONFIG_NODE_ASSET_REDO_GREY,
    "Asset_Logo" => CONFIG_NODE_ASSET_LOGO,
    "Asset_CopyWhite" => CONFIG_NODE_ASSET_COPY_WHITE,
    "Asset_CopyGrey" => CONFIG_NODE_ASSET_COPY_GREY,
    "Asset_CopyGreen" => CONFIG_NODE_ASSET_COPY_GREEN,
    "Asset_PasteWhite" => CONFIG_NODE_ASSET_PASTE_WHITE,
    "Asset_PasteGrey" => CONFIG_NODE_ASSET_PASTE_GREY,
    "Asset_PasteGreen" => CONFIG_NODE_ASSET_PASTE_GREEN,
    "Asset_FullScreenBlack" => CONFIG_NODE_ASSET_FULLSCREEN_BLACK,
    "Asset_FullScreenWhite" => CONFIG_NODE_ASSET_FULLSCREEN_WHITE,
    "Asset_FullScreenGrey" => CONFIG_NODE_ASSET_FULLSCREEN_GREY,
    "Asset_PlayBlack" => CONFIG_NODE_ASSET_PLAY_BLACK,
    "Asset_PlayWhite" => CONFIG_NODE_ASSET_PLAY_WHITE,
    "Asset_PlayGrey" => CONFIG_NODE_ASSET_PLAY_GREY,
    "Asset_FwdBlack" => CONFIG_NODE_ASSET_FWD_BLACK,
    "Asset_FwdWhite" => CONFIG_NODE_ASSET_FWD_WHITE,
    "Asset_FwdGrey" => CONFIG_NODE_ASSET_FWD_GREY,
    "Asset_FastFwdBlack" => CONFIG_NODE_ASSET_FAST_FWD_BLACK,
    "Asset_FastFwdWhite" => CONFIG_NODE_ASSET_FAST_FWD_WHITE,
    "Asset_FastFwdGrey" => CONFIG_NODE_ASSET_FAST_FWD_GREY,
    "Asset_BwdBlack" => CONFIG_NODE_ASSET_BWD_BLACK,
    "Asset_BwdWhite" => CONFIG_NODE_ASSET_BWD_WHITE,
    "Asset_BwdGrey" => CONFIG_NODE_ASSET_BWD_GREY,
    "Asset_FastBwdBlack" => CONFIG_NODE_ASSET_FAST_BWD_BLACK,
    "Asset_FastBwdWhite" => CONFIG_NODE_ASSET_FAST_BWD_WHITE,
    "Asset_FastBwdGrey" => CONFIG_NODE_ASSET_FAST_BWD_GREY,
    "Asset_StopBlack" => CONFIG_NODE_ASSET_STOP_BLACK,
    "Asset_StopWhite" => CONFIG_NODE_ASSET_STOP_WHITE,
    "Asset_StopGrey" => CONFIG_NODE_ASSET_STOP_GREY,
    "Asset_PauseBlack" => CONFIG_NODE_ASSET_PAUSE_BLACK,
    "Asset_PauseWhite" => CONFIG_NODE_ASSET_PAUSE_WHITE,
    "Asset_PauseGrey" => CONFIG_NODE_ASSET_PAUSE_GREY,
    "Asset_Battery" => CONFIG_NODE_ASSET_BATTERY,
    "Part_None" => CONFIG_NODE_PART_NONE,
    "Part_Trash" => CONFIG_NODE_PART_TRASH,
    "Asset_SupplyUp" => CONFIG_NODE_ASSET_SUPPLY_UP,
    "Asset_SupplyRight" => CONFIG_NODE_ASSET_SUPPLY_RIGHT,
    "Asset_SupplyDown" => CONFIG_NODE_ASSET_SUPPLY_DOWN,
    "Asset_SupplyLeft" => CONFIG_NODE_ASSET_SUPPLY_LEFT,
    "Asset_DemandUp" => CONFIG_NODE_ASSET_DEMAND_UP,
    "Asset_DemandRight" => CONFIG_NODE_ASSET_DEMAND_RIGHT,
    "Asset_DemandDown" => CONFIG_NODE_ASSET_DEMAND_DOWN,
    "Asset_DemandLeft" => CONFIG_NODE_ASSET_DEMAND_LEFT,
    "Asset_DockUp" => CONFIG_NODE_ASSET_DOCK_UP,
    "Asset_DockRight" => CONFIG_NODE_ASSET_DOCK_RIGHT,
    "Asset_DockDown" => CONFIG_NODE_ASSET_DOCK_DOWN,
    "Asset_DockLeft" => CONFIG_NODE_ASSET_DOCK_LEFT,
    "Belt_None" => CONFIG_NODE_BELT_NONE,
    "Belt_Unknown" => CONFIG_NODE_BELT_UNKNOWN,
    "Belt_Invalid" => CONFIG_NODE_BELT_INVALID,
    "Belt_L_" => CONFIG_NODE_BELT_L_,
    "Belt__L" => CONFIG_NODE_BELT__L,
    "Belt___L" => CONFIG_NODE_BELT___L,
    "Belt_D_" => CONFIG_NODE_BELT_D_,
    "Belt_DL_" => CONFIG_NODE_BELT_DL_,
    "Belt_D_L" => CONFIG_NODE_BELT_D_L,
    "Belt_D__L" => CONFIG_NODE_BELT_D__L,
    "Belt__D" => CONFIG_NODE_BELT__D,
    "Belt_L_D" => CONFIG_NODE_BELT_L_D,
    "Belt__DL" => CONFIG_NODE_BELT__DL,
    "Belt__D_L" => CONFIG_NODE_BELT__D_L,
    "Belt___D" => CONFIG_NODE_BELT___D,
    "Belt_L__D" => CONFIG_NODE_BELT_L__D,
    "Belt__L_D" => CONFIG_NODE_BELT__L_D,
    "Belt___DL" => CONFIG_NODE_BELT___DL,
    "Belt_R_" => CONFIG_NODE_BELT_R_,
    "Belt_LR_" => CONFIG_NODE_BELT_LR_,
    "Belt_R_L" => CONFIG_NODE_BELT_R_L,
    "Belt_R__L" => CONFIG_NODE_BELT_R__L,
    "Belt_DR_" => CONFIG_NODE_BELT_DR_,
    "Belt_DLR_" => CONFIG_NODE_BELT_DLR_,
    "Belt_DR_L" => CONFIG_NODE_BELT_DR_L,
    "Belt_DR__L" => CONFIG_NODE_BELT_DR__L,
    "Belt_R_D" => CONFIG_NODE_BELT_R_D,
    "Belt_LR_D" => CONFIG_NODE_BELT_LR_D,
    "Belt_R_DL" => CONFIG_NODE_BELT_R_DL,
    "Belt_R_D_L" => CONFIG_NODE_BELT_R_D_L,
    "Belt_R__D" => CONFIG_NODE_BELT_R__D,
    "Belt_LR__D" => CONFIG_NODE_BELT_LR__D,
    "Belt_R_L_D" => CONFIG_NODE_BELT_R_L_D,
    "Belt_R__DL" => CONFIG_NODE_BELT_R__DL,
    "Belt__R" => CONFIG_NODE_BELT__R,
    "Belt_L_R" => CONFIG_NODE_BELT_L_R,
    "Belt__LR" => CONFIG_NODE_BELT__LR,
    "Belt__R_L" => CONFIG_NODE_BELT__R_L,
    "Belt_D_R" => CONFIG_NODE_BELT_D_R,
    "Belt_DL_R" => CONFIG_NODE_BELT_DL_R,
    "Belt_D_LR" => CONFIG_NODE_BELT_D_LR,
    "Belt_D_R_L" => CONFIG_NODE_BELT_D_R_L,
    "Belt__DR" => CONFIG_NODE_BELT__DR,
    "Belt_L_DR" => CONFIG_NODE_BELT_L_DR,
    "Belt__DLR" => CONFIG_NODE_BELT__DLR,
    "Belt__DR_L" => CONFIG_NODE_BELT__DR_L,
    "Belt__R_D" => CONFIG_NODE_BELT__R_D,
    "Belt_L_R_D" => CONFIG_NODE_BELT_L_R_D,
    "Belt__LR_D" => CONFIG_NODE_BELT__LR_D,
    "Belt__R_DL" => CONFIG_NODE_BELT__R_DL,
    "Belt___R" => CONFIG_NODE_BELT___R,
    "Belt_L__R" => CONFIG_NODE_BELT_L__R,
    "Belt__L_R" => CONFIG_NODE_BELT__L_R,
    "Belt___LR" => CONFIG_NODE_BELT___LR,
    "Belt_D__R" => CONFIG_NODE_BELT_D__R,
    "Belt_DL__R" => CONFIG_NODE_BELT_DL__R,
    "Belt_D_L_R" => CONFIG_NODE_BELT_D_L_R,
    "Belt_D__LR" => CONFIG_NODE_BELT_D__LR,
    "Belt__D_R" => CONFIG_NODE_BELT__D_R,
    "Belt_L_D_R" => CONFIG_NODE_BELT_L_D_R,
    "Belt__DL_R" => CONFIG_NODE_BELT__DL_R,
    "Belt__D_LR" => CONFIG_NODE_BELT__D_LR,
    "Belt___DR" => CONFIG_NODE_BELT___DR,
    "Belt_L__DR" => CONFIG_NODE_BELT_L__DR,
    "Belt__L_DR" => CONFIG_NODE_BELT__L_DR,
    "Belt___DLR" => CONFIG_NODE_BELT___DLR,
    "Belt_U_" => CONFIG_NODE_BELT_U_,
    "Belt_LU_" => CONFIG_NODE_BELT_LU_,
    "Belt_U_L" => CONFIG_NODE_BELT_U_L,
    "Belt_U__L" => CONFIG_NODE_BELT_U__L,
    "Belt_DU_" => CONFIG_NODE_BELT_DU_,
    "Belt_DLU_" => CONFIG_NODE_BELT_DLU_,
    "Belt_DU_L" => CONFIG_NODE_BELT_DU_L,
    "Belt_DU__L" => CONFIG_NODE_BELT_DU__L,
    "Belt_U_D" => CONFIG_NODE_BELT_U_D,
    "Belt_LU_D" => CONFIG_NODE_BELT_LU_D,
    "Belt_U_DL" => CONFIG_NODE_BELT_U_DL,
    "Belt_U_D_L" => CONFIG_NODE_BELT_U_D_L,
    "Belt_U__D" => CONFIG_NODE_BELT_U__D,
    "Belt_LU__D" => CONFIG_NODE_BELT_LU__D,
    "Belt_U_L_D" => CONFIG_NODE_BELT_U_L_D,
    "Belt_U__DL" => CONFIG_NODE_BELT_U__DL,
    "Belt_RU_" => CONFIG_NODE_BELT_RU_,
    "Belt_LRU_" => CONFIG_NODE_BELT_LRU_,
    "Belt_RU_L" => CONFIG_NODE_BELT_RU_L,
    "Belt_RU__L" => CONFIG_NODE_BELT_RU__L,
    "Belt_DRU_" => CONFIG_NODE_BELT_DRU_,
    "Belt_DLRU_" => CONFIG_NODE_BELT_DLRU_,
    "Belt_DRU_L" => CONFIG_NODE_BELT_DRU_L,
    "Belt_DRU__L" => CONFIG_NODE_BELT_DRU__L,
    "Belt_RU_D" => CONFIG_NODE_BELT_RU_D,
    "Belt_LRU_D" => CONFIG_NODE_BELT_LRU_D,
    "Belt_RU_DL" => CONFIG_NODE_BELT_RU_DL,
    "Belt_RU_D_L" => CONFIG_NODE_BELT_RU_D_L,
    "Belt_RU__D" => CONFIG_NODE_BELT_RU__D,
    "Belt_LRU__D" => CONFIG_NODE_BELT_LRU__D,
    "Belt_RU_L_D" => CONFIG_NODE_BELT_RU_L_D,
    "Belt_RU__DL" => CONFIG_NODE_BELT_RU__DL,
    "Belt_U_R" => CONFIG_NODE_BELT_U_R,
    "Belt_LU_R" => CONFIG_NODE_BELT_LU_R,
    "Belt_U_LR" => CONFIG_NODE_BELT_U_LR,
    "Belt_U_R_L" => CONFIG_NODE_BELT_U_R_L,
    "Belt_DU_R" => CONFIG_NODE_BELT_DU_R,
    "Belt_DLU_R" => CONFIG_NODE_BELT_DLU_R,
    "Belt_DU_LR" => CONFIG_NODE_BELT_DU_LR,
    "Belt_DU_R_L" => CONFIG_NODE_BELT_DU_R_L,
    "Belt_U_DR" => CONFIG_NODE_BELT_U_DR,
    "Belt_LU_DR" => CONFIG_NODE_BELT_LU_DR,
    "Belt_U_DLR" => CONFIG_NODE_BELT_U_DLR,
    "Belt_U_DR_L" => CONFIG_NODE_BELT_U_DR_L,
    "Belt_U_R_D" => CONFIG_NODE_BELT_U_R_D,
    "Belt_LU_R_D" => CONFIG_NODE_BELT_LU_R_D,
    "Belt_U_LR_D" => CONFIG_NODE_BELT_U_LR_D,
    "Belt_U_R_DL" => CONFIG_NODE_BELT_U_R_DL,
    "Belt_U__R" => CONFIG_NODE_BELT_U__R,
    "Belt_LU__R" => CONFIG_NODE_BELT_LU__R,
    "Belt_U_L_R" => CONFIG_NODE_BELT_U_L_R,
    "Belt_U__LR" => CONFIG_NODE_BELT_U__LR,
    "Belt_DU__R" => CONFIG_NODE_BELT_DU__R,
    "Belt_DLU__R" => CONFIG_NODE_BELT_DLU__R,
    "Belt_DU_L_R" => CONFIG_NODE_BELT_DU_L_R,
    "Belt_DU__LR" => CONFIG_NODE_BELT_DU__LR,
    "Belt_U_D_R" => CONFIG_NODE_BELT_U_D_R,
    "Belt_LU_D_R" => CONFIG_NODE_BELT_LU_D_R,
    "Belt_U_DL_R" => CONFIG_NODE_BELT_U_DL_R,
    "Belt_U_D_LR" => CONFIG_NODE_BELT_U_D_LR,
    "Belt_U__DR" => CONFIG_NODE_BELT_U__DR,
    "Belt_LU__DR" => CONFIG_NODE_BELT_LU__DR,
    "Belt_U_L_DR" => CONFIG_NODE_BELT_U_L_DR,
    "Belt_U__DLR" => CONFIG_NODE_BELT_U__DLR,
    "Belt__U" => CONFIG_NODE_BELT__U,
    "Belt_L_U" => CONFIG_NODE_BELT_L_U,
    "Belt__LU" => CONFIG_NODE_BELT__LU,
    "Belt__U_L" => CONFIG_NODE_BELT__U_L,
    "Belt_D_U" => CONFIG_NODE_BELT_D_U,
    "Belt_DL_U" => CONFIG_NODE_BELT_DL_U,
    "Belt_D_LU" => CONFIG_NODE_BELT_D_LU,
    "Belt_D_U_L" => CONFIG_NODE_BELT_D_U_L,
    "Belt__DU" => CONFIG_NODE_BELT__DU,
    "Belt_L_DU" => CONFIG_NODE_BELT_L_DU,
    "Belt__DLU" => CONFIG_NODE_BELT__DLU,
    "Belt__DU_L" => CONFIG_NODE_BELT__DU_L,
    "Belt__U_D" => CONFIG_NODE_BELT__U_D,
    "Belt_L_U_D" => CONFIG_NODE_BELT_L_U_D,
    "Belt__LU_D" => CONFIG_NODE_BELT__LU_D,
    "Belt__U_DL" => CONFIG_NODE_BELT__U_DL,
    "Belt_R_U" => CONFIG_NODE_BELT_R_U,
    "Belt_LR_U" => CONFIG_NODE_BELT_LR_U,
    "Belt_R_LU" => CONFIG_NODE_BELT_R_LU,
    "Belt_R_U_L" => CONFIG_NODE_BELT_R_U_L,
    "Belt_DR_U" => CONFIG_NODE_BELT_DR_U,
    "Belt_DLR_U" => CONFIG_NODE_BELT_DLR_U,
    "Belt_DR_LU" => CONFIG_NODE_BELT_DR_LU,
    "Belt_DR_U_L" => CONFIG_NODE_BELT_DR_U_L,
    "Belt_R_DU" => CONFIG_NODE_BELT_R_DU,
    "Belt_LR_DU" => CONFIG_NODE_BELT_LR_DU,
    "Belt_R_DLU" => CONFIG_NODE_BELT_R_DLU,
    "Belt_R_DU_L" => CONFIG_NODE_BELT_R_DU_L,
    "Belt_R_U_D" => CONFIG_NODE_BELT_R_U_D,
    "Belt_LR_U_D" => CONFIG_NODE_BELT_LR_U_D,
    "Belt_R_LU_D" => CONFIG_NODE_BELT_R_LU_D,
    "Belt_R_U_DL" => CONFIG_NODE_BELT_R_U_DL,
    "Belt__RU" => CONFIG_NODE_BELT__RU,
    "Belt_L_RU" => CONFIG_NODE_BELT_L_RU,
    "Belt__LRU" => CONFIG_NODE_BELT__LRU,
    "Belt__RU_L" => CONFIG_NODE_BELT__RU_L,
    "Belt_D_RU" => CONFIG_NODE_BELT_D_RU,
    "Belt_DL_RU" => CONFIG_NODE_BELT_DL_RU,
    "Belt_D_LRU" => CONFIG_NODE_BELT_D_LRU,
    "Belt_D_RU_L" => CONFIG_NODE_BELT_D_RU_L,
    "Belt__DRU" => CONFIG_NODE_BELT__DRU,
    "Belt_L_DRU" => CONFIG_NODE_BELT_L_DRU,
    "Belt__DLRU" => CONFIG_NODE_BELT__DLRU,
    "Belt__DRU_L" => CONFIG_NODE_BELT__DRU_L,
    "Belt__RU_D" => CONFIG_NODE_BELT__RU_D,
    "Belt_L_RU_D" => CONFIG_NODE_BELT_L_RU_D,
    "Belt__LRU_D" => CONFIG_NODE_BELT__LRU_D,
    "Belt__RU_DL" => CONFIG_NODE_BELT__RU_DL,
    "Belt__U_R" => CONFIG_NODE_BELT__U_R,
    "Belt_L_U_R" => CONFIG_NODE_BELT_L_U_R,
    "Belt__LU_R" => CONFIG_NODE_BELT__LU_R,
    "Belt__U_LR" => CONFIG_NODE_BELT__U_LR,
    "Belt_D_U_R" => CONFIG_NODE_BELT_D_U_R,
    "Belt_DL_U_R" => CONFIG_NODE_BELT_DL_U_R,
    "Belt_D_LU_R" => CONFIG_NODE_BELT_D_LU_R,
    "Belt_D_U_LR" => CONFIG_NODE_BELT_D_U_LR,
    "Belt__DU_R" => CONFIG_NODE_BELT__DU_R,
    "Belt_L_DU_R" => CONFIG_NODE_BELT_L_DU_R,
    "Belt__DLU_R" => CONFIG_NODE_BELT__DLU_R,
    "Belt__DU_LR" => CONFIG_NODE_BELT__DU_LR,
    "Belt__U_DR" => CONFIG_NODE_BELT__U_DR,
    "Belt_L_U_DR" => CONFIG_NODE_BELT_L_U_DR,
    "Belt__LU_DR" => CONFIG_NODE_BELT__LU_DR,
    "Belt__U_DLR" => CONFIG_NODE_BELT__U_DLR,
    "Belt___U" => CONFIG_NODE_BELT___U,
    "Belt_L__U" => CONFIG_NODE_BELT_L__U,
    "Belt__L_U" => CONFIG_NODE_BELT__L_U,
    "Belt___LU" => CONFIG_NODE_BELT___LU,
    "Belt_D__U" => CONFIG_NODE_BELT_D__U,
    "Belt_DL__U" => CONFIG_NODE_BELT_DL__U,
    "Belt_D_L_U" => CONFIG_NODE_BELT_D_L_U,
    "Belt_D__LU" => CONFIG_NODE_BELT_D__LU,
    "Belt__D_U" => CONFIG_NODE_BELT__D_U,
    "Belt_L_D_U" => CONFIG_NODE_BELT_L_D_U,
    "Belt__DL_U" => CONFIG_NODE_BELT__DL_U,
    "Belt__D_LU" => CONFIG_NODE_BELT__D_LU,
    "Belt___DU" => CONFIG_NODE_BELT___DU,
    "Belt_L__DU" => CONFIG_NODE_BELT_L__DU,
    "Belt__L_DU" => CONFIG_NODE_BELT__L_DU,
    "Belt___DLU" => CONFIG_NODE_BELT___DLU,
    "Belt_R__U" => CONFIG_NODE_BELT_R__U,
    "Belt_LR__U" => CONFIG_NODE_BELT_LR__U,
    "Belt_R_L_U" => CONFIG_NODE_BELT_R_L_U,
    "Belt_R__LU" => CONFIG_NODE_BELT_R__LU,
    "Belt_DR__U" => CONFIG_NODE_BELT_DR__U,
    "Belt_DLR__U" => CONFIG_NODE_BELT_DLR__U,
    "Belt_DR_L_U" => CONFIG_NODE_BELT_DR_L_U,
    "Belt_DR__LU" => CONFIG_NODE_BELT_DR__LU,
    "Belt_R_D_U" => CONFIG_NODE_BELT_R_D_U,
    "Belt_LR_D_U" => CONFIG_NODE_BELT_LR_D_U,
    "Belt_R_DL_U" => CONFIG_NODE_BELT_R_DL_U,
    "Belt_R_D_LU" => CONFIG_NODE_BELT_R_D_LU,
    "Belt_R__DU" => CONFIG_NODE_BELT_R__DU,
    "Belt_LR__DU" => CONFIG_NODE_BELT_LR__DU,
    "Belt_R_L_DU" => CONFIG_NODE_BELT_R_L_DU,
    "Belt_R__DLU" => CONFIG_NODE_BELT_R__DLU,
    "Belt__R_U" => CONFIG_NODE_BELT__R_U,
    "Belt_L_R_U" => CONFIG_NODE_BELT_L_R_U,
    "Belt__LR_U" => CONFIG_NODE_BELT__LR_U,
    "Belt__R_LU" => CONFIG_NODE_BELT__R_LU,
    "Belt_D_R_U" => CONFIG_NODE_BELT_D_R_U,
    "Belt_DL_R_U" => CONFIG_NODE_BELT_DL_R_U,
    "Belt_D_LR_U" => CONFIG_NODE_BELT_D_LR_U,
    "Belt_D_R_LU" => CONFIG_NODE_BELT_D_R_LU,
    "Belt__DR_U" => CONFIG_NODE_BELT__DR_U,
    "Belt_L_DR_U" => CONFIG_NODE_BELT_L_DR_U,
    "Belt__DLR_U" => CONFIG_NODE_BELT__DLR_U,
    "Belt__DR_LU" => CONFIG_NODE_BELT__DR_LU,
    "Belt__R_DU" => CONFIG_NODE_BELT__R_DU,
    "Belt_L_R_DU" => CONFIG_NODE_BELT_L_R_DU,
    "Belt__LR_DU" => CONFIG_NODE_BELT__LR_DU,
    "Belt__R_DLU" => CONFIG_NODE_BELT__R_DLU,
    "Belt___RU" => CONFIG_NODE_BELT___RU,
    "Belt_L__RU" => CONFIG_NODE_BELT_L__RU,
    "Belt__L_RU" => CONFIG_NODE_BELT__L_RU,
    "Belt___LRU" => CONFIG_NODE_BELT___LRU,
    "Belt_D__RU" => CONFIG_NODE_BELT_D__RU,
    "Belt_DL__RU" => CONFIG_NODE_BELT_DL__RU,
    "Belt_D_L_RU" => CONFIG_NODE_BELT_D_L_RU,
    "Belt_D__LRU" => CONFIG_NODE_BELT_D__LRU,
    "Belt__D_RU" => CONFIG_NODE_BELT__D_RU,
    "Belt_L_D_RU" => CONFIG_NODE_BELT_L_D_RU,
    "Belt__DL_RU" => CONFIG_NODE_BELT__DL_RU,
    "Belt__D_LRU" => CONFIG_NODE_BELT__D_LRU,
    "Belt___DRU" => CONFIG_NODE_BELT___DRU,
    "Belt_L__DRU" => CONFIG_NODE_BELT_L__DRU,
    "Belt__L_DRU" => CONFIG_NODE_BELT__L_DRU,
    "Belt___DLRU" => CONFIG_NODE_BELT___DLRU,
    _ => {
      if !name.starts_with("Part_") && !name.starts_with("Quest_") && !name.starts_with("Story_") {
        log!("Warning: {} did not match a known node name and was not Quest, Story, or Part! assigning fresh index: {}", name, def_index);
      }
      if kind != "Part" && kind != "Quest" && !name.starts_with("Story_") {
        panic!("Only expecting parts and quests to be of unknown node types. Detected kind as `{}` for `{}`", kind, name);
      }
      // If not known then return the next index (nodes.len())
      def_index
    },
  };
}

fn get_system_nodes() -> Vec<ConfigNode> {
  // These are default nodes that you can still override like any other node in the config

  let v = vec!(
    config_node_part(CONFIG_NODE_PART_NONE, "None".to_string(), ' '),
    config_node_part(CONFIG_NODE_PART_TRASH, "Trash".to_string(), 't'),
    config_node_asset(CONFIG_NODE_ASSET_SUPPLY_UP, "SupplyUp"),
    config_node_asset(CONFIG_NODE_ASSET_SUPPLY_RIGHT, "SupplyRight"),
    config_node_asset(CONFIG_NODE_ASSET_SUPPLY_DOWN, "SupplyDown"),
    config_node_asset(CONFIG_NODE_ASSET_SUPPLY_LEFT, "SupplyLeft"),
    config_node_asset(CONFIG_NODE_ASSET_DEMAND_UP, "DemandUp"),
    config_node_asset(CONFIG_NODE_ASSET_DEMAND_RIGHT, "DemandRight"),
    config_node_asset(CONFIG_NODE_ASSET_DEMAND_DOWN, "DemandDown"),
    config_node_asset(CONFIG_NODE_ASSET_DEMAND_LEFT, "DemandLeft"),
    config_node_asset(CONFIG_NODE_ASSET_DOCK_UP, "DockUp"),
    config_node_asset(CONFIG_NODE_ASSET_DOCK_RIGHT, "DockRight"),
    config_node_asset(CONFIG_NODE_ASSET_DOCK_DOWN, "DockDown"),
    config_node_asset(CONFIG_NODE_ASSET_DOCK_LEFT, "DockLeft"),
    config_node_part(CONFIG_NODE_MACHINE_1X1, "None".to_string(), ' '), // obsolete
    config_node_part(CONFIG_NODE_MACHINE_2X2, "None".to_string(), ' '), // obsolete
    config_node_part(CONFIG_NODE_MACHINE_3X3, "None".to_string(), ' '), // obsolete
    config_node_belt(CONFIG_NODE_BELT_NONE, "None"),
    config_node_belt(CONFIG_NODE_BELT_UNKNOWN, "Unknown"),
    config_node_belt(CONFIG_NODE_BELT_INVALID, "Invalid"),
    config_node_belt(CONFIG_NODE_BELT_L_, "L_"),
    config_node_belt(CONFIG_NODE_BELT__L, "_L"),
    config_node_belt(CONFIG_NODE_BELT___L, "__L"),
    config_node_belt(CONFIG_NODE_BELT_D_, "D_"),
    config_node_belt(CONFIG_NODE_BELT_DL_, "DL_"),
    config_node_belt(CONFIG_NODE_BELT_D_L, "D_L"),
    config_node_belt(CONFIG_NODE_BELT_D__L, "D__L"),
    config_node_belt(CONFIG_NODE_BELT__D, "_D"),
    config_node_belt(CONFIG_NODE_BELT_L_D, "L_D"),
    config_node_belt(CONFIG_NODE_BELT__DL, "_DL"),
    config_node_belt(CONFIG_NODE_BELT__D_L, "_D_L"),
    config_node_belt(CONFIG_NODE_BELT___D, "__D"),
    config_node_belt(CONFIG_NODE_BELT_L__D, "L__D"),
    config_node_belt(CONFIG_NODE_BELT__L_D, "_L_D"),
    config_node_belt(CONFIG_NODE_BELT___DL, "__DL"),
    config_node_belt(CONFIG_NODE_BELT_R_, "R_"),
    config_node_belt(CONFIG_NODE_BELT_LR_, "LR_"),
    config_node_belt(CONFIG_NODE_BELT_R_L, "R_L"),
    config_node_belt(CONFIG_NODE_BELT_R__L, "R__L"),
    config_node_belt(CONFIG_NODE_BELT_DR_, "DR_"),
    config_node_belt(CONFIG_NODE_BELT_DLR_, "DLR_"),
    config_node_belt(CONFIG_NODE_BELT_DR_L, "DR_L"),
    config_node_belt(CONFIG_NODE_BELT_DR__L, "DR__L"),
    config_node_belt(CONFIG_NODE_BELT_R_D, "R_D"),
    config_node_belt(CONFIG_NODE_BELT_LR_D, "LR_D"),
    config_node_belt(CONFIG_NODE_BELT_R_DL, "R_DL"),
    config_node_belt(CONFIG_NODE_BELT_R_D_L, "R_D_L"),
    config_node_belt(CONFIG_NODE_BELT_R__D, "R__D"),
    config_node_belt(CONFIG_NODE_BELT_LR__D, "LR__D"),
    config_node_belt(CONFIG_NODE_BELT_R_L_D, "R_L_D"),
    config_node_belt(CONFIG_NODE_BELT_R__DL, "R__DL"),
    config_node_belt(CONFIG_NODE_BELT__R, "_R"),
    config_node_belt(CONFIG_NODE_BELT_L_R, "L_R"),
    config_node_belt(CONFIG_NODE_BELT__LR, "_LR"),
    config_node_belt(CONFIG_NODE_BELT__R_L, "_R_L"),
    config_node_belt(CONFIG_NODE_BELT_D_R, "D_R"),
    config_node_belt(CONFIG_NODE_BELT_DL_R, "DL_R"),
    config_node_belt(CONFIG_NODE_BELT_D_LR, "D_LR"),
    config_node_belt(CONFIG_NODE_BELT_D_R_L, "D_R_L"),
    config_node_belt(CONFIG_NODE_BELT__DR, "_DR"),
    config_node_belt(CONFIG_NODE_BELT_L_DR, "L_DR"),
    config_node_belt(CONFIG_NODE_BELT__DLR, "_DLR"),
    config_node_belt(CONFIG_NODE_BELT__DR_L, "_DR_L"),
    config_node_belt(CONFIG_NODE_BELT__R_D, "_R_D"),
    config_node_belt(CONFIG_NODE_BELT_L_R_D, "L_R_D"),
    config_node_belt(CONFIG_NODE_BELT__LR_D, "_LR_D"),
    config_node_belt(CONFIG_NODE_BELT__R_DL, "_R_DL"),
    config_node_belt(CONFIG_NODE_BELT___R, "__R"),
    config_node_belt(CONFIG_NODE_BELT_L__R, "L__R"),
    config_node_belt(CONFIG_NODE_BELT__L_R, "_L_R"),
    config_node_belt(CONFIG_NODE_BELT___LR, "__LR"),
    config_node_belt(CONFIG_NODE_BELT_D__R, "D__R"),
    config_node_belt(CONFIG_NODE_BELT_DL__R, "DL__R"),
    config_node_belt(CONFIG_NODE_BELT_D_L_R, "D_L_R"),
    config_node_belt(CONFIG_NODE_BELT_D__LR, "D__LR"),
    config_node_belt(CONFIG_NODE_BELT__D_R, "_D_R"),
    config_node_belt(CONFIG_NODE_BELT_L_D_R, "L_D_R"),
    config_node_belt(CONFIG_NODE_BELT__DL_R, "_DL_R"),
    config_node_belt(CONFIG_NODE_BELT__D_LR, "_D_LR"),
    config_node_belt(CONFIG_NODE_BELT___DR, "__DR"),
    config_node_belt(CONFIG_NODE_BELT_L__DR, "L__DR"),
    config_node_belt(CONFIG_NODE_BELT__L_DR, "_L_DR"),
    config_node_belt(CONFIG_NODE_BELT___DLR, "__DLR"),
    config_node_belt(CONFIG_NODE_BELT_U_, "U_"),
    config_node_belt(CONFIG_NODE_BELT_LU_, "LU_"),
    config_node_belt(CONFIG_NODE_BELT_U_L, "U_L"),
    config_node_belt(CONFIG_NODE_BELT_U__L, "U__L"),
    config_node_belt(CONFIG_NODE_BELT_DU_, "DU_"),
    config_node_belt(CONFIG_NODE_BELT_DLU_, "DLU_"),
    config_node_belt(CONFIG_NODE_BELT_DU_L, "DU_L"),
    config_node_belt(CONFIG_NODE_BELT_DU__L, "DU__L"),
    config_node_belt(CONFIG_NODE_BELT_U_D, "U_D"),
    config_node_belt(CONFIG_NODE_BELT_LU_D, "LU_D"),
    config_node_belt(CONFIG_NODE_BELT_U_DL, "U_DL"),
    config_node_belt(CONFIG_NODE_BELT_U_D_L, "U_D_L"),
    config_node_belt(CONFIG_NODE_BELT_U__D, "U__D"),
    config_node_belt(CONFIG_NODE_BELT_LU__D, "LU__D"),
    config_node_belt(CONFIG_NODE_BELT_U_L_D, "U_L_D"),
    config_node_belt(CONFIG_NODE_BELT_U__DL, "U__DL"),
    config_node_belt(CONFIG_NODE_BELT_RU_, "RU_"),
    config_node_belt(CONFIG_NODE_BELT_LRU_, "LRU_"),
    config_node_belt(CONFIG_NODE_BELT_RU_L, "RU_L"),
    config_node_belt(CONFIG_NODE_BELT_RU__L, "RU__L"),
    config_node_belt(CONFIG_NODE_BELT_DRU_, "DRU_"),
    config_node_belt(CONFIG_NODE_BELT_DLRU_, "DLRU_"),
    config_node_belt(CONFIG_NODE_BELT_DRU_L, "DRU_L"),
    config_node_belt(CONFIG_NODE_BELT_DRU__L, "DRU__L"),
    config_node_belt(CONFIG_NODE_BELT_RU_D, "RU_D"),
    config_node_belt(CONFIG_NODE_BELT_LRU_D, "LRU_D"),
    config_node_belt(CONFIG_NODE_BELT_RU_DL, "RU_DL"),
    config_node_belt(CONFIG_NODE_BELT_RU_D_L, "RU_D_L"),
    config_node_belt(CONFIG_NODE_BELT_RU__D, "RU__D"),
    config_node_belt(CONFIG_NODE_BELT_LRU__D, "LRU__D"),
    config_node_belt(CONFIG_NODE_BELT_RU_L_D, "RU_L_D"),
    config_node_belt(CONFIG_NODE_BELT_RU__DL, "RU__DL"),
    config_node_belt(CONFIG_NODE_BELT_U_R, "U_R"),
    config_node_belt(CONFIG_NODE_BELT_LU_R, "LU_R"),
    config_node_belt(CONFIG_NODE_BELT_U_LR, "U_LR"),
    config_node_belt(CONFIG_NODE_BELT_U_R_L, "U_R_L"),
    config_node_belt(CONFIG_NODE_BELT_DU_R, "DU_R"),
    config_node_belt(CONFIG_NODE_BELT_DLU_R, "DLU_R"),
    config_node_belt(CONFIG_NODE_BELT_DU_LR, "DU_LR"),
    config_node_belt(CONFIG_NODE_BELT_DU_R_L, "DU_R_L"),
    config_node_belt(CONFIG_NODE_BELT_U_DR, "U_DR"),
    config_node_belt(CONFIG_NODE_BELT_LU_DR, "LU_DR"),
    config_node_belt(CONFIG_NODE_BELT_U_DLR, "U_DLR"),
    config_node_belt(CONFIG_NODE_BELT_U_DR_L, "U_DR_L"),
    config_node_belt(CONFIG_NODE_BELT_U_R_D, "U_R_D"),
    config_node_belt(CONFIG_NODE_BELT_LU_R_D, "LU_R_D"),
    config_node_belt(CONFIG_NODE_BELT_U_LR_D, "U_LR_D"),
    config_node_belt(CONFIG_NODE_BELT_U_R_DL, "U_R_DL"),
    config_node_belt(CONFIG_NODE_BELT_U__R, "U__R"),
    config_node_belt(CONFIG_NODE_BELT_LU__R, "LU__R"),
    config_node_belt(CONFIG_NODE_BELT_U_L_R, "U_L_R"),
    config_node_belt(CONFIG_NODE_BELT_U__LR, "U__LR"),
    config_node_belt(CONFIG_NODE_BELT_DU__R, "DU__R"),
    config_node_belt(CONFIG_NODE_BELT_DLU__R, "DLU__R"),
    config_node_belt(CONFIG_NODE_BELT_DU_L_R, "DU_L_R"),
    config_node_belt(CONFIG_NODE_BELT_DU__LR, "DU__LR"),
    config_node_belt(CONFIG_NODE_BELT_U_D_R, "U_D_R"),
    config_node_belt(CONFIG_NODE_BELT_LU_D_R, "LU_D_R"),
    config_node_belt(CONFIG_NODE_BELT_U_DL_R, "U_DL_R"),
    config_node_belt(CONFIG_NODE_BELT_U_D_LR, "U_D_LR"),
    config_node_belt(CONFIG_NODE_BELT_U__DR, "U__DR"),
    config_node_belt(CONFIG_NODE_BELT_LU__DR, "LU__DR"),
    config_node_belt(CONFIG_NODE_BELT_U_L_DR, "U_L_DR"),
    config_node_belt(CONFIG_NODE_BELT_U__DLR, "U__DLR"),
    config_node_belt(CONFIG_NODE_BELT__U, "_U"),
    config_node_belt(CONFIG_NODE_BELT_L_U, "L_U"),
    config_node_belt(CONFIG_NODE_BELT__LU, "_LU"),
    config_node_belt(CONFIG_NODE_BELT__U_L, "_U_L"),
    config_node_belt(CONFIG_NODE_BELT_D_U, "D_U"),
    config_node_belt(CONFIG_NODE_BELT_DL_U, "DL_U"),
    config_node_belt(CONFIG_NODE_BELT_D_LU, "D_LU"),
    config_node_belt(CONFIG_NODE_BELT_D_U_L, "D_U_L"),
    config_node_belt(CONFIG_NODE_BELT__DU, "_DU"),
    config_node_belt(CONFIG_NODE_BELT_L_DU, "L_DU"),
    config_node_belt(CONFIG_NODE_BELT__DLU, "_DLU"),
    config_node_belt(CONFIG_NODE_BELT__DU_L, "_DU_L"),
    config_node_belt(CONFIG_NODE_BELT__U_D, "_U_D"),
    config_node_belt(CONFIG_NODE_BELT_L_U_D, "L_U_D"),
    config_node_belt(CONFIG_NODE_BELT__LU_D, "_LU_D"),
    config_node_belt(CONFIG_NODE_BELT__U_DL, "_U_DL"),
    config_node_belt(CONFIG_NODE_BELT_R_U, "R_U"),
    config_node_belt(CONFIG_NODE_BELT_LR_U, "LR_U"),
    config_node_belt(CONFIG_NODE_BELT_R_LU, "R_LU"),
    config_node_belt(CONFIG_NODE_BELT_R_U_L, "R_U_L"),
    config_node_belt(CONFIG_NODE_BELT_DR_U, "DR_U"),
    config_node_belt(CONFIG_NODE_BELT_DLR_U, "DLR_U"),
    config_node_belt(CONFIG_NODE_BELT_DR_LU, "DR_LU"),
    config_node_belt(CONFIG_NODE_BELT_DR_U_L, "DR_U_L"),
    config_node_belt(CONFIG_NODE_BELT_R_DU, "R_DU"),
    config_node_belt(CONFIG_NODE_BELT_LR_DU, "LR_DU"),
    config_node_belt(CONFIG_NODE_BELT_R_DLU, "R_DLU"),
    config_node_belt(CONFIG_NODE_BELT_R_DU_L, "R_DU_L"),
    config_node_belt(CONFIG_NODE_BELT_R_U_D, "R_U_D"),
    config_node_belt(CONFIG_NODE_BELT_LR_U_D, "LR_U_D"),
    config_node_belt(CONFIG_NODE_BELT_R_LU_D, "R_LU_D"),
    config_node_belt(CONFIG_NODE_BELT_R_U_DL, "R_U_DL"),
    config_node_belt(CONFIG_NODE_BELT__RU, "_RU"),
    config_node_belt(CONFIG_NODE_BELT_L_RU, "L_RU"),
    config_node_belt(CONFIG_NODE_BELT__LRU, "_LRU"),
    config_node_belt(CONFIG_NODE_BELT__RU_L, "_RU_L"),
    config_node_belt(CONFIG_NODE_BELT_D_RU, "D_RU"),
    config_node_belt(CONFIG_NODE_BELT_DL_RU, "DL_RU"),
    config_node_belt(CONFIG_NODE_BELT_D_LRU, "D_LRU"),
    config_node_belt(CONFIG_NODE_BELT_D_RU_L, "D_RU_L"),
    config_node_belt(CONFIG_NODE_BELT__DRU, "_DRU"),
    config_node_belt(CONFIG_NODE_BELT_L_DRU, "L_DRU"),
    config_node_belt(CONFIG_NODE_BELT__DLRU, "_DLRU"),
    config_node_belt(CONFIG_NODE_BELT__DRU_L, "_DRU_L"),
    config_node_belt(CONFIG_NODE_BELT__RU_D, "_RU_D"),
    config_node_belt(CONFIG_NODE_BELT_L_RU_D, "L_RU_D"),
    config_node_belt(CONFIG_NODE_BELT__LRU_D, "_LRU_D"),
    config_node_belt(CONFIG_NODE_BELT__RU_DL, "_RU_DL"),
    config_node_belt(CONFIG_NODE_BELT__U_R, "_U_R"),
    config_node_belt(CONFIG_NODE_BELT_L_U_R, "L_U_R"),
    config_node_belt(CONFIG_NODE_BELT__LU_R, "_LU_R"),
    config_node_belt(CONFIG_NODE_BELT__U_LR, "_U_LR"),
    config_node_belt(CONFIG_NODE_BELT_D_U_R, "D_U_R"),
    config_node_belt(CONFIG_NODE_BELT_DL_U_R, "DL_U_R"),
    config_node_belt(CONFIG_NODE_BELT_D_LU_R, "D_LU_R"),
    config_node_belt(CONFIG_NODE_BELT_D_U_LR, "D_U_LR"),
    config_node_belt(CONFIG_NODE_BELT__DU_R, "_DU_R"),
    config_node_belt(CONFIG_NODE_BELT_L_DU_R, "L_DU_R"),
    config_node_belt(CONFIG_NODE_BELT__DLU_R, "_DLU_R"),
    config_node_belt(CONFIG_NODE_BELT__DU_LR, "_DU_LR"),
    config_node_belt(CONFIG_NODE_BELT__U_DR, "_U_DR"),
    config_node_belt(CONFIG_NODE_BELT_L_U_DR, "L_U_DR"),
    config_node_belt(CONFIG_NODE_BELT__LU_DR, "_LU_DR"),
    config_node_belt(CONFIG_NODE_BELT__U_DLR, "_U_DLR"),
    config_node_belt(CONFIG_NODE_BELT___U, "__U"),
    config_node_belt(CONFIG_NODE_BELT_L__U, "L__U"),
    config_node_belt(CONFIG_NODE_BELT__L_U, "_L_U"),
    config_node_belt(CONFIG_NODE_BELT___LU, "__LU"),
    config_node_belt(CONFIG_NODE_BELT_D__U, "D__U"),
    config_node_belt(CONFIG_NODE_BELT_DL__U, "DL__U"),
    config_node_belt(CONFIG_NODE_BELT_D_L_U, "D_L_U"),
    config_node_belt(CONFIG_NODE_BELT_D__LU, "D__LU"),
    config_node_belt(CONFIG_NODE_BELT__D_U, "_D_U"),
    config_node_belt(CONFIG_NODE_BELT_L_D_U, "L_D_U"),
    config_node_belt(CONFIG_NODE_BELT__DL_U, "_DL_U"),
    config_node_belt(CONFIG_NODE_BELT__D_LU, "_D_LU"),
    config_node_belt(CONFIG_NODE_BELT___DU, "__DU"),
    config_node_belt(CONFIG_NODE_BELT_L__DU, "L__DU"),
    config_node_belt(CONFIG_NODE_BELT__L_DU, "_L_DU"),
    config_node_belt(CONFIG_NODE_BELT___DLU, "__DLU"),
    config_node_belt(CONFIG_NODE_BELT_R__U, "R__U"),
    config_node_belt(CONFIG_NODE_BELT_LR__U, "LR__U"),
    config_node_belt(CONFIG_NODE_BELT_R_L_U, "R_L_U"),
    config_node_belt(CONFIG_NODE_BELT_R__LU, "R__LU"),
    config_node_belt(CONFIG_NODE_BELT_DR__U, "DR__U"),
    config_node_belt(CONFIG_NODE_BELT_DLR__U, "DLR__U"),
    config_node_belt(CONFIG_NODE_BELT_DR_L_U, "DR_L_U"),
    config_node_belt(CONFIG_NODE_BELT_DR__LU, "DR__LU"),
    config_node_belt(CONFIG_NODE_BELT_R_D_U, "R_D_U"),
    config_node_belt(CONFIG_NODE_BELT_LR_D_U, "LR_D_U"),
    config_node_belt(CONFIG_NODE_BELT_R_DL_U, "R_DL_U"),
    config_node_belt(CONFIG_NODE_BELT_R_D_LU, "R_D_LU"),
    config_node_belt(CONFIG_NODE_BELT_R__DU, "R__DU"),
    config_node_belt(CONFIG_NODE_BELT_LR__DU, "LR__DU"),
    config_node_belt(CONFIG_NODE_BELT_R_L_DU, "R_L_DU"),
    config_node_belt(CONFIG_NODE_BELT_R__DLU, "R__DLU"),
    config_node_belt(CONFIG_NODE_BELT__R_U, "_R_U"),
    config_node_belt(CONFIG_NODE_BELT_L_R_U, "L_R_U"),
    config_node_belt(CONFIG_NODE_BELT__LR_U, "_LR_U"),
    config_node_belt(CONFIG_NODE_BELT__R_LU, "_R_LU"),
    config_node_belt(CONFIG_NODE_BELT_D_R_U, "D_R_U"),
    config_node_belt(CONFIG_NODE_BELT_DL_R_U, "DL_R_U"),
    config_node_belt(CONFIG_NODE_BELT_D_LR_U, "D_LR_U"),
    config_node_belt(CONFIG_NODE_BELT_D_R_LU, "D_R_LU"),
    config_node_belt(CONFIG_NODE_BELT__DR_U, "_DR_U"),
    config_node_belt(CONFIG_NODE_BELT_L_DR_U, "L_DR_U"),
    config_node_belt(CONFIG_NODE_BELT__DLR_U, "_DLR_U"),
    config_node_belt(CONFIG_NODE_BELT__DR_LU, "_DR_LU"),
    config_node_belt(CONFIG_NODE_BELT__R_DU, "_R_DU"),
    config_node_belt(CONFIG_NODE_BELT_L_R_DU, "L_R_DU"),
    config_node_belt(CONFIG_NODE_BELT__LR_DU, "_LR_DU"),
    config_node_belt(CONFIG_NODE_BELT__R_DLU, "_R_DLU"),
    config_node_belt(CONFIG_NODE_BELT___RU, "__RU"),
    config_node_belt(CONFIG_NODE_BELT_L__RU, "L__RU"),
    config_node_belt(CONFIG_NODE_BELT__L_RU, "_L_RU"),
    config_node_belt(CONFIG_NODE_BELT___LRU, "__LRU"),
    config_node_belt(CONFIG_NODE_BELT_D__RU, "D__RU"),
    config_node_belt(CONFIG_NODE_BELT_DL__RU, "DL__RU"),
    config_node_belt(CONFIG_NODE_BELT_D_L_RU, "D_L_RU"),
    config_node_belt(CONFIG_NODE_BELT_D__LRU, "D__LRU"),
    config_node_belt(CONFIG_NODE_BELT__D_RU, "_D_RU"),
    config_node_belt(CONFIG_NODE_BELT_L_D_RU, "L_D_RU"),
    config_node_belt(CONFIG_NODE_BELT__DL_RU, "_DL_RU"),
    config_node_belt(CONFIG_NODE_BELT__D_LRU, "_D_LRU"),
    config_node_belt(CONFIG_NODE_BELT___DRU, "__DRU"),
    config_node_belt(CONFIG_NODE_BELT_L__DRU, "L__DRU"),
    config_node_belt(CONFIG_NODE_BELT__L_DRU, "_L_DRU"),
    config_node_belt(CONFIG_NODE_BELT___DLRU, "__DLRU"),
    config_node_asset(CONFIG_NODE_ASSET_WEE_WOO, "WeeWoo"),
    config_node_asset(CONFIG_NODE_ASSET_MISSING_INPUTS, "MissingInputs"),
    config_node_asset(CONFIG_NODE_ASSET_MISSING_OUTPUTS, "MissingOutputs"),
    config_node_asset(CONFIG_NODE_ASSET_MISSING_PURPOSE, "MissingPurpose"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_1_1, "Machine_1_1"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_1_2, "Machine_1_2"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_1_3, "Machine_1_3"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_1_4, "Machine_1_4"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_2_1, "Machine_2_1"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_2_2, "Machine_2_2"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_2_3, "Machine_2_3"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_2_4, "Machine_2_4"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_3_1, "Machine_3_1"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_3_2, "Machine_3_2"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_3_3, "Machine_3_3"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_3_4, "Machine_3_4"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_4_1, "Machine_4_1"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_4_2, "Machine_4_2"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_4_3, "Machine_4_3"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_4_4, "Machine_4_4"),
    config_node_asset(CONFIG_NODE_ASSET_MACHINE_FALLBACK, "Machine_Fallback"),
    config_node_asset(CONFIG_NODE_ASSET_DUMP_TRUCK, "DumpTruck"),
    config_node_asset(CONFIG_NODE_ASSET_SAND, "Sand"),
    config_node_asset(CONFIG_NODE_ASSET_HELP_BLACK, "HelpBlack"),
    config_node_asset(CONFIG_NODE_ASSET_HELP_RED, "HelpRed"),
    config_node_asset(CONFIG_NODE_ASSET_MANUAL, "Manual"),
    config_node_asset(CONFIG_NODE_ASSET_LMB, "Lmb"),
    config_node_asset(CONFIG_NODE_ASSET_RMB, "Rmb"),
    config_node_asset(CONFIG_NODE_ASSET_SAVE_DARK, "SaveDark"),
    config_node_asset(CONFIG_NODE_ASSET_QUEST_FRAME, "QuestFrame"),
    config_node_asset(CONFIG_NODE_ASSET_DOUBLE_ARROW_RIGHT, "DoubleArrowRight"),
    config_node_asset(CONFIG_NODE_ASSET_SINGLE_ARROW_DOWN, "SingleArrowDown"),
    config_node_asset(CONFIG_NODE_ASSET_SINGLE_ARROW_RIGHT, "SingleArrowRight"),
    config_node_asset(CONFIG_NODE_ASSET_SCREEN_LOADER, "ScreenLoader"),
    config_node_asset(CONFIG_NODE_ASSET_SCREEN_PLAY, "ScreenPlay"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_UP_1, "ButtonUp1"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_UP_2, "ButtonUp2"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_UP_3, "ButtonUp3"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_UP_4, "ButtonUp4"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_UP_6, "ButtonUp6"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_UP_7, "ButtonUp7"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_UP_8, "ButtonUp8"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_UP_9, "ButtonUp9"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_DOWN_1, "ButtonDown1"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_DOWN_2, "ButtonDown2"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_DOWN_3, "ButtonDown3"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_DOWN_4, "ButtonDown4"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_DOWN_6, "ButtonDown6"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_DOWN_7, "ButtonDown7"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_DOWN_8, "ButtonDown8"),
    config_node_asset(CONFIG_NODE_ASSET_BUTTON_DOWN_9, "ButtonDown9"),
    config_node_asset(CONFIG_NODE_ASSET_SAVE_LIGHT, "SaveLight"),
    config_node_asset(CONFIG_NODE_ASSET_SAVE_GREY, "SaveGrey"),
    config_node_asset(CONFIG_NODE_ASSET_TRASH_DARK, "TrashDark"),
    config_node_asset(CONFIG_NODE_ASSET_TRASH_LIGHT, "TrashLight"),
    config_node_asset(CONFIG_NODE_ASSET_TRASH_GREY, "TrashGrey"),
    config_node_asset(CONFIG_NODE_ASSET_TRASH_RED, "TrashRed"),
    config_node_asset(CONFIG_NODE_ASSET_TRASH_GREEN, "TrashGreen"),
    config_node_story(CONFIG_NODE_STORY_DEFAULT, "Default"),
    config_node_asset(CONFIG_NDOE_ASSET_TREASURE, "Treasure"),
    config_node_asset(CONFIG_NODE_ASSET_PICKAXE, "Pickaxe"),
    config_node_asset(CONFIG_NODE_ASSET_DRM_PLACEHOLDER, "DrmPlaceholder"),
    config_node_asset(CONFIG_NODE_ASSET_BRUSH_DARK, "BrushDark"),
    config_node_asset(CONFIG_NODE_ASSET_BRUSH_LIGHT, "BrushLight"),
    config_node_asset(CONFIG_NODE_ASSET_BRUSH_RED, "BrushRed"),
    config_node_asset(CONFIG_NODE_ASSET_BRUSH_GREEN, "BrushGreen"),
    config_node_asset(CONFIG_NODE_ASSET_BRUSH_GREY, "BrushGrey"),
    config_node_asset(CONFIG_NODE_ASSET_UNDO_LIGHT, "UndoLight"),
    config_node_asset(CONFIG_NODE_ASSET_UNDO_GREY, "UndoGrey"),
    config_node_asset(CONFIG_NODE_ASSET_REDO_LIGHT, "RedoLight"),
    config_node_asset(CONFIG_NODE_ASSET_REDO_GREY, "RedoGrey"),
    config_node_asset(CONFIG_NODE_ASSET_LOGO, "Logo"),
    config_node_asset(CONFIG_NODE_ASSET_COPY_GREY, "CopyGrey"),
    config_node_asset(CONFIG_NODE_ASSET_PASTE_GREY, "PasteGrey"),
    config_node_asset(CONFIG_NODE_ASSET_COPY_GREEN, "CopyGreen"),
    config_node_asset(CONFIG_NODE_ASSET_PASTE_GREEN, "PasteGreen"),
    config_node_asset(CONFIG_NODE_ASSET_FACTORY, "Factory"),
    config_node_asset(CONFIG_NODE_ASSET_FULLSCREEN_BLACK, "FullScreenBlack"),
    config_node_asset(CONFIG_NODE_ASSET_FULLSCREEN_WHITE, "FullScreenWhite"),
    config_node_asset(CONFIG_NODE_ASSET_FULLSCREEN_GREY, "FullScreenGrey"),
    config_node_asset(CONFIG_NODE_ASSET_PLAY_BLACK, "PlayBlack"),
    config_node_asset(CONFIG_NODE_ASSET_PLAY_WHITE, "PlayWhite"),
    config_node_asset(CONFIG_NODE_ASSET_PLAY_GREY, "PlayGrey"),
    config_node_asset(CONFIG_NODE_ASSET_FWD_BLACK, "FwdBlack"),
    config_node_asset(CONFIG_NODE_ASSET_FWD_WHITE, "FwdWhite"),
    config_node_asset(CONFIG_NODE_ASSET_FWD_GREY, "FwdGrey"),
    config_node_asset(CONFIG_NODE_ASSET_FAST_FWD_BLACK, "FastFwdBlack"),
    config_node_asset(CONFIG_NODE_ASSET_FAST_FWD_WHITE, "FastFwdWhite"),
    config_node_asset(CONFIG_NODE_ASSET_FAST_FWD_GREY, "FastFwdGrey"),
    config_node_asset(CONFIG_NODE_ASSET_BWD_BLACK, "BwdBlack"),
    config_node_asset(CONFIG_NODE_ASSET_BWD_WHITE, "BwdWhite"),
    config_node_asset(CONFIG_NODE_ASSET_BWD_GREY, "BwdGrey"),
    config_node_asset(CONFIG_NODE_ASSET_FAST_BWD_BLACK, "FastBwdBlack"),
    config_node_asset(CONFIG_NODE_ASSET_FAST_BWD_WHITE, "FastBwdWhite"),
    config_node_asset(CONFIG_NODE_ASSET_FAST_BWD_GREY, "FastBwdGrey"),
    config_node_asset(CONFIG_NODE_ASSET_STOP_BLACK, "StopBlack"),
    config_node_asset(CONFIG_NODE_ASSET_STOP_WHITE, "StopWhite"),
    config_node_asset(CONFIG_NODE_ASSET_STOP_GREY, "StopGrey"),
    config_node_asset(CONFIG_NODE_ASSET_PAUSE_BLACK, "PauseBlack"),
    config_node_asset(CONFIG_NODE_ASSET_PAUSE_WHITE, "PauseWhite"),
    config_node_asset(CONFIG_NODE_ASSET_PAUSE_GREY, "PauseGrey"),
    config_node_asset(CONFIG_NODE_ASSET_COPY_WHITE, "CopyWhite"),
    config_node_asset(CONFIG_NODE_ASSET_PASTE_WHITE, "PasteWhite"),
    config_node_asset(CONFIG_NODE_ASSET_HELP_WHITE, "HelpWhite"),
    config_node_asset(CONFIG_NODE_ASSET_HELP_GREY, "HelpGrey"),
    config_node_asset(CONFIG_NODE_ASSET_BATTERY, "Battery"),
  );

  v.iter().enumerate().for_each(|(i, node)| assert!(node.index == i, "system node indexes must match their global constant value; mismatch for index {} in get_system_nodes(), node.index= {}", i, node.index));

  return v;
}

pub fn config_get_initial_unlocks(options: &Options, state: &State, config: &Config, story_index: usize) -> Vec<PartKind> {
  if options.trace_quest_status { log!("config_get_initial_unlocks()"); }
  let mut unlocked_part_kinds: Vec<PartKind> = vec!();

  config.nodes.iter()
    .for_each(|node| {
      if node.story_index != story_index && node.story_index != 0 { return; }
      if node.kind != ConfigNodeKind::Quest { return; }
      if node.quest_init_status != QuestStatus::Active { return; }
      if options.trace_quest_status { log!("- quest {} ({}) story {} ({}); quest initially active", node.index, node.raw_name, node.story_index, config.nodes[config.stories[node.story_index].story_node_index].raw_name); }

      if options.trace_quest_status { log!("  - Adding node.starting_part_by_index {:?} to unlocks", node.starting_part_by_index); }
      node.starting_part_by_index.iter().for_each(|index| {
        if !unlocked_part_kinds.contains(index) { unlocked_part_kinds.push(*index); }
      });

      if options.trace_quest_status { log!("  - Adding node.production_target_by_index {:?} to unlocks", node.production_target_by_index.iter().map(|(_, p)|config.nodes[*p].raw_name.clone()).collect::<Vec<String>>()); }
      node.production_target_by_index.iter().for_each(|(_count, part_kind)| {
        if !unlocked_part_kinds.contains(part_kind) { unlocked_part_kinds.push(*part_kind); }
        // Automatically unlock all parts required to create this target part.
        // Won't do this recursively. Other mechanism must prevent proper quest unlock order.
        if options.trace_quest_status { log!("    - These parts are required to create it; {:?} ({:?})", config.nodes[*part_kind].pattern_unique_kinds, config.nodes[*part_kind].pattern_unique_kinds.iter().map(|p|config.nodes[*p].raw_name.clone()).collect::<Vec<String>>()); }
        config.nodes[*part_kind].pattern_unique_kinds.iter().for_each(|kind| {
          if !unlocked_part_kinds.contains(kind) { unlocked_part_kinds.push(*kind); }
        });
      });
    });

  return unlocked_part_kinds;
}

fn config_node_part(index: PartKind, name: String, icon: char) -> ConfigNode {
  let raw_name = format!("Part_{}", name);
  return ConfigNode {
    index,
    story_index: 0,
    quest_index: 0,
    quest_init_status: QuestStatus::Waiting,
    kind: ConfigNodeKind::Part,
    name,
    raw_name,
    unused: false,
    unlocks_after_by_name: vec!(),
    unlocks_after_by_index: vec!(),
    unlocks_todo_by_index: vec!(),
    starting_part_by_name: vec!(),
    starting_part_by_index: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    required_by_quest_indexes: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    pattern_by_icon: vec!(),
    pattern: "".to_string(),
    pattern_unique_kinds: vec!(),
    icon,
    machine_width: 0,
    machine_height: 0,
    machine_asset_name: "Asset_Machine_3_3".to_string(),
    machine_asset_index: CONFIG_NODE_ASSET_MACHINE_3_3,
    special: ('n', 0),

    drm: false,
    sprite_config: SpriteConfig {
      frame_offset: 0,
      frame_count: 1,
      frame_direction: SpriteConfigDirection::Right,
      initial_delay: 10,
      frame_delay: 0,
      looping: true,
      loop_delay: 0,
      loop_backwards: false,
      frames: vec!(
        SpriteFrame {
          file: "".to_string(),
          name: "do not use me; part".to_string(),
          file_canvas_cache_index: 0,
          x: 0.0,
          y: 0.0,
          w: 0.0,
          h: 0.0,
        }
      )
    },
  };
}
fn config_node_belt(index: PartKind, name: &str) -> ConfigNode {
  let raw_name = format!("Belt_{}", name);
  let belt_type = belt_name_to_belt_type(name);
  let belt_meta = belt_type_to_belt_meta(belt_type);
  return ConfigNode {
    index,
    story_index: 0,
    quest_index: 0,
    quest_init_status: QuestStatus::Waiting,
    kind: ConfigNodeKind::Belt,
    name: name.to_string(),
    raw_name,
    unused: false,
    unlocks_after_by_name: vec!(),
    unlocks_after_by_index: vec!(),
    unlocks_todo_by_index: vec!(),
    starting_part_by_name: vec!(),
    starting_part_by_index: vec!(),
    pattern_unique_kinds: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    required_by_quest_indexes: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    pattern_by_icon: vec!(),
    pattern: "".to_string(),
    icon: '?',
    machine_width: 0,
    machine_height: 0,
    machine_asset_name: "Asset_Machine_3_3".to_string(),
    machine_asset_index: CONFIG_NODE_ASSET_MACHINE_3_3,
    special: ('n', 0),

    drm: false,
    sprite_config: SpriteConfig {
      frame_offset: 0,
      frame_count: 1,
      frame_direction: SpriteConfigDirection::Right,
      initial_delay: 10,
      frame_delay: 0,
      looping: true,
      loop_delay: 0,
      loop_backwards: false,
      frames: vec!(
        SpriteFrame {
          file: belt_meta.src.to_string(),
          name: "do not use me; belt".to_string(),
          file_canvas_cache_index: 0,
          x: 0.0,
          y: 0.0,
          w: 160.0,
          h: 160.0
        }
      )
    },
  };
}
fn config_node_asset(index: PartKind, name: &str) -> ConfigNode {
  let raw_name = format!("Asset_{}", name);
  return ConfigNode {
    index,
    story_index: 0,
    quest_index: 0,
    quest_init_status: QuestStatus::Waiting,
    kind: ConfigNodeKind::Asset,
    name: name.to_string(),
    raw_name,
    unused: false,
    unlocks_after_by_name: vec!(),
    unlocks_after_by_index: vec!(),
    unlocks_todo_by_index: vec!(),
    starting_part_by_name: vec!(),
    starting_part_by_index: vec!(),
    pattern_unique_kinds: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    required_by_quest_indexes: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    pattern_by_icon: vec!(),
    pattern: "".to_string(),
    icon: '?',
    machine_width: 0,
    machine_height: 0,
    machine_asset_name: "Asset_Machine_3_3".to_string(),
    machine_asset_index: CONFIG_NODE_ASSET_MACHINE_3_3,
    special: ('n', 0),

    drm: false,
    sprite_config: SpriteConfig {
      frame_offset: 0,
      frame_count: 1,
      frame_direction: SpriteConfigDirection::Right,
      initial_delay: 10,
      frame_delay: 0,
      looping: false,
      loop_delay: 0,
      loop_backwards: false,
      frames: vec!(
        SpriteFrame {
          file: "./img/none.png".to_string(),
          name: "do not use me; belt".to_string(),
          file_canvas_cache_index: 0,
          x: 0.0,
          y: 0.0,
          w: 64.0,
          h: 64.0
        }
      )
    },
  };
}
fn config_node_story(index: PartKind, name: &str) -> ConfigNode {
  let raw_name = format!("Story_{}", name);
  return ConfigNode {
    index,
    story_index: 0,
    quest_index: 0,
    quest_init_status: QuestStatus::Waiting,
    kind: ConfigNodeKind::Story,
    name: name.to_string(),
    raw_name,
    unused: false,
    unlocks_after_by_name: vec!(),
    unlocks_after_by_index: vec!(),
    unlocks_todo_by_index: vec!(),
    starting_part_by_name: vec!(),
    starting_part_by_index: vec!(),
    pattern_unique_kinds: vec!(),
    production_target_by_name: vec!(),
    production_target_by_index: vec!(),
    required_by_quest_indexes: vec!(),
    pattern_by_index: vec!(),
    pattern_by_name: vec!(),
    pattern_by_icon: vec!(),
    pattern: "".to_string(),
    icon: '?',
    machine_width: 0,
    machine_height: 0,
    machine_asset_name: "Asset_Machine_3_3".to_string(),
    machine_asset_index: CONFIG_NODE_ASSET_MACHINE_3_3,
    special: ('n', 0),

    drm: false,
    sprite_config: SpriteConfig {
      frame_offset: 0,
      frame_count: 1,
      frame_direction: SpriteConfigDirection::Right,
      initial_delay: 10,
      frame_delay: 0,
      looping: false,
      loop_delay: 0,
      loop_backwards: false,
      frames: vec!(
        SpriteFrame {
          file: "./img/none.png".to_string(),
          name: "do not use me; story".to_string(),
          file_canvas_cache_index: 0,
          x: 0.0,
          y: 0.0,
          w: 64.0,
          h: 64.0
        }
      )
    },
  };
}

pub fn config_get_sprite_details<'x>(config: &'x Config, options: &Options, config_index: usize, sprite_start_at: u64, ticks: u64) -> (f64, f64, f64, f64, &'x web_sys::HtmlImageElement) {
  assert!(config_index < config.nodes.len(), "config_index should be a node index: {} < {}", config_index, config.nodes.len());
  let mut node = &config.nodes[config_index];
  if node.drm && !options.show_drm {
    node = &config.nodes[CONFIG_NODE_ASSET_DRM_PLACEHOLDER];
  }
  let sprite_config = &node.sprite_config;

  let frame_offset = sprite_config.frame_offset;

  if sprite_start_at - ticks < sprite_config.initial_delay {
    let sprite = &node.sprite_config.frames[frame_offset];
    return ( sprite.x, sprite.y, sprite.w, sprite.h, &config.sprite_cache_canvas[sprite.file_canvas_cache_index] );
  }

  let frame_count = sprite_config.frames.len();
  let frame_delay = sprite_config.frame_delay.max(1);
  let loop_delay = sprite_config.loop_delay.max(0);
  let progress = ticks - (sprite_start_at + sprite_config.initial_delay);
  let frame_duration_loop = frame_count as u64 * frame_delay;
  let looping = sprite_config.looping;
  let loop_duration = frame_count as u64 * frame_delay + loop_delay;
  let current_loop = if looping { progress % loop_duration.max(1) } else { progress.max(1).min(loop_duration - 1) };
  let frame_index = (current_loop / sprite_config.frame_delay.max(1)) as usize;

  assert!(loop_delay != 0 || frame_index < frame_count, "without loop delay the frame index should not exceed the frame count");
  // If "progress" is inside the loop delay then pin the last frame and prevent oob here
  let frame_index1 = frame_index.min(frame_count - 1);
  // If not looping and the index is beyond the last, pin it to the last. Otherwise loop it.
  let frame_index2 = if looping { frame_index1 % frame_count } else { frame_index1 };
  // Move pointer to compensate for starting frame
  let frame_index3 = (frame_index2 + frame_offset) % frame_count;
  // If backward then flip the index
  let frame_index4 = if sprite_config.loop_backwards { (frame_count - 1) - frame_index2 } else { frame_index2 };

  let sprite = &node.sprite_config.frames[frame_index4];
  return ( sprite.x, sprite.y, sprite.w, sprite.h, &config.sprite_cache_canvas[sprite.file_canvas_cache_index] );
}

// Convert a config node to jsvalue (sorta) so we can send it to the html for debug/editor
fn convert_vec_string_to_jsvalue(v: &Vec<String>) -> JsValue {
  return v.iter().cloned().map(|x| JsValue::from(x)).collect::<js_sys::Array>().into();
}
fn convert_vec_char_to_jsvalue(v: &Vec<char>) -> JsValue {
  return v.iter().cloned().map(|x| JsValue::from(format!("{}", x))).collect::<js_sys::Array>().into();
}
fn convert_vec_usize_to_jsvalue(v: &Vec<usize>) -> JsValue {
  return v.iter().cloned().map(|x| JsValue::from(x)).collect::<js_sys::Array>().into();
}
fn convert_string_to_pair(a: &str, b: &str) -> js_sys::Array {
  return convert_js_to_pair(a, JsValue::from(b));
}
fn convert_js_to_pair(a: &str, b: JsValue) -> js_sys::Array {
  return js_sys::Array::of2(&JsValue::from(a), &b);
}
fn config_node_to_jsvalue(node: &ConfigNode) -> JsValue {
  return vec!(
    convert_js_to_pair("index", JsValue::from(node.index)),
    convert_string_to_pair("kind", format!("{:?}", node.kind).as_str()),
    convert_string_to_pair("name", &node.name),
    convert_string_to_pair("raw_name", &node.raw_name),

    convert_js_to_pair("unlocks_after_by_name", convert_vec_string_to_jsvalue(&node.unlocks_after_by_name)),
    convert_js_to_pair("unlocks_after_by_index", convert_vec_usize_to_jsvalue(&node.unlocks_after_by_index)),
    convert_js_to_pair("unlocks_todo_by_index", convert_vec_usize_to_jsvalue(&node.unlocks_todo_by_index)),
    convert_js_to_pair("starting_part_by_name", convert_vec_string_to_jsvalue(&node.starting_part_by_name)),
    convert_js_to_pair("starting_part_by_index", convert_vec_usize_to_jsvalue(&node.starting_part_by_index)),
    vec!(
      JsValue::from("production_target_by_name"),
      node.production_target_by_name.iter().cloned().map(|(count, name)| vec!(JsValue::from(count), JsValue::from(name)).iter().collect::<js_sys::Array>()).collect::<js_sys::Array>().into()
    ).iter().collect::<js_sys::Array>(),
    vec!(
      JsValue::from("production_target_by_index"),
      node.production_target_by_index.iter().cloned().map(|(count, part)| vec!(JsValue::from(count), JsValue::from(part)).iter().collect::<js_sys::Array>()).collect::<js_sys::Array>().into()
    ).iter().collect::<js_sys::Array>(),

    convert_js_to_pair("pattern_by_index", convert_vec_usize_to_jsvalue(&node.pattern_by_index)),
    convert_js_to_pair("pattern_by_name", convert_vec_string_to_jsvalue(&node.pattern_by_name)),
    convert_js_to_pair("pattern_by_icon", convert_vec_char_to_jsvalue(&node.pattern_by_icon)),
    convert_js_to_pair("pattern", JsValue::from(node.pattern.clone())),
    convert_js_to_pair("pattern_unique_kinds", convert_vec_usize_to_jsvalue(&node.pattern_unique_kinds)),
    convert_string_to_pair("icon", format!("{}", node.icon).as_str()),

    convert_js_to_pair(
      "sprite_config",
      node.sprite_config.frames.iter().map(|frame| {
        JsValue::from(vec!(
          convert_string_to_pair("file", frame.file.as_str()),
          convert_js_to_pair("file_canvas_cache_index", JsValue::from(frame.file_canvas_cache_index)),
          convert_js_to_pair("x", JsValue::from(frame.x)),
          convert_js_to_pair("y", JsValue::from(frame.y)),
          convert_js_to_pair("w", JsValue::from(frame.w)),
          convert_js_to_pair("h", JsValue::from(frame.h)),
          ).iter().collect::<js_sys::Array>())
      }).collect::<js_sys::Array>().into()
    ),
  ).iter().collect::<js_sys::Array>().into();
}
pub fn config_to_jsvalue(config: &Config) -> JsValue {
  return config.nodes.iter().map(|node| {
    let key: JsValue = node.raw_name.clone().into();
    let value = config_node_to_jsvalue(node);
    return vec!(key, value).iter().collect::<js_sys::Array>();
  }).collect::<js_sys::Array>().into();
}

pub fn config_get_sprite_for_belt_type<'x>(config: &'x Config, options: &Options, belt_type: BeltType, sprite_start_at: u64, ticks: u64) -> (f64, f64, f64, f64, &'x web_sys::HtmlImageElement) {
  return config_get_sprite_details(config, options, belt_type as usize, sprite_start_at, ticks);
}
