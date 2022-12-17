use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{all_consuming, map, map_res};
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{Finish, IResult};
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Packet {
    Value(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, &other) {
            (&Packet::Value(a), &Packet::Value(b)) => return a.cmp(&b),
            (&Packet::Value(a), &Packet::List(_)) => {
                return Packet::List(vec![Packet::Value(*a)]).cmp(other)
            }
            (&Packet::List(_), &Packet::Value(b)) => {
                return self.cmp(&Packet::List(vec![Packet::Value(*b)]))
            }
            (&Packet::List(a), &Packet::List(b)) => {
                let mut i = 0;
                let mut ord = Ordering::Equal;
                while i < a.len() && i < b.len() && ord == Ordering::Equal {
                    ord = a[i].cmp(&b[i]);
                    i += 1;
                }
                if ord != Ordering::Equal || i == a.len() && i == b.len() {
                    return ord;
                }
                if i == a.len() {
                    return Ordering::Less;
                }
                if i == b.len() {
                    return Ordering::Greater;
                }
                unreachable!()
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn parse_number(i: &str) -> IResult<&str, Packet> {
    map_res(digit1, |s: &str| s.parse::<u8>().map(|i| Packet::Value(i)))(i)
}

fn parse_list(i: &str) -> IResult<&str, Packet> {
    delimited(
        tag("["),
        map(
            separated_list0(tag(","), alt((parse_number, parse_list))),
            |s| Packet::List(s),
        ),
        tag("]"),
    )(i)
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|line| line.len() != 0)
        .map(|line| all_consuming(parse_list)(line).finish().unwrap().1)
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(packets: &Vec<Packet>) -> usize {
    let mut total = 0;
    for i in 0..packets.len() / 2 - 1 {
        if packets[2 * i] < packets[2 * i + 1] {
            total += i + 1;
        }
    }
    total
}

#[aoc(day13, part2)]
pub fn solve_part2(packets: &Vec<Packet>) -> usize {
    let added_packets = vec![
        Packet::List(vec![Packet::List(vec![Packet::Value(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Value(6)])]),
    ];
    let mut packets = packets.clone();
    packets.append(&mut added_packets.clone());
    packets.sort();
    packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| added_packets.contains(packet))
        .map(|el| el.0 + 1)
        .product()
}

#[cfg(test)]
mod test {
    use crate::day13::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_p1() {
        let pairs = input_generator(INPUT);
        assert_eq!(solve_part1(&pairs), 13)
    }

    #[test]
    fn test1() {
        const INPUT: &str = "[[],[[[7,6,1,0],6,7,7]],[6],[[],6]]
[[],[],[[],[[]],[[8,5],9,[2],8,5],4]]";

        let pair = input_generator(INPUT);
        assert_eq!(pair[0].cmp(&pair[1]), Ordering::Greater)
    }

    mod part1 {
        use crate::day13::*;

        #[test]
        fn test1() {
            const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]";

            let pair = input_generator(INPUT);
            assert_eq!(pair[0].cmp(&pair[1]), Ordering::Less)
        }

        #[test]
        fn test2() {
            const INPUT: &str = "[9]
[[8,7,6]]";
            let pair = input_generator(INPUT);
            assert_eq!(pair[0].cmp(&pair[1]), Ordering::Greater)
        }

        #[test]
        fn test3() {
            const INPUT: &str = "[[4,4],4,4]
[[4,4],4,4,4]";
            let pair = input_generator(INPUT);
            assert_eq!(pair[0].cmp(&pair[1]), Ordering::Less)
        }

        #[test]
        fn test4() {
            const INPUT: &str = "[]
[3]";
            let pair = input_generator(INPUT);
            assert_eq!(pair[0].cmp(&pair[1]), Ordering::Less)
        }

        #[test]
        fn test5() {
            const INPUT: &str = "[[[]]]
[[]]";
            let pair = input_generator(INPUT);
            assert_eq!(pair[0].cmp(&pair[1]), Ordering::Greater)
        }

        #[test]
        fn test6() {
            const INPUT: &str = "[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
            let pair = input_generator(INPUT);
            assert_eq!(pair[0].cmp(&pair[1]), Ordering::Greater)
        }

        #[test]
        fn test7() {
            const INPUT: &str = "[[1],[2,3,4]]
[[1],4]";
            let pair = input_generator(INPUT);
            assert_eq!(pair[0].cmp(&pair[1]), Ordering::Less)
        }
    }
}
