use std::fs;

const MAPWIDTH: u32 = 10;
const MAPHEIGHT: u32 = 5;


fn idx(x: i32, y: i32) -> Option<i32> {
    if x < 0 || x >= MAPWIDTH as i32 {
        return None
    }
    if y < 0 || y >= MAPHEIGHT as i32 {
        return None
    }

    Some((y * MAPWIDTH as i32) as i32 + x as i32)
}

fn neighbours(point: (i32, i32)) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = vec![-1, 1]
        .iter()
        .map(|pos| (point.0 + pos, point.1))
        .collect();

    points.append(
        &mut vec![-1, 1]
        .iter()
        .map(|pos| (point.0, point.1 + pos))
        .collect::<Vec<(i32, i32)>>()
    );

    return points;
}


fn main() {
    let map: Vec<i32> = fs::read_to_string("input/day9.txt")
        .expect("Something went wrong reading the file")
        .chars()
        .filter(|c| c.to_string() != "" && c.to_string() != "\n")
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();


    let mut sum = 0;
    for y in 0..MAPHEIGHT {
        for x in 0..MAPWIDTH {
            let positions = vec![-1, 1];
            let mut points: Vec<i32> = positions
                .iter()
                .filter(|&pos| idx(x as i32 + *pos, y as i32) != None)
                .map(|&pos| map[idx(x as i32 + pos, y as i32).unwrap() as usize])
                .collect();

            points.append(
                &mut positions
                .iter()
                .filter(|&pos| idx(x as i32, y as i32 + *pos) != None)
                .map(|&pos| map[idx(x as i32, y as i32 + pos).unwrap() as usize]) 
                .collect::<Vec<i32>>()
            );

            if let Some(i) = idx(x as i32, y as i32) {
                let min = *points.iter().min().unwrap();
                let act = map[i as usize];

                if act < min {
                    sum += 1 + act;
                }
            }
        }
    }


    for y in 0..MAPHEIGHT {
        for x in 0..MAPWIDTH {
            let positions = vec![-1, 1];
            let mut points: Vec<i32> = positions
                .iter()
                .filter(|&pos| idx(x as i32 + *pos, y as i32) != None)
                .map(|&pos| map[idx(x as i32 + pos, y as i32).unwrap() as usize])
                .collect();

            points.append(
                &mut positions
                .iter()
                .filter(|&pos| idx(x as i32, y as i32 + *pos) != None)
                .map(|&pos| map[idx(x as i32, y as i32 + pos).unwrap() as usize]) 
                .collect::<Vec<i32>>()
            );

            if let Some(mut i) = idx(x as i32, y as i32) {
                let min = *points.iter().min().unwrap();
                let act = map[i as usize];

                let mut xx = x;
                let mut yy = y;

                // basin
                if act < min {
                    let mut marks = vec![false; (MAPHEIGHT * MAPWIDTH) as usize];
                    let mut basins = Vec::new();
                    basins.push((x,y));
                    println!("current: {}", act);

                    for basin in &mut basins {
                        let a = basin.0 as i32;
                        let b = basin.1 as i32;

                        for neighbour in neighbours((a,b)) {
                            let aa = neighbour.0;
                            let bb = neighbour.1;
                            if let Some(index) = idx(aa as i32, bb as i32) {
                                let tile = map[index as usize];
                                if tile != 9 && !marks[index as usize] {
                                    basins.push((aa as u32, bb as u32));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("p1: {}", sum);
}
