use std::str::FromStr;

pub struct Category {}

impl FromStr for Category {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .replace("map:", "")
            .split("-to-")
            .map(|i| i.trim().to_owned())
            .collect::<Vec<_>>();

        if map.len() != 2 {
            Err(())
        } else {
            Ok(Category {})
        }
    }
}
