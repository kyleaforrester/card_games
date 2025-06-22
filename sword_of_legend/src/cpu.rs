use crate::sword::Sword;
use crate::card::Card;
use crate::card::Suit;
use crate::target::Target;

pub fn draft_card_cpu(sword: &Sword, sword_cards: &mut Vec::<Card>, target_deck: &Vec<Card>) -> Card {
    let mut card_cands: Vec<(usize, &Card)> = Vec::new();
    match sword_cards.iter().enumerate().filter(|c| c.1.suit == Suit::Sharpness).max_by_key(|c| c.1.value) {
        Some(tup) => card_cands.push(tup),
        None => (),
    };
    match sword_cards.iter().enumerate().filter(|c| c.1.suit == Suit::Balance).max_by_key(|c| c.1.value) {
        Some(tup) => card_cands.push(tup),
        None => (),
    };
    match sword_cards.iter().enumerate().filter(|c| c.1.suit == Suit::Durability).max_by_key(|c| c.1.value) {
        Some(tup) => card_cands.push(tup),
        None => (),
    };
    match sword_cards.iter().enumerate().filter(|c| c.1.suit == Suit::Honor).max_by_key(|c| c.1.value) {
        Some(tup) => card_cands.push(tup),
        None => (),
    };

    let card_tup: &(usize, &Card) = card_cands.iter().max_by(|a, b| evaluate_draft_card(a.1, sword, target_deck).partial_cmp(&evaluate_draft_card(b.1, sword, target_deck)).unwrap()).unwrap();

    sword_cards.swap_remove(card_tup.0)
}

fn evaluate_draft_card(card: &Card, sword: &Sword, target_deck: &Vec<Card>) -> f64 {
    let current_fail_rate = fail_rate(sword, target_deck).0;

    let mut new_sword = sword.clone();
    new_sword.cards.push(card.clone());
    let new_fail_rate = fail_rate(&new_sword, target_deck).0;

    current_fail_rate - new_fail_rate
}

pub fn swing_decision_cpu(sword: &Sword, target: &Target, swing_decisions: &Vec<(usize, bool, f64, f64)>, target_deck: &Vec<Card>, discard_pile: &Vec<Card>, player_count: usize) -> (f64, f64) {
    let mut full_targets: Vec<Target> = Vec::new();
    let mut sharpness_successes: u32 = 0;
    let mut balance_fails: u32 = 0;
    let mut durability_fails: u32 = 0;
    let mut honor_fails: u32 = 0;

    match target_deck.len() {
        0 => {
            for i_tup in discard_pile.iter().enumerate() {
                for j in discard_pile.iter().skip(i_tup.0 + 1) {
                    let mut full_target = target.clone();
                    let i = i_tup.1;
                    full_target.add_card(i.clone());
                    full_target.add_card(j.clone());
                    full_targets.push(full_target);
                }
            }
        },
        1 => {
            for i in discard_pile.iter() {
                let mut full_target = target.clone();
                full_target.add_card(target_deck[0].clone());
                full_target.add_card(i.clone());
                full_targets.push(full_target);
            }
        },
        _ => {
            for i_tup in target_deck.iter().enumerate() {
                for j in target_deck.iter().skip(i_tup.0 + 1) {
                    let mut full_target = target.clone();
                    let i = i_tup.1;
                    full_target.add_card(i.clone());
                    full_target.add_card(j.clone());
                    full_targets.push(full_target);
                }
            }
        },
    }

    let mut sword_sharpness = 0;
    let mut sword_balance = 0;
    let mut sword_durability = 0;
    let mut sword_honor = 0;
    for c in sword.cards.iter() {
        match c.suit {
            Suit::Sharpness => sword_sharpness += c.value,
            Suit::Balance => sword_balance += c.value,
            Suit::Durability => sword_durability += c.value,
            Suit::Honor => sword_honor += c.value,
        }
    }
    for full_target in full_targets.iter() {
        let mut target_sharpness = 0;
        let mut target_balance = 0;
        let mut target_durability = 0;
        let mut target_honor = 0;
        for c in full_target.cards.iter() {
            match c.suit {
                Suit::Sharpness => target_sharpness += c.value,
                Suit::Balance => target_balance += c.value,
                Suit::Durability => target_durability += c.value,
                Suit::Honor => target_honor += c.value,
            }
        }

        if sword_sharpness >= target_sharpness {
            sharpness_successes += 1;
        }
        if target_balance > sword_balance {
            balance_fails += 1;
        }
        if target_durability > sword_durability {
            durability_fails += 1;
        }
        if target_honor > sword_honor {
            honor_fails += 1;
        }
    }

    let sharpness_success_rate = sharpness_successes as f64 / full_targets.len() as f64;
    let balance_fail_rate = balance_fails as f64 / full_targets.len() as f64;
    let durability_fail_rate = durability_fails as f64 / full_targets.len() as f64;
    let honor_fail_rate = honor_fails as f64 / full_targets.len() as f64;

    let mut fake_target_deck = target_deck.clone();
    fake_target_deck.extend(discard_pile.iter().cloned());
    fake_target_deck.extend(target.cards.iter().cloned());
    let (_, _, avg_balance_fail_rate, _, avg_honor_fail_rate) = fail_rate(sword, &fake_target_deck);

    let mut target_intact = 1.0f64;
    for s_dec in swing_decisions.iter() {
        target_intact *= 1.0 - s_dec.2 * s_dec.3;
    }
    let cut_chance = sharpness_success_rate * target_intact;

    let mut scaled_balance_penalty = if swing_decisions.len() + 1 == player_count {
        // We are the last in balance rank
        0.0
    } else {
        balance_fail_rate * (1.0 - avg_balance_fail_rate)
    };

    let scaled_durability_penalty = durability_fail_rate * (((13 - sword.trophies - sword.cards.len() as u32) as f64) / 4.0).powf(2.0);

    let mut scaled_honor_penalty = honor_fail_rate * (1.0 - avg_honor_fail_rate);
    if sword.trophies == 6 {
        scaled_balance_penalty *= 0.5;
        scaled_honor_penalty *= 0.5;
    };

    // Convert a floating point value where 1.0 is a definite swing, 0.0 is a 50% swing, and -1.0
    // or less is a 0% swing to a swing_percentage
    let penalty_sum = scaled_balance_penalty + scaled_durability_penalty + scaled_honor_penalty;
    let swing_chance = if cut_chance == 0.0 {
        0.0
    } else {
        cut_chance as f64 / (cut_chance + penalty_sum) as f64
    };

    //println!("\t{} sharpness_success: {}, balance_fail: {}, avg_balance_fail: {}, balance_fear: {}, durability_fail: {} durability_fear: {}, honor_fail: {}, avg_honor_fail: {}, honor_fear: {}, cut_chance: {}, swing_chance: {}", sword.name, sharpness_success_rate, balance_fail_rate, avg_balance_fail_rate, scaled_balance_penalty, durability_fail_rate, scaled_durability_penalty, honor_fail_rate, avg_honor_fail_rate, scaled_honor_penalty, cut_chance, swing_chance);
    return (swing_chance, sharpness_success_rate);
}

pub fn pick_trophy_picker_cpu(sword_idx: usize, target: &Target, swords: &Vec<Sword>, target_deck: &Vec<Card>, discard_pile: &Vec<Card>) -> usize {
    let mut picks: Vec<(usize, usize)> = Vec::new();
    for sword_tup in swords.iter().enumerate().filter(|s| s.0 != sword_idx) {
        picks.push((sword_tup.0, pick_trophy_cpu(sword_tup.0, target, swords, target_deck, discard_pile)));
    }

    let mut fake_target_deck = target_deck.clone();
    let mut fake_target = Target::new();
    for p in picks.iter() {
        if fake_target.cards.contains(&target.cards[p.1]) {
            continue;
        }

        fake_target.add_card(target.cards[p.1].clone());
    }
    for c in target.cards.iter().filter(|c| !fake_target.cards.contains(c)) {
        fake_target_deck.push(c.clone());
    }

    // Pretend we are picking a trophy, but our only options are those trophies we think our
    // opponents would pick
    let preferred_trophy_card = &fake_target.cards[pick_trophy_cpu(sword_idx, &fake_target, swords, &fake_target_deck, discard_pile)];
    
    //println!("Predicted trophies: {:?}, Preferred_Trophy: {}, target: {:?}", fake_target.cards, preferred_trophy_card, target.cards);
    return picks.iter().filter(|p| target.cards[p.1] == *preferred_trophy_card).nth(0).unwrap().0;
}

pub fn pick_trophy_cpu(picker_idx: usize, target: &Target, swords: &Vec<Sword>, target_deck: &Vec<Card>, discard_pile: &Vec<Card>) -> usize {
    // Pick the trophy that gives the largest fail_rate difference between yourself and your best opponent
    let mut all_target_cards = target_deck.clone();
    all_target_cards.extend(discard_pile.iter().cloned());
    all_target_cards.extend(target.cards.iter().cloned());
    let mut opponent_swords: Vec<(f64, &Sword)> = swords.iter().enumerate().filter(|s_tup| s_tup.0 != picker_idx).map(|s_tup| ((7.0 - s_tup.1.trophies as f64) * fail_rate(s_tup.1, &all_target_cards).0, s_tup.1)).collect::<Vec<_>>(); 
    opponent_swords.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let best_opponent_sword = &opponent_swords[0].1;

    let mut max_score = f64::MIN;
    let mut max_index = 0;
    for c_tup in target.cards.iter().enumerate() {
        let mut all_target_cards_after: Vec<Card> = target_deck.clone();
        all_target_cards_after.extend(discard_pile.iter().cloned());
        all_target_cards_after.extend(target.cards.iter().enumerate().filter(|c| c.0 != c_tup.0).map(|c| c.1).cloned());
        
        let our_fail_rate_before = fail_rate(&swords[picker_idx], &all_target_cards).0;
        let opponent_fail_rate_before = fail_rate(best_opponent_sword, &all_target_cards).0;

        let our_fail_rate_after = fail_rate(&swords[picker_idx], &all_target_cards_after).0;
        let opponent_fail_rate_after = fail_rate(best_opponent_sword, &all_target_cards_after).0;

        let score = (our_fail_rate_before - our_fail_rate_after) - (opponent_fail_rate_before - opponent_fail_rate_after);
        //println!("{} says best opponent is {}. For card {}: FRB: {}, FRA: {}, OFRB: {}, OFRA: {}, Score: {}", &swords[picker_idx].name, best_opponent_sword.name, c_tup.1, our_fail_rate_before, our_fail_rate_after, opponent_fail_rate_before, opponent_fail_rate_after, score);
        if score > max_score {
            max_score = score;
            max_index = c_tup.0;
        }
    }

    return max_index;
}

fn fail_rate(sword: &Sword, target_deck: &Vec<Card>) -> (f64, f64, f64, f64, f64) {
    let mut total_targets = 0;
    let mut sharpness_failures = 0;
    let mut balance_failures = 0;
    let mut durability_failures = 0;
    let mut honor_failures = 0;

    let mut sword_sharpness = 0;
    let mut sword_balance = 0;
    let mut sword_durability = 0;
    let mut sword_honor = 0;
    for c in sword.cards.iter() {
        match c.suit {
            Suit::Sharpness => sword_sharpness += c.value,
            Suit::Balance => sword_balance += c.value,
            Suit::Durability => sword_durability += c.value,
            Suit::Honor => sword_honor += c.value,
        }
    }

    for i_tup in target_deck.iter().enumerate() {
        for j_tup in target_deck.iter().enumerate().skip(i_tup.0 + 1) {
            for k_tup in target_deck.iter().enumerate().skip(j_tup.0 + 1) {
                for l in target_deck.iter().skip(k_tup.0 + 1) {
                    let mut target_sharpness = 0;
                    let mut target_balance = 0;
                    let mut target_durability = 0;
                    let mut target_honor = 0;
                    for c in [i_tup.1, j_tup.1, k_tup.1, l] {
                        match c.suit {
                            Suit::Sharpness => target_sharpness += c.value,
                            Suit::Balance => target_balance += c.value,
                            Suit::Durability => target_durability += c.value,
                            Suit::Honor => target_honor += c.value,
                        }
                    }

                    if target_sharpness > sword_sharpness {
                        sharpness_failures += 1;
                    }
                    if target_balance > sword_balance {
                        balance_failures += 1;
                    }
                    if target_durability > sword_durability {
                        durability_failures += 1;
                    }
                    if target_honor > sword_honor {
                        honor_failures += 1;
                    }
                    total_targets += 1;
                }
            }
        }
    }

    let total_failures = sharpness_failures + balance_failures + durability_failures + honor_failures;
    (total_failures as f64 / total_targets as f64, sharpness_failures as f64 / total_targets as f64, balance_failures as f64 / total_targets as f64, durability_failures as f64 / total_targets as f64, honor_failures as f64 / total_targets as f64)
}
