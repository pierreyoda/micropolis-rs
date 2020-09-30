use super::City;
use crate::game::{GameSpeed, GameSpeedPreset};
use crate::map::{MapPosition, TileType};

pub struct Simulation {
    speed: GameSpeed,
    speed_cycle: u16,
    phase_cycle: u8,
    /// Number of passes through the similator loop.
    passes: u32,
    /// Current simulator loop pass counter.
    pass_index: usize,
    /// Incremented every time the map changes.
    map_serial: u32,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            speed: GameSpeed::from(GameSpeedPreset::Normal),
            speed_cycle: 0,
            phase_cycle: 0,
            passes: 0,
            pass_index: 0,
            map_serial: 1,
        }
    }
    pub fn reset_pass_counter(&mut self) {
        self.pass_index = 0;
    }

    pub fn on_map_updated(&mut self) {
        self.map_serial += 1;
    }

    /// Advance the city simulation and its visualization by one frame tick.
    pub fn step(&mut self, city: &mut City) -> Result<(), String> {
        let sim_steps_per_update = self.speed.get_sim_steps_per_update();
        if sim_steps_per_update == 0 {
            return Ok(());
        }
        self.speed_cycle += 1;
        if self.speed_cycle == 1024 {
            self.speed_cycle = 0;
        }
        match sim_steps_per_update {
            1 if self.speed_cycle % 5 != 0 => return Ok(()),
            2 if self.speed_cycle % 3 != 0 => return Ok(()),
            _ => {}
        }

        self.simulate(city)
    }

    fn simulate(&mut self, city: &mut City) -> Result<(), String> {
        // The simulator has 16 different phases, which we cycle through
        // according to `phase_cycle`, which is incremented and wrapped at the
        // end of this switch.

        let map_size = city.map.bounds();

        // TODO: initSimLoad behavior
        self.phase_cycle &= 15;
        // TODO: replace phase_cycle integer by enum?
        match self.phase_cycle {
            0 => {}
            // Scan 1/8th of the map for each of these 8 phases
            1..=8 => {
                let phase_cycle = self.phase_cycle as usize;
                self.scan_map_section(
                    city,
                    (phase_cycle - 1) * map_size.get_width() / 8,
                    phase_cycle * map_size.get_height() / 8,
                )?
            }
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            _ => unreachable!(),
        };
        self.phase_cycle = (self.phase_cycle + 1) & 15;

        Ok(())
    }

    fn scan_map_section(&self, city: &mut City, x1: usize, x2: usize) -> Result<(), String> {
        let map_height = city.map.bounds().get_height();
        let flood_type_raw = TileType::Flood
            .to_u16()
            .ok_or("Flood tile type raw conversion error")?;
        for x in x1..x2 {
            for y in 0..map_height {
                let tile = city
                    .map
                    .tiles()
                    .get(x)
                    .ok_or(format!(
                        "Simulation.scan_map_section map X overflow at {}",
                        x
                    ))?
                    .get(y)
                    .ok_or(format!(
                        "Simulation.scan_map_section map Y overflow at {}",
                        y
                    ))?;
                if tile.get_type() == &Some(TileType::Dirt) {
                    continue;
                }

                let tile_type_raw = tile.get_type_raw();
                if tile_type_raw < flood_type_raw {
                    continue;
                }

                let position = MapPosition::new(x as i32, y as i32);
                if tile_type_raw < TileType::HorizontalBridge as u16
                    && tile_type_raw >= TileType::Fire as u16
                {
                    city.fires_count += 1;
                }
            }
        }

        Ok(())
    }
}
