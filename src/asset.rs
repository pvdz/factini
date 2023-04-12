use super::config::*;
use super::utils::*;
use super::log;

pub fn asset_to_sprite_coord_from_config(config: &Config, config_index: usize) -> (f64, f64, f64, f64, &web_sys::HtmlImageElement ) {
  assert!(config_index < config.nodes.len(), "part kind should be a node index: {} < {}", config_index, config.nodes.len());

  return config_get_sprite_details(config, config_index, 0, 0);
}
