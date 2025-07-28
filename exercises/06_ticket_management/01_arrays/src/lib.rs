// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.
use std::ops::{Index, IndexMut};

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Default, Clone, Copy)]
struct WeekTemperatures([Option<i32>; 7]);

impl Index<Weekday> for WeekTemperatures {
    type Output = Option<i32>;

    fn index(&self, day: Weekday) -> &Self::Output {
        match day {
            Weekday::Monday => &self.0[0], // Access the inner array directly
            Weekday::Tuesday => &self.0[1],
            Weekday::Wednesday => &self.0[2],
            Weekday::Thursday => &self.0[3],
            Weekday::Friday => &self.0[4],
            Weekday::Saturday => &self.0[5],
            Weekday::Sunday => &self.0[6],
        }
    }
}

impl IndexMut<Weekday> for WeekTemperatures {
    fn index_mut(&mut self, day: Weekday) -> &mut Self::Output {
        match day {
            Weekday::Monday => &mut self.0[0],
            Weekday::Tuesday => &mut self.0[1],
            Weekday::Wednesday => &mut self.0[2],
            Weekday::Thursday => &mut self.0[3],
            Weekday::Friday => &mut self.0[4],
            Weekday::Saturday => &mut self.0[5],
            Weekday::Sunday => &mut self.0[6],
        }
    }
}

impl WeekTemperatures {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        self[day]
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        self[day] = Some(temperature);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
