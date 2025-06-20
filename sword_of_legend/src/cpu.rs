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
    let current_fail_rate = suit_fail_rate(&card.suit, sword, target_deck);

    let mut new_sword = sword.clone();
    new_sword.cards.push(card.clone());
    let new_fail_rate = suit_fail_rate(&card.suit, &new_sword, target_deck);

    current_fail_rate - new_fail_rate
}

fn suit_fail_rate(suit: &Suit, sword: &Sword, target_deck: &Vec<Card>) -> f64 {
    let cards: f64 = target_deck.len() as f64;
    let suit_cards_iter = target_deck.iter().filter(|c| c.suit == *suit);
    let nonsuit_count: f64 = cards - suit_cards_iter.clone().count() as f64;
    let sword_value: u32 = sword.cards.iter().filter(|c| c.suit == *suit).map(|c| c.value).sum();


    let one_card_chance = 4.0 * (1.0 / cards) * (nonsuit_count)/(cards-1.0) * (nonsuit_count-1.0)/(cards-2.0) * (nonsuit_count-2.0)/(cards-3.0);
    let two_card_chance = 6.0 * (2.0 / cards) * 1.0/(cards-1.0) * (nonsuit_count)/(cards-2.0) * (nonsuit_count-1.0)/(cards-3.0);
    let three_card_chance = 4.0 * (3.0 / cards) * 2.0/(cards-1.0) * 1.0/(cards-2.0) * (nonsuit_count)/(cards-3.0);
    let four_card_chance = (4.0 / cards) * 3.0/(cards-1.0) * 2.0/(cards-2.0) * 1.0/(cards-3.0);

    let mut failure_rate = 0.0f64;

    failure_rate += one_card_chance * (suit_cards_iter.clone().filter(|c| c.value > sword_value).count() as f64);
    //println!("\tSword {} with singles failure rate {} after singles: {}", sword_value, one_card_chance, failure_rate);

    for a in suit_cards_iter.clone().enumerate() {
        for b in suit_cards_iter.clone().skip(a.0 + 1) {
            if a.1.value + b.value > sword_value {
                //println!("\tSword {} Failed {} and {}. Total {}. Adding {}", sword_value, a.1.value, b.value, a.1.value + b.value, two_card_chance);
                failure_rate += two_card_chance;
            }
        }
    }

    for a in suit_cards_iter.clone().enumerate() {
        for b in suit_cards_iter.clone().enumerate().skip(a.0 + 1) {
            for c in suit_cards_iter.clone().skip(b.0 + 1) {
                if a.1.value + b.1.value + c.value > sword_value {
                    //println!("\tSword {} Failed {} and {} and {}. Total {}. Adding {}", sword_value, a.1.value, b.1.value, c.value, a.1.value + b.1.value + c.value, three_card_chance);
                    failure_rate += three_card_chance;
                }
            }
        }
    }

    for a in suit_cards_iter.clone().enumerate() {
        for b in suit_cards_iter.clone().enumerate().skip(a.0 + 1) {
            for c in suit_cards_iter.clone().enumerate().skip(b.0 + 1) {
                for d in suit_cards_iter.clone().skip(c.0 + 1) {
                    if a.1.value + b.1.value + c.1.value + d.value > sword_value {
                        //println!("\tSword {} Failed {} and {} and {} and {}. Total {}. Adding {}", sword_value, a.1.value, b.1.value, c.1.value, d.value, a.1.value + b.1.value + c.1.value + d.value, four_card_chance);
                        failure_rate += four_card_chance;
                    }
                }
            }
        }
    }

    failure_rate
}

pub fn swing_decision_cpu(sword: &Sword, target: &Target, swing_decisions: &Vec<(usize, bool, bool, f64)>, target_deck: &Vec<Card>, discard_pile: &Vec<Card>, player_count: usize) -> (bool, f64) {
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

    let cut_chance = match swing_decisions.iter().filter(|s_d| s_d.2).map(|s_d| s_d.3).max_by(|a, b| a.partial_cmp(&b).unwrap()) {
        Some(m) => sharpness_success_rate - m,
        None => sharpness_success_rate,
    };

    let scaled_balance_penalty = if swing_decisions.len() + 1 == player_count {
        // We are the last in balance rank
        0.0
    } else {
        balance_fail_rate
    };
    let scaled_balance_penalty = balance_fail_rate * (3.0 - (swing_decisions.len() as f64)) / 3.0 * (49.0 - (sword.trophies * sword.trophies) as f64) / 49.0 * (sword.cards.len() as f64) / 6.0;
    let scaled_durability_penalty = durability_fail_rate * ((13 - sword.trophies - sword.cards.len() as u32) as f64) / 4.0;
    let mut scaled_honor_penalty = honor_fail_rate * (sword.cards.len() as f64) / 6.0;
    if sword.trophies == 6 {
        scaled_honor_penalty *= 0.5;
    };

    let decision = if cut_chance - scaled_balance_penalty - scaled_durability_penalty - scaled_honor_penalty > 0.0 {
        true
    } else {
        false
    };

    //println!("\t{} sharpness_success: {}, balance_fail: {}, balance_scaled: {}, durability_fail: {} durability_scaled: {}, honor_fail: {}, honor_scaled: {}, cut_chance: {}, decision: {}", sword.name, sharpness_success_rate, balance_fail_rate, scaled_balance_penalty, durability_fail_rate, scaled_durability_penalty, honor_fail_rate, scaled_honor_penalty, cut_chance, cut_chance - scaled_balance_penalty - scaled_durability_penalty - scaled_honor_penalty);
    return (decision, sharpness_success_rate);
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
    let best_opponent_sword = swords.iter().enumerate().filter(|s_tup| s_tup.0 != picker_idx).max_by_key(|s_tup| 5 * (s_tup.1.trophies as i32 + s_tup.1.cards.len() as i32) - s_tup.1.meditations_remaining as i32).unwrap().1;

    let mut max_score = f64::MIN;
    let mut max_index = 0;
    for c_tup in target.cards.iter().enumerate() {
        let mut all_target_cards_after: Vec<Card> = target_deck.clone();
        all_target_cards_after.extend(discard_pile.iter().cloned());
        all_target_cards_after.extend(target.cards.iter().enumerate().filter(|c| c.0 != c_tup.0).map(|c| c.1).cloned());
        
        let mut all_target_cards_before = all_target_cards_after.clone();
        all_target_cards_before.push(c_tup.1.clone());

        let our_fail_rate_before = fail_rate(&swords[picker_idx], &all_target_cards_before);
        let opponent_fail_rate_before = fail_rate(best_opponent_sword, &all_target_cards_before);

        let our_fail_rate_after = fail_rate(&swords[picker_idx], &all_target_cards_after);
        let opponent_fail_rate_after = fail_rate(best_opponent_sword, &all_target_cards_after);

        let score = (our_fail_rate_before - our_fail_rate_after) - (opponent_fail_rate_before - opponent_fail_rate_after);
        //println!("{} says best opponent is {}. For card {}: FRB: {}, FRA: {}, OFRB: {}, OFRA: {}, Score: {}", &swords[picker_idx].name, best_opponent_sword.name, c_tup.1, our_fail_rate_before, our_fail_rate_after, opponent_fail_rate_before, opponent_fail_rate_after, score);
        if score > max_score {
            max_score = score;
            max_index = c_tup.0;
        }
    }

    return max_index;
}

fn fail_rate(sword: &Sword, target_deck: &Vec<Card>) -> f64 {
    let mut total_targets = 0;
    let mut failures = 0;

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
                        failures += 1;
                    }
                    if target_balance > sword_balance {
                        failures += 1;
                    }
                    if target_durability > sword_durability {
                        failures += 1;
                    }
                    if target_honor > sword_honor {
                        failures += 1;
                    }
                    total_targets += 1;
                }
            }
        }
    }

    failures as f64 / total_targets as f64
}
