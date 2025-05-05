#[derive(Clone, Copy, Debug)]
pub enum GraphStructure {
    Hypercube,
    Torus,
}

impl GraphStructure {
    fn distance(&self, a: &usize, b: &usize) -> usize {
        if *a >= 64 || *b >= 64 {
            panic!("Invalid page number");
        }
        match self {
            GraphStructure::Hypercube => (a ^ b).count_ones() as usize,
            GraphStructure::Torus => (0..(3 * 2))
                .step_by(2)
                .map(
                    |i| match ((a >> i) % 4).abs_diff((b >> i) % 4) {
                        0 => 0,
                        2 => 2,
                        1 | 3 => 1,
                        _ => unreachable!(),
                    },
                )
                .sum(),
        }
    }
}

impl std::fmt::Display for GraphStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphStructure::Hypercube => write!(f, "Hypercube"),
            GraphStructure::Torus => write!(f, "Torus"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MigrationType {
    MoveToMin,
    CoinFlip,
}

impl std::fmt::Display for MigrationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationType::MoveToMin => write!(f, "MoveToMin"),
            MigrationType::CoinFlip => write!(f, "CoinFlip"),
        }
    }
}

pub struct PageMigration {
    page: usize,
    moving_cost: usize,
    graph_structure: GraphStructure,
    migration_type: MigrationType,
    previous_requests: Option<Vec<usize>>,
}

impl PageMigration {
    pub fn new(
        page: usize,
        moving_cost: usize,
        graph_structure: GraphStructure,
        migration_type: MigrationType,
    ) -> Self {
        Self {
            page,
            moving_cost,
            graph_structure,
            previous_requests: match &migration_type {
                MigrationType::CoinFlip => None,
                MigrationType::MoveToMin => Some(Vec::with_capacity(moving_cost)),
            },
            migration_type,
        }
    }

    pub fn migrate_multiple(&mut self, targets: &[usize]) -> usize {
        targets
            .iter()
            .fold(0, |acc, &target| acc + self.migrate(target))
    }

    pub fn migrate(&mut self, target: usize) -> usize {
        match self.migration_type {
            MigrationType::MoveToMin => self.move_to_min_migrate(target),
            MigrationType::CoinFlip => self.random_flip_migrate(target),
        }
    }

    /// MOVE-TO-MIN - dzielimy σ na fazy długości D. W trakcie fazy nie przesuwamy strony. Po fazie zapytań v_1, . . . , v_D przenosimy zasób do wierzchołka m takiego, że minimalizuje SUM^D_(i=1) d(m, v_i).
    fn move_to_min_migrate(&mut self, target: usize) -> usize {
        let mut cost = self.graph_structure.distance(&self.page, &target);
        self.previous_requests.as_mut().unwrap().push(target);

        if self.previous_requests.as_ref().unwrap().len() == self.moving_cost {
            let min = (0..64)
                .min_by_key(|x| {
                    self.previous_requests
                        .as_ref()
                        .unwrap()
                        .iter()
                        .fold(0, |acc, y| acc + self.graph_structure.distance(x, y))
                })
                .expect("No minimum found");

            cost += self.moving_cost * self.graph_structure.distance(&self.page, &min);
            self.previous_requests.as_mut().unwrap().clear();
            self.page = min;
        }

        cost
    }

    /// Algorytm COINFLIP dla dowolnych grafów. Przy każdym żądaniu z prawdopodobieństwem 1 / D skopiuj zasób z najbliższego źródła.
    fn random_flip_migrate(&mut self, target: usize) -> usize {
        let mut cost = self.graph_structure.distance(&self.page, &target);
        if rand::random_bool(1. / (2 * self.moving_cost) as f64) {
            cost += self.moving_cost * self.graph_structure.distance(&self.page, &target);
            self.page = target;
        }

        cost
    }
}
