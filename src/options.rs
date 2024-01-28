use super::log;
use super::utils::*;
use super::state::*;

// Design was for the default speed to run 10k ticks per real world second

// How many seconds should it take for one part to travel across one belt cell at normal speed?
// This ultimately determines visual game speed
pub const PART_OVER_BELT_SPEED_SEC: f64 = 0.5;
pub const MAZE_TICK_INTERVAL: u64 = 400; // One maze tick every this many factory ticks

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

  pub dbg_dump_options_string: bool,
  pub trace_all_moves: bool,
  pub trace_moves_belt: bool,
  pub trace_moves_machine: bool,
  pub trace_moves_supply: bool,
  pub trace_moves_demand: bool,
  pub cli_factory_output_interval: u64,
  pub trace_auto_layout: bool,
  pub trace_cell_connect: bool,
  pub trace_cell_set_port: bool,
  pub trace_parse_fmd: bool,
  pub trace_get_quote_status: bool,
  pub trace_img_loader: bool,
  pub trace_priority_step: bool,
  pub trace_porting_step: bool,
  pub trace_map_parsing: bool,
  pub dbg_paint_tile_priority: bool, // Print the prio index of a tile in the game (debug, web)
  pub dbg_onload_dump_factory: bool, // Print the CLI version of the floor after generating it initially?
  pub trace_auto_builder: bool,

  pub show_drm: bool, // Draw media where configNode.drm == true ?
  pub dbg_paint_part_borders: bool, // Draw a border around Parts? Helps debugging invisible parts due to sprite problems
  pub dbg_paint_part_char_icon: bool, // Draw the char icon representation for a part on top of it
  pub dbg_paint_part_kind_id: bool, // Draw the part kind id representation for a part on top of it
  pub dbg_paint_port_arrows: bool, // Draw the port directional arrows?
  pub paint_belts: bool, // Paint the belt background tiles?
  pub dbg_paint_belt_id: bool, // Draw the belt id on top of each belt? "dl_r" etc

  pub dbg_paint_zone_hovers: bool, // Draw a rect on the area where the mouse is detected
  pub dbg_paint_zone_borders: bool, // Draw a guide around each grid section of the ui?
  pub dbg_zone_border_color: String, // the color of this border

  pub speed_modifier_floor: f64, // Increase or decrease ticks per second by this rate for (actual factory game/floor) animations
  pub speed_modifier_ui: f64, // Same as speed_modifier_floor but for rest of the UI (buttons, bouncers, trucks, dropzone pulse, etc)
  pub touch_drag_compensation: bool, // Show the mouse pointer 50x50 away from the actual pointer? helpful for dragging on touch screen but can be annoying

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

  pub dbg_maze_enable_runner: bool, // Start the maze runner at all?
  pub dbg_maze_paint_stats_text: bool, // Show actual numbers of prepared maze stats?

  pub splash_keep_loader: bool, // Keep showing the loader screen, even when loading is complete
  pub splash_no_loader: bool, // Skip the loader screen and go straight to the game
  pub splash_keep_main: bool, // Show main menu?
  pub splash_no_main: bool, // Skip main menu?

  pub dbg_animate_cli_output_in_web: bool, // Print the simplified cli output in web version?

  pub initial_event_type_swapped: bool, // sets initial state.event_type_swapped -> MOUSE / TOUCH

  pub dbg_trash_is_joker: bool, // Trash serves as joker item for machines?
  pub dbg_joker_corrupts_factory: bool, // Show visual change when corrupting the factory
  pub dbg_machine_produce_trash: bool, // If a machine trashes a part and expects no inputs, should it output trash instead of discarding it?
  pub dbg_clickable_quests: bool,
  pub dbg_print_quest_states: bool,
  pub dbg_auto_builder_zero_pause: bool,
  pub dbg_auto_builder_zero_duration: bool,

  pub dbg_loop_atom_truck: bool,
  pub dbg_loop_woop_truck: bool,

  pub default_demand_speed: u64,
  pub default_demand_cooldown: u64,

  pub enable_quick_save_menu: bool,
  pub enable_maze_roundway_and_collection: bool,
  pub enable_maze_full: bool,
  pub enable_speed_menu: bool,
  pub dbg_show_secret_menu: bool,
  pub dbg_show_bottom_info: bool,
  pub dbg_show_fps: bool,

  pub test: u64, // just a temporary flag
}

pub fn create_options(speed_modifier_floor: f64, speed_modifier_ui: f64) -> Options {
  return Options {
    options_started_from_source: 0, // Updated elsewhere
    initial_map_from_source: 0, // Updated elsewhere

    dbg_dump_options_string: true,
    trace_all_moves: false,
    trace_moves_belt: false,
    trace_moves_machine: false,
    trace_moves_supply: false,
    trace_moves_demand: false,
    cli_factory_output_interval: 5000,
    trace_auto_layout: false,
    trace_cell_connect: false,
    trace_cell_set_port: false,
    trace_parse_fmd: false,
    trace_get_quote_status: false,
    trace_img_loader: false,
    trace_priority_step: false,
    trace_porting_step: false,
    trace_map_parsing: false,
    dbg_paint_tile_priority: false,
    dbg_onload_dump_factory: false,
    trace_auto_builder: false,
    show_drm: true,
    dbg_paint_part_borders: false,
    dbg_paint_part_char_icon: false,
    dbg_paint_part_kind_id: false,
    dbg_paint_port_arrows: false,
    paint_belts: true,
    dbg_paint_belt_id: false,
    dbg_paint_zone_hovers: false,
    dbg_paint_zone_borders: false,
    dbg_zone_border_color: "white".to_string(),
    speed_modifier_floor,
    speed_modifier_ui,
    touch_drag_compensation: false,
    dropzone_color_offset: 75,
    dropzone_bounce_speed: 100,
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
    dbg_maze_enable_runner: true,
    dbg_maze_paint_stats_text: false,
    splash_keep_loader: false,
    splash_no_loader: false,
    splash_keep_main: false,
    splash_no_main: false,
    dbg_animate_cli_output_in_web: false,
    initial_event_type_swapped: false,
    dbg_trash_is_joker: true,
    dbg_joker_corrupts_factory: true,
    dbg_machine_produce_trash: true,
    dbg_clickable_quests: true,
    dbg_print_quest_states: false,
    dbg_auto_builder_zero_pause: false,
    dbg_auto_builder_zero_duration: false,
    dbg_loop_atom_truck: false,
    dbg_loop_woop_truck: false,

    default_demand_speed: 1000,
    default_demand_cooldown: 500,
    enable_quick_save_menu: true,
    enable_maze_roundway_and_collection: true,
    enable_maze_full: true,
    enable_speed_menu: true,
    dbg_show_secret_menu: true,
    dbg_show_bottom_info: true,
    dbg_show_fps: false,
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
  log!("parse_options_into(options.dbg_dump_options_string={})", options.dbg_dump_options_string);

  let mut verbose = options.dbg_dump_options_string;

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

          if name == "dbg_dump_options_string" { verbose = value == "true"; }
          if verbose { log!("- updating options.{} to `{}`", name, value); }

          match name {
            "dbg_dump_options_string" => options.dbg_dump_options_string = parse_bool(value, name, strict, options.dbg_dump_options_string, verbose),
            "options_started_from_source" => options.options_started_from_source = parse_u64(value, name, strict, options.options_started_from_source, verbose),
            "initial_map_from_source" => options.initial_map_from_source = parse_u64(value, name, strict, options.initial_map_from_source, verbose),
            "trace_all_moves" => options.trace_all_moves = parse_bool(value, name, strict, options.trace_all_moves, verbose),
            "trace_moves_belt" => options.trace_moves_belt = parse_bool(value, name, strict, options.trace_moves_belt, verbose),
            "trace_moves_machine" => options.trace_moves_machine = parse_bool(value, name, strict, options.trace_moves_machine, verbose),
            "trace_moves_supply" => options.trace_moves_supply = parse_bool(value, name, strict, options.trace_moves_supply, verbose),
            "trace_moves_demand" => options.trace_moves_demand = parse_bool(value, name, strict, options.trace_moves_demand, verbose),
            "cli_factory_output_interval" => options.cli_factory_output_interval = parse_u64(value, name, strict, options.cli_factory_output_interval, verbose),
            "trace_auto_layout" => options.trace_auto_layout = parse_bool(value, name, strict, options.trace_auto_layout, verbose),
            "trace_cell_connect" => options.trace_cell_connect = parse_bool(value, name, strict, options.trace_cell_connect, verbose),
            "trace_cell_set_port" => options.trace_cell_set_port = parse_bool(value, name, strict, options.trace_cell_set_port, verbose),
            "trace_parse_fmd" => options.trace_parse_fmd = parse_bool(value, name, strict, options.trace_parse_fmd, verbose),
            "trace_get_quote_status" => options.trace_get_quote_status = parse_bool(value, name, strict, options.trace_get_quote_status, verbose),
            "trace_img_loader" => options.trace_img_loader = parse_bool(value, name, strict, options.trace_img_loader, verbose),
            "trace_priority_step" => options.trace_priority_step = parse_bool(value, name, strict, options.trace_priority_step, verbose),
            "trace_porting_step" => options.trace_porting_step = parse_bool(value, name, strict, options.trace_porting_step, verbose),
            "trace_map_parsing" => options.trace_map_parsing = parse_bool(value, name, strict, options.trace_map_parsing, verbose),
            "dbg_paint_tile_priority" => options.dbg_paint_tile_priority = parse_bool(value, name, strict, options.dbg_paint_tile_priority, verbose),
            "dbg_onload_dump_factory" => options.dbg_onload_dump_factory = parse_bool(value, name, strict, options.dbg_onload_dump_factory, verbose),
            "trace_auto_builder" => options.trace_auto_builder = parse_bool(value, name, strict, options.trace_auto_builder, verbose),
            "show_drm" => options.show_drm = parse_bool(value, name, strict, options.show_drm, verbose),
            "dbg_paint_part_borders" => options.dbg_paint_part_borders = parse_bool(value, name, strict, options.dbg_paint_part_borders, verbose),
            "dbg_paint_part_char_icon" => options.dbg_paint_part_char_icon = parse_bool(value, name, strict, options.dbg_paint_part_char_icon, verbose),
            "dbg_paint_part_kind_id" => options.dbg_paint_part_kind_id = parse_bool(value, name, strict, options.dbg_paint_part_kind_id, verbose),
            "dbg_paint_port_arrows" => options.dbg_paint_port_arrows = parse_bool(value, name, strict, options.dbg_paint_port_arrows, verbose),
            "paint_belts" => options.paint_belts = parse_bool(value, name, strict, options.paint_belts, verbose),
            "dbg_paint_belt_id" => options.dbg_paint_belt_id = parse_bool(value, name, strict, options.dbg_paint_belt_id, verbose),
            "dbg_paint_zone_hovers" => options.dbg_paint_zone_hovers = parse_bool(value, name, strict, options.dbg_paint_zone_hovers, verbose),
            "dbg_paint_zone_borders" => options.dbg_paint_zone_borders = parse_bool(value, name, strict, options.dbg_paint_zone_borders, verbose),
            "dbg_zone_border_color" => options.dbg_zone_border_color = parse_string(value.to_string(), name, strict, options.dbg_zone_border_color.clone(), verbose),
            "bouncer_decay_rate_modifier" => options.bouncer_decay_rate_modifier = parse_f64(value, name, strict, options.bouncer_decay_rate_modifier, verbose),
            "bouncer_amplitude_decay_rate" => options.bouncer_amplitude_decay_rate = parse_f64(value, name, strict, options.bouncer_amplitude_decay_rate, verbose),
            "bouncer_wave_decay_rate" => options.bouncer_wave_decay_rate = parse_f64(value, name, strict, options.bouncer_wave_decay_rate, verbose),
            "bouncer_initial_angle" => options.bouncer_initial_angle = parse_f64(value, name, strict, options.bouncer_initial_angle, verbose),
            "bouncer_angular_freq" => options.bouncer_angular_freq = parse_f64(value, name, strict, options.bouncer_angular_freq, verbose),
            "bouncer_time_to_factory" => options.bouncer_time_to_factory = parse_f64(value, name, strict, options.bouncer_time_to_factory, verbose),
            "speed_modifier_floor" => options.speed_modifier_floor = parse_f64(value, name, strict, options.speed_modifier_floor, verbose),
            "speed_modifier_ui" => options.speed_modifier_ui = parse_f64(value, name, strict, options.speed_modifier_ui, verbose),
            "bouncer_trail_time" => options.bouncer_trail_time = parse_f64(value, name, strict, options.bouncer_trail_time, verbose),
            "bouncer_fade_time" => options.bouncer_fade_time = parse_f64(value, name, strict, options.bouncer_fade_time, verbose),
            "bouncer_stamp_interval" => options.bouncer_stamp_interval = parse_u64(value, name, strict, options.bouncer_stamp_interval, verbose),
            "bouncer_stop_after" => options.bouncer_stop_after = parse_f64(value, name, strict, options.bouncer_stop_after, verbose),
            "bouncer_formula_total_distance" => options.bouncer_formula_total_distance = parse_f64(value, name, strict, options.bouncer_formula_total_distance, verbose),
            "dbg_maze_enable_runner" => options.dbg_maze_enable_runner = parse_bool(value, name, strict, options.dbg_maze_enable_runner, verbose),
            "dbg_maze_paint_stats_text" => options.dbg_maze_paint_stats_text = parse_bool(value, name, strict, options.dbg_maze_paint_stats_text, verbose),
            "splash_keep_loader" => options.splash_keep_loader = parse_bool(value, name, strict, options.splash_keep_loader, verbose),
            "splash_no_loader" => options.splash_no_loader = parse_bool(value, name, strict, options.splash_no_loader, verbose),
            "splash_keep_main" => options.splash_keep_main = parse_bool(value, name, strict, options.splash_keep_main, verbose),
            "splash_no_main" => options.splash_no_main = parse_bool(value, name, strict, options.splash_no_main, verbose),
            "touch_drag_compensation" => options.touch_drag_compensation = parse_bool(value, name, strict, options.touch_drag_compensation, verbose),
            "dropzone_color_offset" => options.dropzone_color_offset = parse_u64(value, name, strict, options.dropzone_color_offset, verbose),
            "dropzone_bounce_speed" => options.dropzone_bounce_speed = parse_u64(value, name, strict, options.dropzone_bounce_speed, verbose),
            "dropzone_bounce_distance" => options.dropzone_bounce_distance = parse_u64(value, name, strict, options.dropzone_bounce_distance, verbose),
            "dbg_animate_cli_output_in_web" => options.dbg_animate_cli_output_in_web = parse_bool(value, name, strict, options.dbg_animate_cli_output_in_web, verbose),
            "initial_event_type_swapped" => options.initial_event_type_swapped = parse_bool(value, name, strict, options.initial_event_type_swapped, verbose),
            "dbg_trash_is_joker" => options.dbg_trash_is_joker = parse_bool(value, name, strict, options.dbg_trash_is_joker, verbose),
            "dbg_joker_corrupts_factory" => options.dbg_joker_corrupts_factory = parse_bool(value, name, strict, options.dbg_joker_corrupts_factory, verbose),
            "dbg_machine_produce_trash" => options.dbg_machine_produce_trash = parse_bool(value, name, strict, options.dbg_machine_produce_trash, verbose),
            "dbg_clickable_quotes" => options.dbg_clickable_quests = parse_bool(value, name, strict, options.dbg_clickable_quests, verbose),
            "dbg_print_quest_states" => options.dbg_print_quest_states = parse_bool(value, name, strict, options.dbg_print_quest_states, verbose),
            "dbg_auto_builder_zero_pause" => options.dbg_auto_builder_zero_pause = parse_bool(value, name, strict, options.dbg_auto_builder_zero_pause, verbose),
            "dbg_auto_builder_zero_duration" => options.dbg_auto_builder_zero_duration = parse_bool(value, name, strict, options.dbg_auto_builder_zero_duration, verbose),
            "dbg_loop_atom_truck" => options.dbg_loop_atom_truck = parse_bool(value, name, strict, options.dbg_loop_atom_truck, verbose),
            "dbg_loop_woop_truck" => options.dbg_loop_woop_truck = parse_bool(value, name, strict, options.dbg_loop_woop_truck, verbose),
            "default_demand_speed" => options.default_demand_speed = parse_u64(value, name, strict, options.default_demand_speed, verbose),
            "default_demand_cooldown" => options.default_demand_cooldown = parse_u64(value, name, strict, options.default_demand_cooldown, verbose),
            "enable_quick_save_menu" => options.enable_quick_save_menu = parse_bool(value, name, strict, options.enable_quick_save_menu, verbose),
            "enable_maze_roundway_and_collection" => options.enable_maze_roundway_and_collection = parse_bool(value, name, strict, options.enable_maze_roundway_and_collection, verbose),
            "enable_maze_full" => options.enable_maze_full = parse_bool(value, name, strict, options.enable_maze_full, verbose),
            "enable_speed_menu" => options.enable_speed_menu = parse_bool(value, name, strict, options.enable_speed_menu, verbose),
            "dbg_show_secret_menu" => options.dbg_show_secret_menu = parse_bool(value, name, strict, options.dbg_show_secret_menu, verbose),
            "dbg_show_bottom_info" => options.dbg_show_bottom_info = parse_bool(value, name, strict, options.dbg_show_bottom_info, verbose),
            "dbg_show_fps" => options.dbg_show_fps = parse_bool(value, name, strict, options.dbg_show_fps, verbose),
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
  arr.push(format!("- dbg_dump_options_string: {}", options.dbg_dump_options_string));
  arr.push(format!("- trace_all_moves: {}", options.trace_all_moves));
  arr.push(format!("- trace_moves_belt: {}", options.trace_moves_belt));
  arr.push(format!("- trace_moves_machine: {}", options.trace_moves_machine));
  arr.push(format!("- trace_moves_supply: {}", options.trace_moves_supply));
  arr.push(format!("- trace_moves_demand: {}", options.trace_moves_demand));
  arr.push(format!("- cli_factory_output_interval: {}", options.cli_factory_output_interval));
  arr.push(format!("- trace_auto_layout: {}", options.trace_auto_layout));
  arr.push(format!("- trace_cell_connect: {}", options.trace_cell_connect));
  arr.push(format!("- trace_cell_set_port: {}", options.trace_cell_set_port));
  arr.push(format!("- trace_parse_fmd: {}", options.trace_parse_fmd));
  arr.push(format!("- trace_get_quote_status: {}", options.trace_get_quote_status));
  arr.push(format!("- trace_img_loader: {}", options.trace_img_loader));
  arr.push(format!("- trace_priority_step: {}", options.trace_priority_step));
  arr.push(format!("- trace_porting_step: {}", options.trace_porting_step));
  arr.push(format!("- trace_map_parsing: {}", options.trace_map_parsing));
  arr.push(format!("- dbg_paint_tile_priority: {}", options.dbg_paint_tile_priority));
  arr.push(format!("- dbg_onload_dump_factory: {}", options.dbg_onload_dump_factory));
  arr.push(format!("- trace_auto_builder: {}", options.trace_auto_builder));
  arr.push(format!("- show_drm: {}", options.show_drm));
  arr.push(format!("- dbg_paint_part_borders: {}", options.dbg_paint_part_borders));
  arr.push(format!("- dbg_paint_part_char_icon: {}", options.dbg_paint_part_char_icon));
  arr.push(format!("- dbg_paint_part_kind_id: {}", options.dbg_paint_part_kind_id));
  arr.push(format!("- dbg_paint_port_arrows: {}", options.dbg_paint_port_arrows));
  arr.push(format!("- paint_belts: {}", options.paint_belts));
  arr.push(format!("- dbg_paint_belt_id: {}", options.dbg_paint_belt_id));
  arr.push(format!("- dbg_paint_zone_hovers: {}", options.dbg_paint_zone_hovers));
  arr.push(format!("- dbg_paint_zone_borders: {}", options.dbg_paint_zone_borders));
  arr.push(format!("- dbg_zone_border_color: '{}'", options.dbg_zone_border_color));
  arr.push(format!("- speed_modifier_floor: {}", options.speed_modifier_floor));
  arr.push(format!("- speed_modifier_ui: {}", options.speed_modifier_ui));
  arr.push(format!("- touch_drag_compensation: {}", options.touch_drag_compensation));
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
  arr.push(format!("- dbg_maze_enable_runner: {}", options.dbg_maze_enable_runner));
  arr.push(format!("- dbg_maze_paint_stats_text: {}", options.dbg_maze_paint_stats_text));
  arr.push(format!("- splash_keep_loader: {}", options.splash_keep_loader));
  arr.push(format!("- splash_no_loader: {}", options.splash_no_loader));
  arr.push(format!("- splash_keep_main: {}", options.splash_keep_main));
  arr.push(format!("- splash_no_main: {}", options.splash_no_main));
  arr.push(format!("- dbg_animate_cli_output_in_web: {}", options.dbg_animate_cli_output_in_web));
  arr.push(format!("- initial_event_type_swapped: {}", options.initial_event_type_swapped));
  arr.push(format!("- dbg_trash_is_joker: {}", options.dbg_trash_is_joker));
  arr.push(format!("- dbg_joker_corrupts_factory: {}", options.dbg_joker_corrupts_factory));
  arr.push(format!("- dbg_machine_produce_trash: {}", options.dbg_machine_produce_trash));
  arr.push(format!("- dbg_clickable_quotes: {}", options.dbg_clickable_quests));
  arr.push(format!("- dbg_print_quest_states: {}", options.dbg_print_quest_states));
  arr.push(format!("- dbg_auto_builder_zero_pause: {}", options.dbg_auto_builder_zero_pause));
  arr.push(format!("- dbg_auto_builder_zero_duration: {}", options.dbg_auto_builder_zero_duration));
  arr.push(format!("- dbg_loop_atom_truck: {}", options.dbg_loop_atom_truck));
  arr.push(format!("- dbg_loop_woop_truck: {}", options.dbg_loop_woop_truck));
  arr.push(format!("- default_demand_speed: {}", options.default_demand_speed));
  arr.push(format!("- default_demand_cooldown: {}", options.default_demand_cooldown));
  arr.push(format!("- enable_quick_save_menu: {}", options.enable_quick_save_menu));
  arr.push(format!("- enable_maze_roundway_and_collection: {}", options.enable_maze_roundway_and_collection));
  arr.push(format!("- enable_maze_full: {}", options.enable_maze_full));
  arr.push(format!("- enable_speed_menu: {}", options.enable_speed_menu));
  arr.push(format!("- dbg_show_secret_menu: {}", options.dbg_show_secret_menu));
  arr.push(format!("- dbg_show_bottom_info: {}", options.dbg_show_bottom_info));
  arr.push(format!("- dbg_show_fps: {}", options.dbg_show_fps));
  arr.push(format!("- test: {}", options.test));
  return arr.join("\n");
}

pub fn factory_ticks_to_game_ui_time(options: &Options, ticks: u64) -> f64 {
  return (ticks as f64 / (ONE_SECOND as f64 * options.speed_modifier_floor)) * options.speed_modifier_ui;
}
