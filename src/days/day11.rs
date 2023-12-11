#[derive(Clone, Debug)]
enum Point {
    Empty,
    Galaxy,
}

#[derive(Clone, Debug)]
struct ImageRow {
    row: Vec<Point>,
}

#[derive(Debug)]
struct Image {
    rows: Vec<ImageRow>,
}

pub fn main(input: &str) -> (String, String) {
    let mut rows = Vec::<ImageRow>::new();
    for l in input.lines() {
        let mut row = Vec::<Point>::new();
        for c in l.chars() {
            let p = match c {
                '.' => Point::Empty,
                '#' => Point::Galaxy,
                other => panic!("{other}"),
            };
            row.push(p);
        }
        rows.push(ImageRow { row });
    }
    let undilated = Image { rows };

    let mut is_row_dilated = Vec::<bool>::new();
    let mut is_col_dilated = Vec::<bool>::new();

    for row in undilated.rows.iter() {
        is_row_dilated.push(row.row.iter().all(|p| matches!(p, Point::Empty)));
    }

    for x in 0..undilated.rows[0].row.len() {
        is_col_dilated.push(
            undilated
                .rows
                .iter()
                .all(|r| matches!(r.row[x], Point::Empty)),
        );
    }

    let mut gal_coord = Vec::<(usize, usize)>::new();

    for (y, r) in undilated.rows.iter().enumerate() {
        for (x, p) in r.row.iter().enumerate() {
            if matches!(p, Point::Galaxy) {
                gal_coord.push((x, y));
            }
        }
    }

    let calc_shortest_dist_sum = |expansion_factor: usize| -> usize {
        let mut shortest_dist_sum = 0usize;
        for i in 0..gal_coord.len() {
            for j in i + 1..gal_coord.len() {
                let this = gal_coord[i];
                let that = gal_coord[j];

                let mut shortest_dist = 0usize;

                let mut xr = [this.0, that.0];
                xr.sort();

                for x in xr[0]..xr[1] {
                    match is_col_dilated[x] {
                        true => shortest_dist += expansion_factor,
                        false => shortest_dist += 1,
                    }
                }

                let mut yr = [this.1, that.1];
                yr.sort();

                for y in yr[0]..yr[1] {
                    match is_row_dilated[y] {
                        true => shortest_dist += expansion_factor,
                        false => shortest_dist += 1,
                    }
                }

                shortest_dist_sum += shortest_dist;
            }
        }
        shortest_dist_sum
    };

    let expansion_2_sum = calc_shortest_dist_sum(2);
    let expansion_mil_sum = calc_shortest_dist_sum(1000000);

    (format!("{expansion_2_sum}"), format!("{expansion_mil_sum}"))
}
