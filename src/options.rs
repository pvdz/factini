use super::log;
use super::utils::*;
use super::state::*;

// Design is for the default speed to run 10k ticks per real world second
pub const ONE_MS: u64 = 10;
pub const ONE_SECOND: u64 = 1000 * ONE_MS;
pub const MAX_TICKS_PER_FRAME: u64 = 1000; // Frame limiter

// Size of the floor
pub const FLOOR_CELLS_W: usize = 1 + 5*3 + 1;
pub const FLOOR_CELLS_H: usize = 1 + 5*3 + 1;
pub const FLOOR_CELLS_WH: usize = FLOOR_CELLS_W * FLOOR_CELLS_H;

// Size of a cell
pub const CELL_W: f64 = 32.0;
pub const CELL_H: f64 = 32.0;

// World pixels size of the floor
pub const FLOOR_WIDTH: f64 = FLOOR_CELLS_W as f64 * CELL_W;
pub const FLOOR_HEIGHT: f64 = FLOOR_CELLS_H as f64 * CELL_H;

// Size of parts on a belt
pub const PART_W: f64 = 20.0;
pub const PART_H: f64 = 20.0;

pub struct Options {
  pub options_started_from_source: u64, // If non-zero, the current config started from local storage rather than source (options.md.js) and it will be the byte size
  pub initial_map_from_source: u64, // If non-zero, the initial map started from local storage rather than source (map.md.js) and it will be the byte size

  pub print_options_string: bool,
  pub print_choices: bool,
  pub print_choices_belt: bool,
  pub print_choices_machine: bool,
  pub print_choices_supply: bool,
  pub print_choices_demand: bool,
  pub print_moves: bool,
  pub print_moves_belt: bool,
  pub print_moves_machine: bool,
  pub print_moves_supply: bool,
  pub print_moves_demand: bool,
  pub print_price_deltas: bool,
  pub print_machine_actions: bool,
  pub print_factory_interval: u64,
  pub print_stats_interval: u64,
  pub print_auto_layout_debug: bool,
  pub print_fmd_trace: bool,
  pub print_img_loader_trace: bool,
  pub trace_priority_step: bool,
  pub trace_porting_step: bool,
  pub trace_map_parsing: bool,
  pub print_priority_tile_order: bool, // Print the prio index of a tile in the game (debug, web)
  pub print_initial_table: bool, // Print the CLI version of the floor after generating it initially?

  pub show_drm: bool, // Draw media where configNode.drm == true ?
  pub draw_part_borders: bool, // Draw a border around Parts? Helps debugging invisible parts due to sprite problems
  pub draw_part_char_icon: bool, // Draw the char icon representation for a part on top of it
  pub draw_part_kind: bool, // Draw the part kind id representation for a part on top of it
  pub draw_port_arrows: bool, // Draw the port directional arrows?
  pub paint_belts: bool, // Paint the belt background tiles?
  pub draw_belt_dbg_id: bool, // Draw the belt id on top of each belt? "dl_r" etc

  pub enable_craft_menu_circle: bool, // When you select a machine, should a craft menu open up for it? This used to be a thing but then I changed my mind. It was too confusing. Otherwise it just shows the cells to drag offers onto.
  pub enable_craft_menu_interact: bool, // Can you interact with items in a machine? This used to be the default but I simplified it which includes disabling this interaction. This option would enable it again.

  pub draw_zone_hovers: bool, // Draw a rect on the area where the mouse is detected
  pub draw_zone_borders: bool, // Draw a guide around each grid section of the ui?
  pub zone_borders_color: String, // the color of this border

  pub short_term_window: u64, // For stats; average over this many ticks
  pub long_term_window: u64, // For stats; average over this many ticks

  pub speed_modifier_floor: f64, // Increase or decrease ticks per second by this rate for (actual factory game/floor) animations
  pub speed_modifier_ui: f64, // Same as speed_modifier_floor but for rest of the UI (buttons, bouncers, trucks, dropzone pulse, etc)
  pub touch_drag_compensation: bool, // Show the mouse pointer 50x50 away from the actual pointer? helpful for dragging on touch screen but can be annoying

  pub game_enable_clean_days: bool, // Require to achieve quests from a clean day start rather than in any way
  pub game_auto_reset_day: bool, // Immediately reset day (and clear parts) upon factory change? I find it annoying but maybe you don't?

  // Dropzone hint for offers
  pub dropzone_color_offset: u64, // In a 255 color range, what's the "zero" bounce value?
  pub dropzone_bounce_speed: u64, // The rgb color value will "bounce" up and down
  pub dropzone_bounce_distance: u64, // Range that the color bounces

  pub bouncer_time_to_factory: f64, // After how much time should the bouncer be considered complete?
  pub bouncer_decay_rate_modifier: f64, // General decay modifier
  pub bouncer_amplitude_decay_rate: f64, // How fast the wave become less tall (not shorter)
  pub bouncer_wave_decay_rate: f64, // How fast the bounces become shorter (not less tall)
  pub bouncer_initial_angle: f64, // Determines where the wave starts. Kinda sensitive to tune against the badges.
  pub bouncer_angular_freq: f64, // Influences wave length
  pub bouncer_trail_time: f64, // How long does a ghost stay 100% opaque before starting to fade? Relative to one real world second, subject to be modified by ui speed
  pub bouncer_fade_time: f64, // How long does it take for a ghost to fade out? Relative to one real world second, subject to be modified by ui speed
  pub bouncer_stamp_interval: u64, // Every how many frames do we create a ghost frame?
  pub bouncer_stop_after: f64, // On the scale of bouncer_formula_total_distance, how far into the formula do we end the bouncer? this would be ideally when the bouncer is inside the factory.
  pub bouncer_formula_total_distance: f64, // The bouncer follows a formula and this is the would-be total distance until it "stops bouncing", the bouncer_stop_after is a normalized point onto this value

  // obsolete
  pub bouncer_decay_speed: f64,

  pub splash_keep_loader: bool, // Keep showing the loader screen, even when loading is complete
  pub splash_no_loader: bool, // Skip the loader screen and go straight to the game
  pub splash_keep_main: bool, // Show main menu?
  pub splash_no_main: bool, // Skip main menu?

  pub web_output_cli: bool, // Print the simplified cli output in web version?

  pub initial_event_type_swapped: bool, // sets initial state.event_type_swapped -> MOUSE / TOUCH

  pub dbg_trash_is_joker: bool, // Trash serves as joker item for machines?
  pub db_joker_corrupts_factory: bool, // Show visual change when corrupting the factory
  pub dbg_machine_produce_trash: bool, // If a machine trashes a part and expects no inputs, should it output trash instead of discarding it?
  pub dbg_clickable_quests: bool,
  pub dbg_print_quest_states: bool,

  pub default_demand_speed: u64,
  pub default_demand_cooldown: u64,

  pub test: u64, // just a temporary flag
}

pub fn create_options(speed_modifier_floor: f64, speed_modifier_ui: f64) -> Options {
  return Options {
    options_started_from_source: 0, // Updated elsewhere
    initial_map_from_source: 0, // Updated elsewhere

    print_options_string: true,
    print_choices: false,
    print_choices_belt: false,
    print_choices_machine: false,
    print_choices_supply: false,
    print_choices_demand: false,
    print_moves: false,
    print_moves_belt: false,
    print_moves_machine: false,
    print_moves_supply: false,
    print_moves_demand: false,
    print_price_deltas: false,
    print_machine_actions: false,
    print_factory_interval: 5000,
    print_stats_interval: 100000,
    print_auto_layout_debug: false,
    print_fmd_trace: false,
    print_img_loader_trace: false,
    trace_priority_step: false,
    trace_porting_step: false,
    trace_map_parsing: false,
    print_priority_tile_order: false,
    print_initial_table: false,
    show_drm: true,
    draw_part_borders: false,
    draw_part_char_icon: false,
    draw_part_kind: false,
    draw_port_arrows: false,
    paint_belts: true,
    draw_belt_dbg_id: false,
    enable_craft_menu_circle: false,
    enable_craft_menu_interact: false,
    draw_zone_hovers: false,
    draw_zone_borders: false,
    zone_borders_color: "white".to_string(),
    short_term_window: 10000,
    long_term_window: 600000,
    speed_modifier_floor,
    speed_modifier_ui,
    touch_drag_compensation: false,
    game_enable_clean_days: false,
    game_auto_reset_day: false,
    dropzone_color_offset: 75,
    dropzone_bounce_speed: 10,
    dropzone_bounce_distance: 150,
    bouncer_decay_rate_modifier: 3.0,
    bouncer_amplitude_decay_rate: 1.0,
    bouncer_wave_decay_rate: 1.0,
    bouncer_initial_angle: 45.0,
    bouncer_angular_freq: 6.28,
    bouncer_time_to_factory: 10.0,
    bouncer_trail_time: 2.0,
    bouncer_fade_time: 2.0,
    bouncer_stamp_interval: 20,
    bouncer_stop_after: 1200.0,
    bouncer_formula_total_distance: 650.0,
    splash_keep_loader: false,
    splash_no_loader: false,
    splash_keep_main: false,
    splash_no_main: false,
    bouncer_decay_speed: 1.2,
    web_output_cli: false,
    initial_event_type_swapped: false,
    dbg_trash_is_joker: true,
    db_joker_corrupts_factory: true,
    dbg_machine_produce_trash: true,
    dbg_clickable_quests: true,
    dbg_print_quest_states: false,
    default_demand_speed: 1000,
    default_demand_cooldown: 500,
    test: 0,
  };
}

fn parse_bool(value: &str, key: &str, strict: bool, def: bool, verbose: bool) -> bool {
  let out = match value {
    "true" => true,
    "false" => false,
    _ => {
      if strict {
        panic!("Invalid value for options.{}; expecting a boolean, received `{}`", key, value)
      } else {
        if verbose { log!("Invalid value for options.{}; expecting a boolean, received `{}`", key, value); }
        def
      }
    },
  };
  if out != def && verbose{
    log!("Changed value for options.{}: from: {}, to: {}", key, def, out);
  }
  return out;
}

fn parse_f64(value: &str, key: &str, strict: bool, def: f64, verbose: bool) -> f64 {
  let b =
    if !value.contains(".") {
      format!("{}.0", value)
    } else {
      value.to_string()
    };

  let t = b.parse::<f64>();
  let out =
    if strict {
      t.expect(format!("The value for options.{} must be a number, received `{}`", key, b).as_str())
    } else {
      t.or::<f64>(Ok(def)).unwrap()
    };
  if out != def && verbose{
    log!("Changed value for options.{}: from: {}, to: {}", key, def, out);
  }
  return out;
}

fn parse_u64(value: &str, key: &str, strict: bool, def: u64, verbose: bool) -> u64 {
  return parse_f64(value, key, strict, def as f64, verbose) as u64;
}

fn parse_string(value: String, key: &str, strict: bool, def: String, verbose: bool) -> String {
  let out = _parse_string(value, key, strict, &def, verbose);
  if out != def && verbose {
    log!("Changed value for options.{}: from: {}, to: {}", key, def, out);
  }
  return out;
}
fn _parse_string(value: String, key: &str, strict: bool, def: &String, verbose: bool) -> String {
  if value.starts_with("\"") {
    if !value.ends_with("\"") {
      if strict {
        panic!("Missing double quote at end of value; options.{}, value = `{:?}`", key, value);
      } else {
        if verbose { log!("Missing double quote at end of value; options.{}, value = `{:?}`", key, value); }
        return def.clone();
      }
    }
  }
  else if value.starts_with("'") {
    if !value.ends_with("'") {
      if strict {
        panic!("Missing single quote at end of value; options.{}, value = `{:?}`", key, value);
      } else {
        if verbose { log!("Missing single quote at end of value; options.{}, value = `{:?}`", key, value); }
        return def.clone();
      }
    }
  }
  else {
    if strict {
      panic!("Unable to strict parse string for options.{}; value was `{:?}`, {}, {}", key, value, value.starts_with("\""), value.starts_with("'"));
    } else {
      if verbose { log!("Unable to parse string for options.{}; value was `{:?}`", key, value); }
      return def.clone();
    }
  }

  return format!("{}", value)[1..value.len()-1].to_string();
}

pub fn parse_options_into(input: String, options: &mut Options, strict: bool) {
  log!("parse_options_into()");

  let mut verbose = options.print_options_string;

  let trimmed = input.trim().clone().split('\n');
  trimmed.for_each(|line| {
    // Drop the list prefix ('-')
    let line = line.trim();
    if line.starts_with('-') {
      // Get the name and the value
      match line[1..].trim().split_once(':') {
        Some((name, value)) => {
          let name = name.trim();
          let value = value.trim();

          if name == "print_options_string" { verbose = value == "true"; }
          if verbose { log!("- updating options.{} to `{}`", name, value); }

          match name {
            "print_options_string" => options.print_options_string = parse_bool(value, name, strict, options.print_options_string, verbose),
            "options_started_from_source" => options.options_started_from_source = parse_u64(value, name, strict, options.options_started_from_source, verbose),
            "initial_map_from_source" => options.initial_map_from_source = parse_u64(value, name, strict, options.initial_map_from_source, verbose),
            "print_choices" => options.print_choices = parse_bool(value, name, strict, options.print_choices, verbose),
            "print_choices_belt" => options.print_choices_belt = parse_bool(value, name, strict, options.print_choices_belt, verbose),
            "print_choices_machine" => options.print_choices_machine = parse_bool(value, name, strict, options.print_choices_machine, verbose),
            "print_choices_supply" => options.print_choices_supply = parse_bool(value, name, strict, options.print_choices_supply, verbose),
            "print_choices_demand" => options.print_choices_demand = parse_bool(value, name, strict, options.print_choices_demand, verbose),
            "print_moves" => options.print_moves = parse_bool(value, name, strict, options.print_moves, verbose),
            "print_moves_belt" => options.print_moves_belt = parse_bool(value, name, strict, options.print_moves_belt, verbose),
            "print_moves_machine" => options.print_moves_machine = parse_bool(value, name, strict, options.print_moves_machine, verbose),
            "print_moves_supply" => options.print_moves_supply = parse_bool(value, name, strict, options.print_moves_supply, verbose),
            "print_moves_demand" => options.print_moves_demand = parse_bool(value, name, strict, options.print_moves_demand, verbose),
            "print_price_deltas" => options.print_price_deltas = parse_bool(value, name, strict, options.print_price_deltas, verbose),
            "print_machine_actions" => options.print_machine_actions = parse_bool(value, name, strict, options.print_machine_actions, verbose),
            "print_factory_interval" => options.print_factory_interval = parse_u64(value, name, strict, options.print_factory_interval, verbose),
            "print_stats_interval" => options.print_stats_interval = parse_u64(value, name, strict, options.print_stats_interval, verbose),
            "print_auto_layout_debug" => options.print_auto_layout_debug = parse_bool(value, name, strict, options.print_auto_layout_debug, verbose),
            "print_fmd_trace" => options.print_fmd_trace = parse_bool(value, name, strict, options.print_fmd_trace, verbose),
            "print_img_loader_trace" => options.print_img_loader_trace = parse_bool(value, name, strict, options.print_img_loader_trace, verbose),
            "trace_priority_step" => options.trace_priority_step = parse_bool(value, name, strict, options.trace_priority_step, verbose),
            "trace_porting_step" => options.trace_porting_step = parse_bool(value, name, strict, options.trace_porting_step, verbose),
            "trace_map_parsing" => options.trace_map_parsing = parse_bool(value, name, strict, options.trace_map_parsing, verbose),
            "print_priority_tile_order" => options.print_priority_tile_order = parse_bool(value, name, strict, options.print_priority_tile_order, verbose),
            "print_initial_table" => options.print_initial_table = parse_bool(value, name, strict, options.print_initial_table, verbose),
            "show_drm" => options.show_drm = parse_bool(value, name, strict, options.show_drm, verbose),
            "draw_part_borders" => options.draw_part_borders = parse_bool(value, name, strict, options.draw_part_borders, verbose),
            "draw_part_char_icon" => options.draw_part_char_icon = parse_bool(value, name, strict, options.draw_part_char_icon, verbose),
            "draw_part_kind" => options.draw_part_kind = parse_bool(value, name, strict, options.draw_part_kind, verbose),
            "draw_port_arrows" => options.draw_port_arrows = parse_bool(value, name, strict, options.draw_port_arrows, verbose),
            "paint_belts" => options.paint_belts = parse_bool(value, name, strict, options.paint_belts, verbose),
            "draw_belt_dbg_id" => options.draw_belt_dbg_id = parse_bool(value, name, strict, options.draw_belt_dbg_id, verbose),
            "enable_craft_menu_circle" => options.enable_craft_menu_circle = parse_bool(value, name, strict, options.enable_craft_menu_circle, verbose),
            "enable_craft_menu_interact" => options.enable_craft_menu_interact = parse_bool(value, name, strict, options.enable_craft_menu_interact, verbose),
            "draw_zone_hovers" => options.draw_zone_hovers = parse_bool(value, name, strict, options.draw_zone_hovers, verbose),
            "draw_zone_borders" => options.draw_zone_borders = parse_bool(value, name, strict, options.draw_zone_borders, verbose),
            "zone_borders_color" => options.zone_borders_color = parse_string(value.to_string(), name, strict, options.zone_borders_color.clone(), verbose),
            "short_term_window" => options.short_term_window = parse_u64(value, name, strict, options.short_term_window, verbose),
            "long_term_window" => options.long_term_window = parse_u64(value, name, strict, options.long_term_window, verbose),
            "bouncer_decay_rate_modifier" => options.bouncer_decay_rate_modifier = parse_f64(value, name, strict, options.bouncer_decay_rate_modifier, verbose),
            "bouncer_amplitude_decay_rate" => options.bouncer_amplitude_decay_rate = parse_f64(value, name, strict, options.bouncer_amplitude_decay_rate, verbose),
            "bouncer_wave_decay_rate" => options.bouncer_wave_decay_rate = parse_f64(value, name, strict, options.bouncer_wave_decay_rate, verbose),
            "bouncer_initial_angle" => options.bouncer_initial_angle = parse_f64(value, name, strict, options.bouncer_initial_angle, verbose),
            "bouncer_angular_freq" => options.bouncer_angular_freq = parse_f64(value, name, strict, options.bouncer_angular_freq, verbose),
            "bouncer_time_to_factory" => options.bouncer_time_to_factory = parse_f64(value, name, strict, options.bouncer_time_to_factory, verbose),
            "bouncer_decay_speed" => options.bouncer_decay_speed = parse_f64(value, name, strict, options.bouncer_decay_speed, verbose),
            "speed_modifier_floor" => options.speed_modifier_floor = parse_f64(value, name, strict, options.speed_modifier_floor, verbose),
            "speed_modifier_ui" => options.speed_modifier_ui = parse_f64(value, name, strict, options.speed_modifier_ui, verbose),
            "bouncer_trail_time" => options.bouncer_trail_time = parse_f64(value, name, strict, options.bouncer_trail_time, verbose),
            "bouncer_fade_time" => options.bouncer_fade_time = parse_f64(value, name, strict, options.bouncer_fade_time, verbose),
            "bouncer_stamp_interval" => options.bouncer_stamp_interval = parse_u64(value, name, strict, options.bouncer_stamp_interval, verbose),
            "bouncer_stop_after" => options.bouncer_stop_after = parse_f64(value, name, strict, options.bouncer_stop_after, verbose),
            "bouncer_formula_total_distance" => options.bouncer_formula_total_distance = parse_f64(value, name, strict, options.bouncer_formula_total_distance, verbose),
            "splash_keep_loader" => options.splash_keep_loader = parse_bool(value, name, strict, options.splash_keep_loader, verbose),
            "splash_no_loader" => options.splash_no_loader = parse_bool(value, name, strict, options.splash_no_loader, verbose),
            "splash_keep_main" => options.splash_keep_main = parse_bool(value, name, strict, options.splash_keep_main, verbose),
            "splash_no_main" => options.splash_no_main = parse_bool(value, name, strict, options.splash_no_main, verbose),
            "touch_drag_compensation" => options.touch_drag_compensation = parse_bool(value, name, strict, options.touch_drag_compensation, verbose),
            "game_enable_clean_days" => options.game_enable_clean_days = parse_bool(value, name, strict, options.game_enable_clean_days, verbose),
            "game_auto_reset_day" => options.game_auto_reset_day = parse_bool(value, name, strict, options.game_auto_reset_day, verbose),
            "dropzone_color_offset" => options.dropzone_color_offset = parse_u64(value, name, strict, options.dropzone_color_offset, verbose),
            "dropzone_bounce_speed" => options.dropzone_bounce_speed = parse_u64(value, name, strict, options.dropzone_bounce_speed, verbose),
            "dropzone_bounce_distance" => options.dropzone_bounce_distance = parse_u64(value, name, strict, options.dropzone_bounce_distance, verbose),
            "web_output_cli" => options.web_output_cli = parse_bool(value, name, strict, options.web_output_cli, verbose),
            "initial_event_type_swapped" => options.initial_event_type_swapped = parse_bool(value, name, strict, options.initial_event_type_swapped, verbose),
            "dbg_trash_is_joker" => options.dbg_trash_is_joker = parse_bool(value, name, strict, options.dbg_trash_is_joker, verbose),
            "db_joker_corrupts_factory" => options.db_joker_corrupts_factory = parse_bool(value, name, strict, options.db_joker_corrupts_factory, verbose),
            "dbg_machine_produce_trash" => options.dbg_machine_produce_trash = parse_bool(value, name, strict, options.dbg_machine_produce_trash, verbose),
            "dbg_clickable_quotes" => options.dbg_clickable_quests = parse_bool(value, name, strict, options.dbg_clickable_quests, verbose),
            "dbg_print_quest_states" => options.dbg_print_quest_states = parse_bool(value, name, strict, options.dbg_print_quest_states, verbose),
            "default_demand_speed" => options.default_demand_speed = parse_u64(value, name, strict, options.default_demand_speed, verbose),
            "default_demand_cooldown" => options.default_demand_cooldown = parse_u64(value, name, strict, options.default_demand_cooldown, verbose),
            "test" => options.test = parse_u64(value, name, strict, options.test, verbose),
            _ => {
              log!("  - ignoring `{}` because it is an unknown option or because it needs to be added to the options parser", name);
            }
          }
        }
        None => {
          // Ignore parts of the md that do not have a colon
        }
      }
    }
  })
}

pub fn options_serialize(options: &Options) -> String {
  let mut arr = vec!();
  arr.push(format!("- options_started_from_source: {}", options.options_started_from_source));
  arr.push(format!("- initial_map_from_source: {}", options.initial_map_from_source));
  arr.push(format!("- print_options_string: {}", options.print_options_string));
  arr.push(format!("- print_choices: {}", options.print_choices));
  arr.push(format!("- print_choices_belt: {}", options.print_choices_belt));
  arr.push(format!("- print_choices_machine: {}", options.print_choices_machine));
  arr.push(format!("- print_choices_supply: {}", options.print_choices_supply));
  arr.push(format!("- print_choices_demand: {}", options.print_choices_demand));
  arr.push(format!("- print_moves: {}", options.print_moves));
  arr.push(format!("- print_moves_belt: {}", options.print_moves_belt));
  arr.push(format!("- print_moves_machine: {}", options.print_moves_machine));
  arr.push(format!("- print_moves_supply: {}", options.print_moves_supply));
  arr.push(format!("- print_moves_demand: {}", options.print_moves_demand));
  arr.push(format!("- print_price_deltas: {}", options.print_price_deltas));
  arr.push(format!("- print_machine_actions: {}", options.print_machine_actions));
  arr.push(format!("- print_factory_interval: {}", options.print_factory_interval));
  arr.push(format!("- print_stats_interval: {}", options.print_stats_interval));
  arr.push(format!("- print_auto_layout_debug: {}", options.print_auto_layout_debug));
  arr.push(format!("- print_fmd_trace: {}", options.print_fmd_trace));
  arr.push(format!("- print_img_loader_trace: {}", options.print_img_loader_trace));
  arr.push(format!("- trace_priority_step: {}", options.trace_priority_step));
  arr.push(format!("- trace_porting_step: {}", options.trace_porting_step));
  arr.push(format!("- trace_map_parsing: {}", options.trace_map_parsing));
  arr.push(format!("- print_priority_tile_order: {}", options.print_priority_tile_order));
  arr.push(format!("- print_initial_table: {}", options.print_initial_table));
  arr.push(format!("- show_drm: {}", options.show_drm));
  arr.push(format!("- draw_part_borders: {}", options.draw_part_borders));
  arr.push(format!("- draw_part_char_icon: {}", options.draw_part_char_icon));
  arr.push(format!("- draw_part_kind: {}", options.draw_part_kind));
  arr.push(format!("- draw_port_arrows: {}", options.draw_port_arrows));
  arr.push(format!("- paint_belts: {}", options.paint_belts));
  arr.push(format!("- draw_belt_dbg_id: {}", options.draw_belt_dbg_id));
  arr.push(format!("- enable_craft_menu_circle: {}", options.enable_craft_menu_circle));
  arr.push(format!("- enable_craft_menu_interact: {}", options.enable_craft_menu_interact));
  arr.push(format!("- draw_zone_hovers: {}", options.draw_zone_hovers));
  arr.push(format!("- draw_zone_borders: {}", options.draw_zone_borders));
  arr.push(format!("- zone_borders_color: '{}'", options.zone_borders_color));
  arr.push(format!("- short_term_window: {}", options.short_term_window));
  arr.push(format!("- long_term_window: {}", options.long_term_window));
  arr.push(format!("- speed_modifier_floor: {}", options.speed_modifier_floor));
  arr.push(format!("- speed_modifier_ui: {}", options.speed_modifier_ui));
  arr.push(format!("- touch_drag_compensation: {}", options.touch_drag_compensation));
  arr.push(format!("- game_enable_clean_days: {}", options.game_enable_clean_days));
  arr.push(format!("- game_auto_reset_day: {}", options.game_auto_reset_day));
  arr.push(format!("- dropzone_color_offset: {}", options.dropzone_color_offset));
  arr.push(format!("- dropzone_bounce_speed: {}", options.dropzone_bounce_speed));
  arr.push(format!("- dropzone_bounce_distance: {}", options.dropzone_bounce_distance));
  arr.push(format!("- bouncer_time_to_factory: {}", options.bouncer_time_to_factory));
  arr.push(format!("- bouncer_decay_rate_modifier: {}", options.bouncer_decay_rate_modifier));
  arr.push(format!("- bouncer_amplitude_decay_rate: {}", options.bouncer_amplitude_decay_rate));
  arr.push(format!("- bouncer_wave_decay_rate: {}", options.bouncer_wave_decay_rate));
  arr.push(format!("- bouncer_initial_angle: {}", options.bouncer_initial_angle));
  arr.push(format!("- bouncer_angular_freq: {}", options.bouncer_angular_freq));
  arr.push(format!("- bouncer_trail_time: {}", options.bouncer_trail_time));
  arr.push(format!("- bouncer_fade_time: {}", options.bouncer_fade_time));
  arr.push(format!("- bouncer_stamp_interval: {}", options.bouncer_stamp_interval));
  arr.push(format!("- bouncer_stop_after: {}", options.bouncer_stop_after));
  arr.push(format!("- bouncer_formula_total_distance: {}", options.bouncer_formula_total_distance));
  arr.push(format!("- bouncer_decay_speed: {}", options.bouncer_decay_speed));
  arr.push(format!("- splash_keep_loader: {}", options.splash_keep_loader));
  arr.push(format!("- splash_no_loader: {}", options.splash_no_loader));
  arr.push(format!("- splash_keep_main: {}", options.splash_keep_main));
  arr.push(format!("- splash_no_main: {}", options.splash_no_main));
  arr.push(format!("- web_output_cli: {}", options.web_output_cli));
  arr.push(format!("- initial_event_type_swapped: {}", options.initial_event_type_swapped));
  arr.push(format!("- dbg_trash_is_joker: {}", options.dbg_trash_is_joker));
  arr.push(format!("- db_joker_corrupts_factory: {}", options.db_joker_corrupts_factory));
  arr.push(format!("- dbg_machine_produce_trash: {}", options.dbg_machine_produce_trash));
  arr.push(format!("- dbg_clickable_quotes: {}", options.dbg_clickable_quests));
  arr.push(format!("- dbg_print_quest_states: {}", options.dbg_print_quest_states));
  arr.push(format!("- default_demand_speed: {}", options.default_demand_speed));
  arr.push(format!("- default_demand_cooldown: {}", options.default_demand_cooldown));
  arr.push(format!("- test: {}", options.test));
  return arr.join("\n");
}
