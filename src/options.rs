use super::utils::*;
use super::log;

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
  pub trace_priority_step: bool,
  pub trace_porting_step: bool,
  pub trace_map_parsing: bool,
  pub print_priority_tile_order: bool, // Print the prio index of a tile in the game (debug, web)
  pub print_initial_table: bool, // Print the CLI version of the floor after generating it initially?

  pub draw_part_borders: bool, // Draw a border around Parts? Helps debugging invisible parts due to sprite problems
  pub draw_part_char_icon: bool, // Draw the char icon representation for a part on top of it
  pub draw_part_kind: bool, // Draw the part kind id representation for a part on top of it
  pub draw_port_arrows: bool, // Draw the port directional arrows?
  pub paint_belts: bool, // Paint the belt background tiles?
  pub draw_belt_dbg_id: bool, // Draw the belt id on top of each belt? "dl_r" etc
  pub draw_zone_hovers: bool, // Draw a rect on the area where the mouse is detected

  pub draw_ui_section_border: bool, // Draw a guide around each grid section of the ui?
  pub ui_section_border_color: String, // the color of this border

  pub short_term_window: u64, // For stats; average over this many ticks
  pub long_term_window: u64, // For stats; average over this many ticks

  pub speed_modifier: f64, // Increase or decrease ticks per second by this rate
  pub touch_drag_compensation: bool, // Show the mouse pointer 50x50 away from the actual pointer? helpful for dragging on touch screen but can be annoying

  pub web_output_cli: bool, // Print the simplified cli output in web version?

  pub dbg_trash_is_joker: bool, // Trash serves as joker item for machines?
  pub db_joker_corrupts_factory: bool, // Show visual change when corrupting the factory
  pub dbg_machine_produce_trash: bool, // If a machine trashes a part and expects no inputs, should it output trash instead of discarding it?
  pub dbg_clickable_quotes: bool,

  pub test: u64, // just a temporary flag
}

pub fn create_options(speed_modifier: f64) -> Options {
  return Options {
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
    trace_priority_step: false,
    trace_porting_step: false,
    trace_map_parsing: false,
    print_priority_tile_order: false,
    print_initial_table: false,
    draw_part_borders: false,
    draw_part_char_icon: false,
    draw_part_kind: false,
    draw_port_arrows: false,
    paint_belts: true,
    draw_belt_dbg_id: false,
    draw_zone_hovers: false,
    draw_ui_section_border: false,
    ui_section_border_color: "white".to_string(),
    short_term_window: 10000,
    long_term_window: 600000,
    speed_modifier,
    touch_drag_compensation: false,
    web_output_cli: false,
    dbg_trash_is_joker: true,
    db_joker_corrupts_factory: true,
    dbg_machine_produce_trash: true,
    dbg_clickable_quotes: true,
    test: 0,
  };
}

fn parse_bool(value: &str, key: &str, strict: bool, def: bool) -> bool {
  match value {
    "true" => true,
    "false" => false,
    _ => {
      if strict {
        panic!("Invalid value for options.{}; expecting a boolean, received `{}`", key, value)
      } else {
        log!("Invalid value for options.{}; expecting a boolean, received `{}`", key, value);
        def
      }
    },
  }
}

fn parse_f64(value: &str, key: &str, strict: bool, def: f64) -> f64 {
  let b =
    if !value.contains(".") {
      format!("{}.0", value)
    } else {
      value.to_string()
    };

  let t = b.parse::<f64>();
  if strict {
    return t.expect(format!("The value for options.{} must be a number, received `{}`", key, b).as_str());
  } else {
    return t.or::<f64>(Ok(def)).unwrap();
  }
}

fn parse_u64(value: &str, key: &str, strict: bool, def: u64) -> u64 {
  return parse_f64(value, key, strict, def as f64) as u64;
}

fn parse_string(value: &str, key: &str, strict: bool, def: &String) -> String {
  if value.starts_with("\"") {
    if !value.ends_with("\"") {
      if strict {
        panic!("Missing double quote at end of value; options.{}, value = `{}`", key, value);
      } else {
        log!("Missing double quote at end of value; options.{}, value = `{}`", key, value);
        return def.clone();
      }
    }
  }
  else if value.starts_with("'") {
    if !value.ends_with("'") {
      if strict {
        panic!("Missing single quote at end of value; options.{}, value = `{}`", key, value);
      } else {
        log!("Missing single quote at end of value; options.{}, value = `{}`", key, value);
        return def.clone();
      }
    }
  }
  else {
    if strict {
      panic!("Unable to parse string for options.{}; value was `{}`", key, value);
    } else {
      log!("Unable to parse string for options.{}; value was `{}`", key, value);
      return def.clone();
    }
  }

  return format!("{}", value)[1..value.len()-1].to_string();
}

pub fn parse_options_into(input: String, options: &mut Options, strict: bool) {
  log!("parse_options_into()");

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
          log!("- updating options.{} to `{}`", name, value);

          match name {
            "print_choices" => options.print_choices = parse_bool(value, name, strict, options.print_choices),
            "print_choices_belt" => options.print_choices_belt = parse_bool(value, name, strict, options.print_choices_belt),
            "print_choices_machine" => options.print_choices_machine = parse_bool(value, name, strict, options.print_choices_machine),
            "print_choices_supply" => options.print_choices_supply = parse_bool(value, name, strict, options.print_choices_supply),
            "print_choices_demand" => options.print_choices_demand = parse_bool(value, name, strict, options.print_choices_demand),
            "print_moves" => options.print_moves = parse_bool(value, name, strict, options.print_moves),
            "print_moves_belt" => options.print_moves_belt = parse_bool(value, name, strict, options.print_moves_belt),
            "print_moves_machine" => options.print_moves_machine = parse_bool(value, name, strict, options.print_moves_machine),
            "print_moves_supply" => options.print_moves_supply = parse_bool(value, name, strict, options.print_moves_supply),
            "print_moves_demand" => options.print_moves_demand = parse_bool(value, name, strict, options.print_moves_demand),
            "print_price_deltas" => options.print_price_deltas = parse_bool(value, name, strict, options.print_price_deltas),
            "print_machine_actions" => options.print_machine_actions = parse_bool(value, name, strict, options.print_machine_actions),
            "print_factory_interval" => options.print_factory_interval = parse_u64(value, name, strict, options.print_factory_interval),
            "print_stats_interval" => options.print_stats_interval = parse_u64(value, name, strict, options.print_stats_interval),
            "print_auto_layout_debug" => options.print_auto_layout_debug = parse_bool(value, name, strict, options.print_auto_layout_debug),
            "print_fmd_trace" => options.print_fmd_trace = parse_bool(value, name, strict, options.print_fmd_trace),
            "trace_priority_step" => options.trace_priority_step = parse_bool(value, name, strict, options.trace_priority_step),
            "trace_porting_step" => options.trace_porting_step = parse_bool(value, name, strict, options.trace_porting_step),
            "trace_map_parsing" => options.trace_map_parsing = parse_bool(value, name, strict, options.trace_map_parsing),
            "print_priority_tile_order" => options.print_priority_tile_order = parse_bool(value, name, strict, options.print_priority_tile_order),
            "print_initial_table" => options.print_initial_table = parse_bool(value, name, strict, options.print_initial_table),
            "draw_part_borders" => options.draw_part_borders = parse_bool(value, name, strict, options.draw_part_borders),
            "draw_part_char_icon" => options.draw_part_char_icon = parse_bool(value, name, strict, options.draw_part_char_icon),
            "draw_part_kind" => options.draw_part_kind = parse_bool(value, name, strict, options.draw_part_kind),
            "draw_port_arrows" => options.draw_port_arrows = parse_bool(value, name, strict, options.draw_port_arrows),
            "paint_belts" => options.paint_belts = parse_bool(value, name, strict, options.paint_belts),
            "draw_belt_dbg_id" => options.draw_belt_dbg_id = parse_bool(value, name, strict, options.draw_belt_dbg_id),
            "draw_zone_hovers" => options.draw_zone_hovers = parse_bool(value, name, strict, options.draw_zone_hovers),
            "draw_ui_section_border" => options.draw_ui_section_border = parse_bool(value, name, strict, options.draw_ui_section_border),
            "ui_section_border_color" => options.ui_section_border_color = parse_string(value, name, strict, &options.ui_section_border_color),
            "short_term_window" => options.short_term_window = parse_u64(value, name, strict, options.short_term_window),
            "long_term_window" => options.long_term_window = parse_u64(value, name, strict, options.long_term_window),
            "speed_modifier" => options.speed_modifier = parse_f64(value, name, strict, options.speed_modifier),
            "touch_drag_compensation" => options.touch_drag_compensation = parse_bool(value, name, strict, options.touch_drag_compensation),
            "web_output_cli" => options.web_output_cli = parse_bool(value, name, strict, options.web_output_cli),
            "dbg_trash_is_joker" => options.dbg_trash_is_joker = parse_bool(value, name, strict, options.dbg_trash_is_joker),
            "db_joker_corrupts_factory" => options.db_joker_corrupts_factory = parse_bool(value, name, strict, options.db_joker_corrupts_factory),
            "dbg_machine_produce_trash" => options.dbg_machine_produce_trash = parse_bool(value, name, strict, options.dbg_machine_produce_trash),
            "dbg_clickable_quotes" => options.dbg_clickable_quotes = parse_bool(value, name, strict, options.dbg_clickable_quotes),
            "test" => options.test = parse_u64(value, name, strict, options.test),
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
