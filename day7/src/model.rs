use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Requirement {
    must_be_finished: char,
    can_begin: char,
}

pub struct Steps {
    requirements: Vec<Requirement>,
    todo: HashSet<char>,
    done: HashSet<char>,
    size: usize,
}

impl Steps {
    pub fn new(reqs: Vec<Requirement>) -> Steps {
        let mut todo = HashSet::new();

        for req in &reqs {
            todo.insert(req.must_be_finished);
            todo.insert(req.can_begin);
        }

        let size = todo.len();
        Steps {
            requirements: reqs,
            todo,
            done: HashSet::new(),
            size,
        }
    }

    fn is_complete(&self) -> bool {
        self.todo.len() == 0 && self.done.len() == self.size
    }

    fn is_done(&self, step: char) -> bool {
        self.done.contains(&step)
    }

    fn is_doable(&self, step: char) -> bool {
        self.requirements
            .iter()
            .filter(|req| req.can_begin == step)
            .all(|req| self.is_done(req.must_be_finished))
    }

    fn get_doable(&self) -> Vec<char> {
        self.todo
            .iter()
            .filter(|step| self.is_doable(**step))
            .cloned()
            .collect()
    }

    fn do_it(&mut self, step: char) {
        self.begin(step);
        self.finish(step);
    }

    fn begin(&mut self, step: char) {
        self.todo.remove(&step);
    }

    fn finish(&mut self, step: char) {
        self.done.insert(step);
    }
}

impl Iterator for Steps {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(step) = self.get_doable().iter().min() {
            self.do_it(*step);

            Some(*step)
        } else {
            None
        }
    }
}

pub struct Instructions {
    requirements: Vec<Requirement>,
}

impl Instructions {
    pub fn new(reqs: Vec<Requirement>) -> Instructions {
        Instructions { requirements: reqs }
    }

    pub fn steps(&self) -> Steps {
        Steps::new(self.requirements.clone())
    }
}

impl Requirement {
    pub fn new(mbf: char, cb: char) -> Requirement {
        Requirement {
            must_be_finished: mbf,
            can_begin: cb,
        }
    }
}

fn parse_step_letter(text: &str) -> Option<char> {
    text.chars().next()
}

named!(parse_requirement<&str, Requirement>,
    do_parse!(
        tag!("Step ") >>
        mbf :  map_opt!(take_until_and_consume!(" "), parse_step_letter) >>
        tag!("must be finished before step ") >>
        c_b :  map_opt!(take_until_and_consume!(" "), parse_step_letter) >>
        tag!("can begin.") >>
        (Requirement::new(mbf, c_b))
    )
);

impl FromStr for Requirement {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match parse_requirement(s) {
            Ok((_, req)) => Ok(req),
            Err(_) => Err(()),
        }
    }
}

pub fn get_step_duration(step: char) -> usize {
    let step_index = step as usize;
    let first_index = 'A' as usize;

    1 + (step_index - first_index)
}

#[derive(Debug, Clone)]
struct Worker {
    step: Option<char>,
    remaining: usize,
}

impl Worker {
    fn new() -> Worker {
        Worker {
            step: None,
            remaining: 0,
        }
    }

    fn is_idle(&self) -> bool {
        self.step.is_none()
    }

    fn assign(&mut self, step: char, time: usize) {
        self.step = Some(step);
        self.remaining = time;
    }

    fn work(&mut self) -> Option<char> {
        if self.remaining == 0 {
            None
        } else if self.remaining == 1 {
            self.remaining = 0;

            self.step.take()
        } else {
            self.remaining -= 1;

            None
        }
    }
}

pub struct Team {
    workers: Vec<Worker>,
    delay: usize,
}

impl Team {
    pub fn new(worker_count: usize, with_delay: bool) -> Team {
        Team {
            workers: vec![Worker::new(); worker_count],
            delay: if with_delay { 60 } else { 0 },
        }
    }

    fn work(&mut self, steps: &mut Steps) {
        for worker in self.workers.iter_mut() {
            if let Some(done) = worker.work() {
                steps.finish(done);
            }
        }
    }

    fn assign_work(&mut self, steps: &mut Steps) {
        let mut doable = steps.get_doable();

        doable.sort();

        for step in doable {
            let mut idle_workers = self.workers.iter_mut().filter(|w| w.is_idle());

            if let Some(worker) = idle_workers.next() {
                let time = self.delay + get_step_duration(step);

                worker.assign(step, time);
                steps.begin(step);
            } else {
                break;
            }
        }
    }

    pub fn complete_steps(&mut self, mut steps: Steps) -> usize {
        let mut clock = 0;

        loop {
            self.work(&mut steps);

            if steps.is_complete() {
                return clock;
            }

            self.assign_work(&mut steps);

            clock += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_req() {
        let parsed = parse_requirement("Step F must be finished before step E can begin.");
        let requirement = Requirement::new('F', 'E');
        let expected = Ok(("", requirement));

        assert_eq!(expected, parsed);
    }

    #[test]
    fn is_doable_finds_root_step() {
        let req = Requirement::new('A', 'B');
        let reqs = vec![req];
        let steps = Steps::new(reqs);

        assert_eq!(true, steps.is_doable('A'));
        assert_eq!(false, steps.is_doable('B'));
    }

    #[test]
    fn is_doable_finds_step_with_finished_pred() {
        let req = Requirement::new('A', 'B');
        let reqs = vec![req];
        let mut steps = Steps::new(reqs);

        steps.do_it('A');

        assert_eq!(true, steps.is_doable('B'));
    }

    #[test]
    fn get_doable_returns_the_unique_step() {
        let req = Requirement::new('A', 'B');
        let reqs = vec![req];
        let steps = Steps::new(reqs);
        let doable = steps.get_doable();

        assert_eq!(1, doable.len());
        assert_eq!('A', doable[0]);
    }

    #[test]
    fn get_doable_returns_nothing_when_root_is_in_progress() {
        let req = Requirement::new('A', 'B');
        let reqs = vec![req];
        let mut steps = Steps::new(reqs);

        steps.begin('A');
        assert_eq!(0, steps.get_doable().len());

        steps.finish('A');
        assert_eq!(1, steps.get_doable().len());
    }

    #[test]
    fn get_step_duration_matches() {
        assert_eq!(1, get_step_duration('A'));
        assert_eq!(2, get_step_duration('B'));
        assert_eq!(3, get_step_duration('C'));
        assert_eq!(26, get_step_duration('Z'));
    }

    #[test]
    fn team_complete_simplest_instructions() {
        let reqs = vec![Requirement::new('A', 'B')];
        let steps = Steps::new(reqs);
        let mut team = Team::new(1, false);
        assert_eq!(3, team.complete_steps(steps));
    }

    #[test]
    fn team_complete_matches() {
        let reqs = vec![
            Requirement::new('C', 'A'),
            Requirement::new('C', 'F'),
            Requirement::new('A', 'B'),
            Requirement::new('A', 'D'),
            Requirement::new('B', 'E'),
            Requirement::new('D', 'E'),
            Requirement::new('F', 'E'),
        ];
        let steps = Steps::new(reqs);
        let mut team = Team::new(2, false);
        assert_eq!(15, team.complete_steps(steps));
    }
}
