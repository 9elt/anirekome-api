use super::init;

type Model = Vec<Vec<[i32; 9]>>;

pub struct AffinityModel<'a> {
    gte: Model,
    lte: Model,
    values: &'a Model,
    avgs: &'a Model,
    is_score_relevant: bool,
    accuracy: i32,
}

impl<'a> AffinityModel<'a> {
    pub fn new(values: &'a Model, avgs: &'a Model) -> Self {
        Self {
            gte: init::empty_affinity(),
            lte: init::empty_affinity(),
            values,
            avgs,
            is_score_relevant: false,
            accuracy: 10,
        }
    }

    pub fn to_array(&'a mut self) -> [Model; 2] {
        [self.gte.to_owned(), self.lte.to_owned()]
    }

    pub fn calc(&'a mut self, accuracy: i32) -> &mut Self {
        self.calc_relevance()
            .calc_accuracy(accuracy)
            .calc_general_stats()
            .calc_detailed_stats()
    }

    fn calc_accuracy(&'a mut self, accuracy: i32) -> &mut Self {
        let mut tot_dev = 0;
        let mut max_dev = 0;
        let mut c_0 = 0;
        for x in 0..self.gte.len(){
            for y in 0..self.gte[x].len() {
                for z in 0..self.gte[x][y].len() {
                    tot_dev += self.avgs[x][y][z].abs();
                    if self.avgs[x][y][z].abs() > max_dev {
                        max_dev = self.avgs[x][y][z].abs();
                    }
                    if self.avgs[x][y][z] == 0 {
                        c_0 += 1;
                    }
                }
            }
        }
        //tot_dev = tot_dev / c;
        println!("list length: {}", self.values[0][0][0]);
        println!("list length dev: {}", self.avgs[0][0][0]);
        println!("TOT avg deviation: {tot_dev}");
        tot_dev = tot_dev / 892;
        println!("TOT avg deviation: {tot_dev}");
        println!("MAX avg deviation: {max_dev}");
        println!("avg 0s: {c_0}");

        const EXP_DEV: i32 = 45;

        let len = match self.avgs[0][0][0] > 0 {
            true => self.avgs[0][0][0].abs() / -10,
            false => self.avgs[0][0][0].abs() / 10
        };
        let mut ov = tot_dev - EXP_DEV;
        if ov < 0 {
            ov = 0;
        }
        let adj = (ov + len) / (2 + ov / (EXP_DEV / 3));
        // needs iterpolation for adj and lev
        // adj sjo

        println!("lev: {}", len);
        println!("adj: {}", adj);

        self.accuracy = accuracy + adj + len;
        println!("accuracy: {}", self.accuracy);
        self
    }

    fn calc_relevance(&'a mut self) -> &mut Self {
        // scored percentage > 25%
        self.is_score_relevant = self.values[0][0][3] > 250;
        self
    }

    fn calc_general_stats(&'a mut self) -> &mut Self {
        // list size limits
        self.gte[0][0][0] = self.values[0][0][0] / 2;
        self.lte[0][0][0] = 300 + self.values[0][0][0] * 8;
        // average mal mean score +- 0.5
        self.gte[0][0][1] = self.values[0][0][1] - (5 * self.accuracy);
        self.lte[0][0][1] = self.values[0][0][1] + (5 * self.accuracy);
        if self.is_score_relevant {
            //  average score deviation +- 0.8
            self.gte[0][0][2] = self.values[0][0][2] - (8 * self.accuracy);
            self.lte[0][0][2] = self.values[0][0][2] + (8 * self.accuracy);
        }
        // completed += 35%
        self.gte[0][1][0] = self.values[0][1][0] - (35 * self.accuracy);
        self.lte[0][1][0] = self.values[0][1][0] + (35 * self.accuracy);
        // ptw += 35%
        self.gte[0][2][0] = self.values[0][2][0] - (35 * self.accuracy);
        self.lte[0][2][0] = self.values[0][2][0] + (35 * self.accuracy);
        // watching += 35%
        self.gte[0][3][0] = self.values[0][3][0] - (35 * self.accuracy);
        self.lte[0][3][0] = self.values[0][3][0] + (35 * self.accuracy);
        // onhold += 35%
        self.gte[0][4][0] = self.values[0][4][0] - (35 * self.accuracy);
        self.lte[0][4][0] = self.values[0][4][0] + (35 * self.accuracy);
        // dropped += 35%
        self.gte[0][5][0] = self.values[0][5][0] - (35 * self.accuracy);
        self.lte[0][5][0] = self.values[0][5][0] + (35 * self.accuracy);
        self
    }

    fn calc_detailed_stats(&'a mut self) -> &mut Self {
        let mut count: i32 = 1;
        let mut tot_accuracy: i32 = 0;
        for x in 1..self.gte.len() {
            let mut max_dev: i32 = 0;
            let mut max_val: i32 = 0;
            for y in 0..self.gte[x].len() {
                if self.avgs[x][y][0].abs() > max_dev.abs() {
                    max_dev = self.avgs[x][y][0].abs();
                }
                if self.values[x][y][0] > max_val {
                    max_val = self.values[x][y][0];
                }
            }
            for y in 0..self.gte[x].len() {
                let stat_accuracy = match Self::is_stat_relevant(
                    self.avgs[x][y][0],
                    max_dev,
                    self.values[x][y][0],
                    max_val,
                ) {
                    true => self.accuracy,
                    false => self.accuracy * 5,
                };

                let v = &self.values[x][y][0];

                self.gte[x][y][0] = v - (stat_accuracy + v);
                self.lte[x][y][0] = v + (stat_accuracy + v);

                count += 1;
                tot_accuracy += v + (stat_accuracy + v);
            }
        }
        tot_accuracy = tot_accuracy / count;

        println!("average accuracy: {tot_accuracy}");
        self
    }

    fn is_stat_relevant(avg_dev: i32, max_dev: i32, value: i32, max_val: i32) -> bool {
        (avg_dev > max_dev / 2) || (avg_dev < max_dev / -2) || value == 0 || value > (max_val * 2) / 3
    }

    fn _is_stat_score_relevant(
        score_dev: i32,
        tot_score_dev: i32,
        scored_pct: i32,
        tot_scored_pct: i32,
    ) -> bool {
        scored_pct > tot_scored_pct
            && (score_dev < (tot_score_dev - tot_score_dev.abs() / 2)
                || score_dev > (tot_score_dev + tot_score_dev.abs() / 2))
    }
}