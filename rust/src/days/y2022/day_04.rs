#[derive(PartialEq, Eq, Debug)]
struct Job {
    start: u32,
    end: u32,
}

impl Job {
    fn includes(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start <= other.start && self.end >= other.start)
            || (self.start <= other.end && self.end >= other.end)
    }
}

impl TryFrom<&str> for Job {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let splitted = value.split_once('-');
        match splitted {
            Some(splitted) => {
                let start = splitted.0.parse();
                let end = splitted.1.parse();
                if start.is_err() || end.is_err() {
                    return Err(());
                }
                let start = start.unwrap();
                let end = end.unwrap();
                Ok(Self { start, end })
            }
            _ => Err(()),
        }
    }
}

fn parse_pair(input: &str) -> (Job, Job) {
    let pair = input.split_once(',').unwrap();
    let first = Job::try_from(pair.0).unwrap();
    let second = Job::try_from(pair.1).unwrap();

    (first, second)
}

pub fn puzzle_1(input: &str) -> String {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(parse_pair)
        .map(|(pa, pb)| pa.includes(&pb) || pb.includes(&pa))
        .filter(|x| *x)
        .count()
        .to_string()
}

pub fn puzzle_2(input: &str) -> String {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(parse_pair)
        .map(|(pa, pb)| pa.overlaps(&pb) || pb.overlaps(&pa))
        .filter(|x| *x)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    use super::*;

    #[test]
    fn test_parse_jobs() {
        let job = "2-4";
        assert_eq!(Job::try_from(job).unwrap(), Job { start: 2, end: 4 });
    }

    #[test]
    fn test_parse_pair() {
        let pair = "2-4,6-8";
        assert_eq!(
            parse_pair(pair),
            (Job { start: 2, end: 4 }, Job { start: 6, end: 8 })
        );
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(INPUT), "2");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(INPUT), "4");
    }
}
