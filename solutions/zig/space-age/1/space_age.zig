pub const Planet = enum {
    mercury,
    venus,
    earth,
    mars,
    jupiter,
    saturn,
    uranus,
    neptune,

    pub fn age(self: Planet, seconds: usize) f64 {
        const earth_year_seconds: f64 = 31_557_600.0;
        return @as(f64, @floatFromInt(seconds)) / (earth_year_seconds * self.orbitalPeriod());
    }

    fn orbitalPeriod(self: Planet) f64 {
        return switch (self) {
            Planet.mercury => 0.2408467,
            Planet.venus => 0.61519726,
            Planet.earth => 1.0,
            Planet.mars => 1.8808158,
            Planet.jupiter => 11.862615,
            Planet.saturn => 29.447498,
            Planet.uranus => 84.016846,
            Planet.neptune => 164.79132,
        };
    }
};
