use std::ops::RangeInclusive;

struct Data {
    #[allow(clippy::type_complexity)]
    fields: Vec<(String, (RangeInclusive<u32>, RangeInclusive<u32>))>,

    my_tickets: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

impl Data {
    fn new(path: &str) -> Self {
        use lazy_static::lazy_static;
        use regex::Regex;

        lazy_static! {
            static ref FIELD: Regex = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
            static ref TICKET: Regex = Regex::new(r"(\d+,)+\d+").unwrap();
        }

        let lines = utils::read_strings(path);
        let mut fields = vec![];

        let mut my_tickets = None;
        let mut nearby_tickets = vec![];

        for l in &lines {
            if let Some(caps) = FIELD.captures(l) {
                let name = caps.get(1).unwrap().as_str().to_string();
                let l1 = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let r1 = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
                let l2 = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
                let r2 = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();

                fields.push((name, (l1..=r1, l2..=r2)));
            } else if TICKET.captures(l).is_some() {
                let t: Vec<u32> = l.split(',').map(|s| s.parse::<u32>().unwrap()).collect();

                if my_tickets.is_none() {
                    my_tickets = Some(t);
                } else {
                    nearby_tickets.push(t);
                }
            }
        }

        Data {
            fields,
            my_tickets: my_tickets.unwrap(),
            nearby_tickets,
        }
    }

    fn part2(&self) -> u64 {
        // candidates[`ticket_id`][`field`]
        let mut candidates = vec![];
        for _ in 0..self.my_tickets.len() {
            candidates.push(vec![true; self.fields.len()]);
        }

        for t in &self.nearby_tickets {
            let mut cont = false;
            for x in t {
                let mut flg = false;
                for (_, (r1, r2)) in &self.fields {
                    flg |= r1.contains(&x);
                    flg |= r2.contains(&x);
                }
                if !flg {
                    cont = true;
                    break;
                }
            }
            if cont {
                continue;
            }

            for (t_idx, v) in t.iter().enumerate() {
                for (f_idx, (_, (r1, r2))) in self.fields.iter().enumerate() {
                    if !(r1.contains(&v) || r2.contains(&v)) {
                        candidates[t_idx][f_idx] = false;
                    }
                }
            }
        }

        for _ in 0..candidates.len() {
            for i in 0..candidates.len() {
                if candidates[i].iter().filter(|&b| *b).count() == 1 {
                    let pos = candidates[i].iter().position(|&b| b).unwrap();

                    for (j, c) in candidates.iter_mut().enumerate() {
                        if i != j {
                            c[pos] = false;
                        }
                    }
                }
            }
        }

        let mut ans = 1u64;
        for i in 0..6 {
            for (idx, c) in candidates.iter().enumerate() {
                if c[i] {
                    ans *= self.my_tickets[idx as usize] as u64;
                }
            }
        }

        ans
    }
}

fn main() {
    let data = Data::new("./day16.txt");

    let mut ans = 0;
    for t in &data.nearby_tickets {
        for x in t {
            let mut flg = false;
            for (_, (r1, r2)) in &data.fields {
                flg |= r1.contains(&x);
                flg |= r2.contains(&x);
            }
            if !flg {
                ans += x;
            }
        }
    }

    println!("part 1: {}", ans);
    println!("Part 2: {}", data.part2());
}
