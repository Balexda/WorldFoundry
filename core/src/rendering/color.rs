use super::tile::RgbaColor;

/// Color palette for rendering different terrain types
pub struct ColorPalette {
    // Water colors (deep to shallow)
    deep_water: RgbaColor,
    shallow_water: RgbaColor,
    
    // Land colors
    desert: RgbaColor,
    grassland: RgbaColor,
    forest: RgbaColor,
    mountain: RgbaColor,
    snow: RgbaColor,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            deep_water: RgbaColor::rgb(25, 55, 109),      // Deep blue
            shallow_water: RgbaColor::rgb(65, 105, 225),   // Royal blue
            desert: RgbaColor::rgb(238, 203, 173),         // Sandy brown
            grassland: RgbaColor::rgb(124, 252, 0),        // Lawn green
            forest: RgbaColor::rgb(34, 139, 34),           // Forest green
            mountain: RgbaColor::rgb(139, 137, 137),       // Dark gray
            snow: RgbaColor::rgb(255, 250, 250),           // Snow white
        }
    }
}

impl ColorPalette {
    /// Get water color based on depth (height < 0.2)
    pub fn water_color(&self, height: f64) -> RgbaColor {
        if height < 0.1 {
            self.deep_water
        } else {
            // Interpolate between deep and shallow water
            let t = (height - 0.1) / 0.1; // 0.1 to 0.2 -> 0.0 to 1.0
            self.interpolate_color(self.deep_water, self.shallow_water, t)
        }
    }
    
    /// Get desert color based on temperature and precipitation
    pub fn desert_color(&self, temperature: f64, precipitation: f64) -> RgbaColor {
        // Vary desert color based on conditions
        let intensity = (temperature * (1.0 - precipitation)).min(1.0);
        let r = (self.desert.r as f64 * (0.7 + 0.3 * intensity)) as u8;
        let g = (self.desert.g as f64 * (0.8 + 0.2 * intensity)) as u8;
        let b = (self.desert.b as f64 * (0.9 + 0.1 * intensity)) as u8;
        RgbaColor::rgb(r, g, b)
    }
    
    /// Get snow/ice color based on temperature
    pub fn snow_color(&self, temperature: f64) -> RgbaColor {
        // Colder = more blue tint
        let blue_tint = (1.0 - temperature) * 0.3;
        let r = (self.snow.r as f64 * (1.0 - blue_tint * 0.1)) as u8;
        let g = (self.snow.g as f64 * (1.0 - blue_tint * 0.1)) as u8;
        let b = (self.snow.b as f64 * (1.0 - blue_tint * 0.05)) as u8;
        RgbaColor::rgb(r, g, b)
    }
    
    /// Get mountain color based on height
    pub fn mountain_color(&self, height: f64) -> RgbaColor {
        // Higher mountains are darker/more rocky
        let darkness = ((height - 0.8) / 0.2).min(1.0); // 0.8 to 1.0 -> 0.0 to 1.0
        let factor = 1.0 - darkness * 0.3;
        let r = (self.mountain.r as f64 * factor) as u8;
        let g = (self.mountain.g as f64 * factor) as u8;
        let b = (self.mountain.b as f64 * factor) as u8;
        RgbaColor::rgb(r, g, b)
    }
    
    /// Get forest color based on precipitation
    pub fn forest_color(&self, precipitation: f64) -> RgbaColor {
        // More precipitation = darker/lusher green
        let lushness = precipitation.min(1.0);
        let r = (self.forest.r as f64 * (1.0 - lushness * 0.2)) as u8;
        let g = (self.forest.g as f64 * (0.8 + lushness * 0.2)) as u8;
        let b = (self.forest.b as f64 * (1.0 - lushness * 0.1)) as u8;
        RgbaColor::rgb(r, g, b)
    }
    
    /// Get grassland color based on precipitation
    pub fn grassland_color(&self, precipitation: f64) -> RgbaColor {
        // Less precipitation = more yellow/brown tint
        let dryness = 1.0 - precipitation.min(1.0);
        let r = (self.grassland.r as f64 + dryness * 50.0).min(255.0) as u8;
        let g = self.grassland.g;
        let b = (self.grassland.b as f64 * (1.0 - dryness * 0.5)) as u8;
        RgbaColor::rgb(r, g, b)
    }
    
    /// Interpolate between two colors
    fn interpolate_color(&self, color1: RgbaColor, color2: RgbaColor, t: f64) -> RgbaColor {
        let t = t.max(0.0).min(1.0);
        let r = (color1.r as f64 * (1.0 - t) + color2.r as f64 * t) as u8;
        let g = (color1.g as f64 * (1.0 - t) + color2.g as f64 * t) as u8;
        let b = (color1.b as f64 * (1.0 - t) + color2.b as f64 * t) as u8;
        let a = (color1.a as f64 * (1.0 - t) + color2.a as f64 * t) as u8;
        RgbaColor::new(r, g, b, a)
    }
}