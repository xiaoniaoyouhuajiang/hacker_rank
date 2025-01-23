use std::collections::{hash_map::Entry, HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    YPositive,
    YNegative,
    XPositive,
    XNegative,
    LeftObliquePositive,
    LeftObliqueNegative,
    RightObliquePositive,
    RightObliqueNegative,
}

#[derive(Debug)]
struct Obstacle{
    x: i32,
    y: i32,
    direction: Direction,
    distance: i32,
}

pub fn queensAttack(n: i32, k: i32, r_q: i32, c_q: i32, obstacles: &[Vec<i32>]) -> i32 {
    // n: 棋盘大小
    // k: 障碍物数量
    // r_q: 皇后所在行
    // c_q: 皇后所在列
    // obstacles: 障碍物坐标
    let x_q = c_q;
    let y_q = r_q;
    let mut targets = get_original_obstacles(n, y_q, x_q);
    let mut attack_number = 0;
    for point in obstacles {
        let x_o = point[1];
        let y_o = point[0];
        let obs = get_obstacle_by_queen(x_q,  y_q, x_o, y_o);
        match obs {
            Some(obstacle) => {
                match targets.entry(obstacle.direction.clone()) {
                    Entry::Vacant(entry) => {
                        entry.insert(obstacle);
                    },
                    Entry::Occupied(mut entry) => {
                        let ori_obstacle = entry.get_mut();
                        if ori_obstacle.distance > obstacle.distance {
                            *ori_obstacle = obstacle;
                        }
                    }
                }
            },
            None => {
                continue;
            }
        }
    }
    for (_, obstacle) in targets.iter() {
        attack_number += obstacle.distance;
    }
    attack_number
}

fn get_obstacle_by_queen(x_q: i32, y_q: i32, x_o: i32, y_o: i32) -> Option<Obstacle> {
    // r1, c1: 点1坐标
    // r2, c2: 点2坐标
    if x_q == x_o {
        if y_o-y_q > 0 {
            return Some(Obstacle {x: x_q, y: y_o, direction: Direction::YPositive, distance: y_o-y_q-1});
        } else {
            return Some(Obstacle {x: x_q, y: y_o, direction: Direction::YNegative, distance: y_q-y_o-1});
        }
    } else if y_q == y_o {
        if x_o-x_q > 0 {
            return Some(Obstacle {x: x_o, y: y_q, direction: Direction::XPositive, distance: x_o-x_q-1});
        } else {
            return Some(Obstacle {x: x_o, y: y_q, direction: Direction::XNegative, distance: x_q-x_o-1});
        }
    } else {
        let fraction = (y_o - y_q) as f32 / (x_o - x_q) as f32;
        if fraction == 1.0{
            // 斜率为1
            if x_o-x_q > 0 {
                return Some(Obstacle {x: x_o, y: y_o, direction: Direction::RightObliquePositive, distance: x_o-x_q-1});
            } else {
                return Some(Obstacle {x: x_o, y: y_o, direction: Direction::RightObliqueNegative, distance: x_q-x_o-1});
            }
        } else if fraction == -1.0{
            // 斜率为-1
            if x_o-x_q > 0 {
                return Some(Obstacle {x: x_o, y: y_o, direction: Direction::LeftObliqueNegative, distance: x_o-x_q-1});
            } else {
                return Some(Obstacle {x: x_o, y: y_o, direction: Direction::LeftObliquePositive, distance: x_q-x_o-1});
            }
        } else {
            return None;
        }
    }
}

fn get_original_obstacles(n: i32, y_q: i32, x_q: i32) -> HashMap<Direction, Obstacle> {
    let mut obstacles = vec![
        Obstacle {x: x_q, y: 0, direction: Direction::YNegative, distance: y_q-1},
        Obstacle {x: x_q, y: n+1, direction: Direction::YPositive, distance: n-y_q},
        Obstacle {x: 0, y: y_q, direction: Direction::XNegative, distance: x_q-1},
        Obstacle {x: n+1, y: y_q, direction: Direction::XPositive, distance: n-x_q}
        // (y, 0), (y, n+1),
        // (0, x), (n+1, x),
    ];
    let mut points = vec!();
    for v in vec!(0, n+1) {
        points.push((v, v-x_q+y_q));
        points.push((v, x_q-v+y_q));
        points.push((v+x_q-y_q, v));
        points.push((x_q+y_q-v, v));
    }
    let unique: Vec<(i32, i32)> = points.into_iter().collect::<HashSet<_>>().into_iter().collect();
    let points: Vec<(i32, i32)> = unique.into_iter().filter(|&(x, y)|x>=0 && x<=n+1 && y>=0 && y<=n+1).collect();
    for (x_o, y_o) in points {
        match get_obstacle_by_queen(x_q, y_q, x_o, y_o) {
            Some(obs) => obstacles.push(obs),
            None => continue,
        };
    }
    let mut table = HashMap::new();
    for obs in obstacles {
        table.insert(obs.direction.clone(), obs);
    }
    table
}

#[test]
fn test_get_original_obstacles() {
    println!("{:?}", get_original_obstacles(8, 4, 4));
}

#[test]
fn test_get_obstacle_by_queen() {
    println!("{:?}", get_obstacle_by_queen(4, 4, 4, 0));
}

#[test]
fn test_queensAttack() {
    println!("{:?}", queensAttack(8, 0, 4, 4, &[]))
}
