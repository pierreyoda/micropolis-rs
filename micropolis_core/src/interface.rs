use rand::Rng;

use crate::{
    city::City,
    map::{tools::apply_tool, tools::tool_down, tools::EditingTool, MapPosition, TileMap},
};

pub struct MicropolisCoreOptions {
    animations_enabled: bool,
    sound_enabled: bool,
    auto_bulldoze: bool,
}

pub struct MicropolisCoreInterfacer {
    city: City,
    options: MicropolisCoreOptions,
}

impl MicropolisCoreInterfacer {
    pub fn tool_down<R: Rng>(
        &mut self,
        rng: &mut R,
        position: &MapPosition,
        tool: &EditingTool,
    ) -> Result<(), String> {
        let total_funds = self.city.total_funds();
        tool_down(
            rng,
            self.city.get_map_mut(),
            position,
            tool,
            self.options.auto_bulldoze,
            self.options.animations_enabled,
            total_funds,
        )?;

        self.city.get_sim_mut().reset_pass_counter();
        self.city.invalidate_map();

        Ok(())
    }

    /// Drag a tool from one tile to another.
    pub fn tool_drag<R: Rng>(
        &mut self,
        rng: &mut R,
        map: &mut TileMap,
        from: &MapPosition,
        to: &MapPosition,
        tool: &EditingTool,
        auto_bulldoze: bool,
        animations_enabled: bool,
        total_funds: u32,
    ) -> Result<(), String> {
        // do not drag big tools
        if tool.clone().size() > 1 {
            apply_tool(
                rng,
                map,
                to,
                tool,
                auto_bulldoze,
                animations_enabled,
                total_funds,
            )?;

            self.city.get_sim_mut().reset_pass_counter(); // update editors overlapping this one
            self.city.invalidate_map();
            return Ok(());
        }

        if from == to {
            return Ok(());
        }
        // ensure the start position is done
        apply_tool(
            rng,
            map,
            from,
            tool,
            auto_bulldoze,
            animations_enabled,
            total_funds,
        )?;
        let direction = (*to - *from).unitary(-1);
        let (mut current_from, mut current_to) = (from.clone(), to.clone());

        match from.axis_equalities_with(to) {
            // vertical line up or down
            (true, false) => {
                while current_from != current_to {
                    current_from = current_from.with_y_offset(direction.get_y() as i8);
                    apply_tool(
                        rng,
                        map,
                        &current_from,
                        tool,
                        auto_bulldoze,
                        animations_enabled,
                        total_funds,
                    )?;
                }
            }
            // horizontal line left or right
            (false, true) => {
                while current_from != current_to {
                    current_from = current_from.with_y_offset(direction.get_x() as i8);
                    apply_tool(
                        rng,
                        map,
                        &current_from,
                        tool,
                        auto_bulldoze,
                        animations_enabled,
                        total_funds,
                    )?;
                }
            }
            // general case: rectangle dragging (both X & Y change)
            _ => {
                let delta = (*to - *from).unitary(0).absolute();
                let sub_steps_count = delta.minimum_axis();
                let (mut sub_x, mut sub_y) = (0, 0); // each X/Y step is DX/DY sub-steps
                while current_from != current_to {
                    sub_x += sub_steps_count;
                    if sub_x >= delta.get_y() {
                        sub_x -= delta.get_y();
                        current_from = current_from.with_x_offset(direction.get_x() as i8);
                        apply_tool(
                            rng,
                            map,
                            &current_from,
                            tool,
                            auto_bulldoze,
                            animations_enabled,
                            total_funds,
                        )?;
                    }

                    sub_y += sub_steps_count;
                    if sub_y >= delta.get_x() {
                        sub_y -= delta.get_x();
                        current_from = current_from.with_x_offset(direction.get_y() as i8);
                        apply_tool(
                            rng,
                            map,
                            &current_from,
                            tool,
                            auto_bulldoze,
                            animations_enabled,
                            total_funds,
                        )?;
                    }
                }
            }
        }

        self.city.get_sim_mut().reset_pass_counter(); // update editors overlapping this one
        self.city.invalidate_map();
        Ok(())
    }
}
