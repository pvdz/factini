pub struct Options {
  pub print_moves: bool,
  pub print_moves_belt: bool,
  pub print_moves_machine: bool,
  pub print_moves_supply: bool,
  pub print_moves_demand: bool,
  pub print_price_deltas: bool,
  pub print_factory_interval: u64,
  pub short_term_window: u64,
  pub long_term_window: u64,
  pub speed_modifier: f64, // Increase or decrease ticks per second by this rate
  pub web_output_cli: bool, // Print the simplified cli output in web version?
}

pub fn create_options() -> Options {
  return Options {
    print_moves: false,
    print_moves_belt: false,
    print_moves_machine: true,
    print_moves_supply: false,
    print_moves_demand: false,
    print_price_deltas: false,
    print_factory_interval: 10000,
    short_term_window: 10000,
    long_term_window: 600000,
    speed_modifier: 1.0,
    web_output_cli: false,
  };
}

pub const ONE_MS: u64 = 5; // design is 10
pub const ONE_SECOND: u64 = 1000 * ONE_MS;
pub const MAX_TICKS_PER_FRAME: u64 = 1000;
