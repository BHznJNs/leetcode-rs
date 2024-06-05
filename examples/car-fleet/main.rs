struct Solution;

#[derive(Debug)]
struct Car {
    pos: i32,
    time_remain: f64,
}
impl Car {
    pub fn new(target: i32, pos: i32, speed: i32) -> Self {
        let time_remain = (target - pos) as f64 / speed as f64;
        return Self { pos, time_remain };
    }
}

struct CarStack {
    list: Vec<Car>,
}
impl CarStack {
    pub fn from(car_list: Vec<Car>) -> Self {
        Self { list: car_list }
    }
    pub fn push(&mut self, val: Car) {
        self.list.push(val)
    }
    pub fn pop(&mut self) -> Option<Car> {
        self.list.pop()
    }
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        if position.len() == 1 {
            return 1
        }

        let mut car_list = position
            .into_iter()
            .zip(speed.into_iter())
            .map(|(pos, speed)| Car::new(target, pos, speed))
            .collect::<Vec<Car>>();
        car_list.sort_by_key(|car| car.pos);

        let mut car_stack = CarStack::from(car_list);
        let mut fleet_count = 0;
        while !car_stack.is_empty() {
            let popped1 = car_stack.pop().unwrap();
            let Some(popped2) = car_stack.pop() else {
                fleet_count += 1;
                break;
            };

            if popped1.time_remain >= popped2.time_remain {
                // popped2 is faster than popped1 and they will meet each other.
                car_stack.push(popped1);
            } else {
                // popped2 is slower than popped1 and they will not meet each other.
                fleet_count += 1;
                car_stack.push(popped2);
            }
        }
        
        return fleet_count;
    }
}

fn main() {
    println!(
        "{}",
        Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3])
    );

    println!(
        "{}",
        Solution::car_fleet(10, vec![3], vec![3])
    );

    println!(
        "{}",
        Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1])
    );
}
