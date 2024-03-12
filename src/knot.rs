use crate::colourings;
use crate::config::Config;
use crate::parser::{parse_segment, split_string};

#[derive(Debug)]
pub struct Knot {
    pub n: i32,
    pub p: i32,
    pub braid_rep: Vec<(i32, i32)>, //(top,bottom)
}

impl Knot {
    pub fn new(config: Config) -> Knot {
        let str_rep = split_string(&config.braid_rep);
        let mut braid_rep: Vec<(i32, i32)> = Vec::new();
        for braid in str_rep.iter() {
            braid_rep.push(parse_segment(braid));
        }
        Knot {
            n: config.n,
            braid_rep,
            p: config.p,
        }
    }
    pub fn count_colourings(&self) -> i32 {
        let starting_rows = colourings::get_colour_combinations(self.n, &self.p);
        let mut count = 0;
        for starting_row in starting_rows.iter() {
            let colouring = colourings::generate_colouring(starting_row.clone(), &self);
            if colourings::validate_colouring(&colouring) {
                count += 1;
            }
        }

        count
    }
}
