use std::time::SystemTime;

pub struct Snowflake {
    epoch: SystemTime,
    last_gen_time: i64,
    pub machine_id: i32,
    pub node_id: i32,
    seq: u16,
}

impl Snowflake {
    pub fn generate(&mut self) -> i64 {
        self.seq = (self.seq + 1) % 4096;

        let mut now_millis = get_time_millis(self.epoch);

        if now_millis == self.last_time_millis {
            if self.idx == 0 {
                now_millis = biding_time_conditions(self.last_time_millis, self.epoch);
                self.last_time_millis = now_millis;
            }
        } else {
            self.last_time_millis = now_millis;
            self.idx = 0;
        }

        self.last_gen_time << 22
            | ((self.machine_id << 17) as i64)
            | ((self.node_id << 12) as i64)
            | (self.seq as i64)
    }
}

pub fn get_time_millis(epoch: SystemTime) -> i64 {
    SystemTime::now()
        .duration_since(epoch)
        .expect("Time went mackward")
        .as_millis() as i64
}