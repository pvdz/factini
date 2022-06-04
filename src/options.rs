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
  pub trace_priority_step: bool,

  pub short_term_window: u64, // For stats; average over this many ticks
  pub long_term_window: u64, // For stats; average over this many ticks

  pub speed_modifier: f64, // Increase or decrease ticks per second by this rate

  pub web_output_cli: bool, // Print the simplified cli output in web version?

  pub paint_cell_segment_grid: bool, // For each non-empty cell print a 9x9 sub-grid?
}

pub fn create_options(speed_modifier: f64) -> Options {
  return Options {
    print_choices: false,
    print_choices_belt: false,
    print_choices_machine: true,
    print_choices_supply: false,
    print_choices_demand: false,
    print_moves: false,
    print_moves_belt: false,
    print_moves_machine: true,
    print_moves_supply: false,
    print_moves_demand: false,
    print_price_deltas: false,
    print_machine_actions: false,
    print_factory_interval: 10000,
    print_stats_interval: 100000,
    trace_priority_step: false,
    short_term_window: 10000,
    long_term_window: 600000,
    speed_modifier,
    web_output_cli: false,
    paint_cell_segment_grid: false,
  };
}

pub const ONE_MS: u64 = 5; // design is 10
pub const ONE_SECOND: u64 = 1000 * ONE_MS;
pub const MAX_TICKS_PER_FRAME: u64 = 1000;

pub const FLOOR_CELLS_W: usize = 1 + 5*3 + 1;
pub const FLOOR_CELLS_H: usize = 1 + 5*3 + 1;
pub const FLOOR_CELLS_WH: usize = FLOOR_CELLS_W * FLOOR_CELLS_H;
