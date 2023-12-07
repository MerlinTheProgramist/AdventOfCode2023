use std::fs::read_to_string;

struct Hand {
    hand: Vec<u8>,
    bid: i32,
    order: u8,
}

const cards: &str = "AKQJT98765432";

fn main() {
    let mut hands = Vec::new();
    for line in read_to_string("p1.in").unwrap().lines() {
        let (hand, bid) = line.split_once(' ').unwrap();

        let bid = bid.parse::<i32>().unwrap();
        let mut points = hand.as_bytes().iter()
            .map(|c| cards.as_bytes().iter().position(|&r| r == *c).unwrap() as u8).collect::<Vec<_>>();
        hands.push(Hand{hand: points.clone(), bid: bid, order: eval_hand(&mut points)});
    }

    hands.sort_by(|x,y| y.hand.cmp(&x.hand));
    hands.sort_by(|x,y| y.order.cmp(&x.order));
    
    let mut ans = 0;
    for (i,h) in hands.iter().enumerate(){
        ans += (i+1)*h.bid as usize;
        println!("{}, ord: {}", std::str::from_utf8(&h.hand.iter().map(|x| cards.as_bytes()[*x as usize]).collect::<Vec<_>>()).unwrap(), h.order);
    }
    
    println!("{}", ans);
}

struct Pile{
    card:u8,
    count:i8,
}
fn eval_hand(hand: &mut Vec<u8>) ->u8{
    hand.sort();
    let mut counts = Vec::with_capacity(5);
    counts.push(Pile{card:hand[0], count:1});
    for p in hand.iter().skip(1) {
        if counts.last().unwrap().card==*p{
            counts.last_mut().unwrap().count += 1;
        }else{
            counts.push(Pile{card:*p, count:1});
        }
    }
    // five of kind
    if counts.len() == 1 {
        return 0;
    }
    // four of kind
    if counts.len() == 2 && (counts[0].count-counts[1].count).abs() == 4{
        return 1;
    }
    // full house
    if counts.len() == 2{
        return 2;
    }
    // three of a kind
    if counts.len() == 3 && (counts[0].count == 3 || counts[1].count == 3 || counts[2].count == 3){
        return 3;
    } 
    // two pair
    if counts.len() == 3{
        return 4;
    }
    // one pair
    if counts.len() == 4{
        return 5;
    }

    // high card
    return 6;
}
