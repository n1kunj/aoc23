use nalgebra::Vector3;
use regex::Regex;
use z3::{ast::Ast, Config, Context};

#[derive(Debug)]
struct HailStone {
    p: Vector3<i64>,
    d: Vector3<i64>,
}

impl HailStone {
    fn at_t(&self, t: i64) -> Vector3<i64> {
        self.p + self.d * t
    }
}

pub fn main(input: &str) -> (String, String) {
    let re = Regex::new(r"(\d+),\s+(\d+),\s+(\d+)\s+@\s+(-?\d+),\s+(-?\d+),\s+(-?\d+)").unwrap();

    let mut hailstones = Vec::<HailStone>::new();
    for l in input.lines() {
        let c = re.captures(l).unwrap();
        let parse = |i: usize| -> i64 { c.get(i).unwrap().as_str().parse::<i64>().unwrap() };
        let h = HailStone {
            p: Vector3::<i64>::new(parse(1), parse(2), parse(3)),
            d: Vector3::<i64>::new(parse(4), parse(5), parse(6)),
        };
        hailstones.push(h);
    }

    let min_x = 200000000000000.0;
    let max_x = 400000000000000.0;
    let min_y = 200000000000000.0;
    let max_y = 400000000000000.0;

    let mut xy_col_in_range = 0usize;

    for (i, hs) in hailstones.iter().enumerate() {
        for hs2 in hailstones[i + 1..].iter() {
            let p1 = hs.at_t(0);
            let p2 = hs.at_t(1);
            let p3 = hs2.at_t(0);
            let p4 = hs2.at_t(1);

            let ua_num = (p4.x - p3.x) * (p1.y - p3.y) - (p4.y - p3.y) * (p1.x - p3.x);

            let ub_num = (p2.x - p1.x) * (p1.y - p3.y) - (p2.y - p1.y) * (p1.x - p3.x);
            let denom = (p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y);

            if denom == 0 {
                continue;
            }

            let ua = ua_num as f64 / denom as f64;
            let ub = ub_num as f64 / denom as f64;

            if ua < 0.0 {
                continue;
            }

            if ub < 0.0 {
                continue;
            }

            let x = p1.x as f64 + ua * (p2.x - p1.x) as f64;
            let y = p1.y as f64 + ua * (p2.y - p1.y) as f64;

            if min_x > x || max_x < x {
                continue;
            }

            if min_y > y || max_y < y {
                continue;
            }

            xy_col_in_range += 1;
        }
    }

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let px = z3::ast::Int::new_const(&ctx, "px");
    let py = z3::ast::Int::new_const(&ctx, "py");
    let pz = z3::ast::Int::new_const(&ctx, "pz");
    let dx = z3::ast::Int::new_const(&ctx, "dx");
    let dy = z3::ast::Int::new_const(&ctx, "dy");
    let dz = z3::ast::Int::new_const(&ctx, "dz");

    for hs in hailstones.iter() {
        let hpx = z3::ast::Int::from_i64(&ctx, hs.p.x);
        let hpy = z3::ast::Int::from_i64(&ctx, hs.p.y);
        let hpz = z3::ast::Int::from_i64(&ctx, hs.p.z);
        let hdx = z3::ast::Int::from_i64(&ctx, hs.d.x);
        let hdy = z3::ast::Int::from_i64(&ctx, hs.d.y);
        let hdz = z3::ast::Int::from_i64(&ctx, hs.d.z);
        let t = z3::ast::Int::fresh_const(&ctx, "t");

        solver.assert(&(&hpx + &t * &hdx)._eq(&(&px + &t * &dx)));
        solver.assert(&(&hpy + &t * &hdy)._eq(&(&py + &t * &dy)));
        solver.assert(&(&hpz + &t * &hdz)._eq(&(&pz + &t * &dz)));
    }

    let pos_sum = match solver.check() {
        z3::SatResult::Unsat => panic!(),
        z3::SatResult::Unknown => panic!(),
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let px = model.get_const_interp(&px).unwrap();
            let py = model.get_const_interp(&py).unwrap();
            let pz = model.get_const_interp(&pz).unwrap();
            px.as_i64().unwrap() + py.as_i64().unwrap() + pz.as_i64().unwrap()
        }
    };

    (format!("{xy_col_in_range}"), format!("{pos_sum}"))
}
