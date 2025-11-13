use sysinfo::{System,Networks,Components};

pub struct App {
    
    pub should_quit: bool,
  
    pub cpu_usage: Vec<(f64, f64)>,
 
    pub memory_usage: f64,
   
    pub network_rx: u64,
   
    pub network_tx: u64,
 
    pub window_width: f64,

       pub core_temp: f32,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            cpu_usage: vec![],
            memory_usage: 0.0,
            network_rx: 0,
            network_tx: 0,
            window_width: 0.0,
            core_temp: 0.0,
        }
    }

    /// Called on every "tick" of the application to update state.
    pub fn on_tick(&mut self, sys: &System,networks: &Networks,components:&Components) {
        self.window_width += 1.0;

        // 1. CPU Handling 
        let usage = sys.global_cpu_info().cpu_usage() as f64;
        self.cpu_usage.push((self.window_width, usage));

        if self.cpu_usage.len() > 100 {
            self.cpu_usage.remove(0);
        }

        // 2. Memory Handling
        let total_mem = sys.total_memory() as f64;
        let used_mem = sys.used_memory() as f64;
        self.memory_usage = if total_mem > 0.0 { (used_mem / total_mem) * 100.0 } else { 0.0 };

        // 3. Network Handling
        let (rx, tx) = networks.iter().fold((0, 0), |acc, (_, data)| {
        (acc.0 + data.total_received(), acc.1 + data.total_transmitted())
});
        self.network_rx = rx;
         self.network_tx = tx;

          if self.window_width == 1.0 {
            let all_sensors = components;
            if all_sensors.is_empty() {
                println!("\nDEBUG: No components (sensors) found by sysinfo.");
                println!("(On Windows, try 'Run as Administrator'. On macOS, this is common.)\n");
            } else {
                println!("\nDEBUG: Found Components (Sensors):");
                for component in all_sensors {
                    println!(
                        "  - Label: '{}', Temp: {}°C",
                        component.label(),
                        component.temperature()
                    );
                }
                println!();
            }
        }

         // 4. Core Temperature Handling
        let core_temp_component = components
            .iter()
            .find(|c| {
                let label = c.label().to_lowercase();
                label.contains("core") || label.contains("cpu")
            })
            .or_else(|| components.first()); // Fallback to first component

        if let Some(component) = core_temp_component {
            self.core_temp = component.temperature();
        } else {
            self.core_temp = 0.0; // No components found
        }
    }
}