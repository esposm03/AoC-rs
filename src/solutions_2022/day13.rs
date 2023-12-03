use std::cmp::Ordering;

use nom::{bytes::complete as bytes, combinator::map, multi, IResult};

use crate::Solution;

pub fn day13(input: &str) -> Solution {
    let input = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, packet_pair) in input.split(|line| line.is_empty()).enumerate() {
        let packet_a = packet_pair[0];
        let packet_b = packet_pair[1];

        let (_, a) = parse_packet(packet_a).unwrap();
        let (_, b) = parse_packet(packet_b).unwrap();

        if cmp(&a, &b) == Ordering::Less {
            sum += i + 1;
        }
    }

    sum.into()
}

pub fn day13_part2(input: &str) -> Solution {
    let div1 = parse_packet("[[2]]").unwrap().1;
    let div2 = parse_packet("[[6]]").unwrap().1;
    let mut packets = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|p| parse_packet(p).unwrap().1)
        .chain([div1.clone(), div2.clone()])
        .collect::<Vec<_>>();

    packets.sort_unstable();

    let pos1 = packets.iter().position(|packet| *packet == div1).unwrap() + 1;
    let pos2 = packets.iter().position(|packet| *packet == div2).unwrap() + 1;

    (pos1 * pos2).into()
}

fn parse_packet(i: &str) -> IResult<&str, El> {
    use nom::Parser;
    parse_elem.or(parse_list).parse(i)
}

fn parse_elem(i: &str) -> IResult<&str, El> {
    let (i, n): (&str, u32) = map(bytes::take_while1(char::is_numeric), |i: &str| {
        i.parse().unwrap()
    })(i)?;

    Ok((i, El::Elem(n)))
}

fn parse_list(i: &str) -> IResult<&str, El> {
    let (a, _) = bytes::tag("[")(i)?;
    let (b, l) = multi::separated_list0(bytes::tag(","), parse_packet)(a)?;
    let (c, _) = bytes::tag("]")(b)?;

    Ok((c, El::List(l)))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum El {
    List(Vec<El>),
    Elem(u32),
}
impl PartialOrd for El {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for El {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp(self, other)
    }
}
fn cmp(a: &El, b: &El) -> Ordering {
    let ord = match (a, b) {
        (El::Elem(a), El::Elem(b)) => a.cmp(b),
        (El::List(a), El::List(b)) => a
            .iter()
            .zip(b.iter())
            .map(|(a, b)| cmp(a, b))
            .find(|ord| *ord != Ordering::Equal)
            .unwrap_or(Ordering::Equal),
        (a @ El::Elem(_), b @ El::List(_)) => cmp(&El::List(vec![a.clone()]), b),
        (a @ El::List(_), b @ El::Elem(_)) => cmp(a, &El::List(vec![b.clone()])),
    };

    if ord == Ordering::Equal {
        if let El::List(a) = a {
            if let El::List(b) = b {
                return a.len().cmp(&b.len());
            }
        }
    }
    ord
}

#[test]
#[cfg(test)]
fn test() {
    assert_eq!(parse_packet("123ab"), Ok(("ab", El::Elem(123))));
    assert_eq!(parse_list("[1]"), Ok(("", El::List(vec![El::Elem(1)]))));
    assert_eq!(
        parse_list("[1,2]"),
        Ok(("", El::List(vec![El::Elem(1), El::Elem(2)])))
    );

    let input = "[1,1,3,1,1]
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

    assert_eq!(day13(input), Solution::Int(13));
    assert_eq!(day13_part2(input), Solution::Int(140));
}
