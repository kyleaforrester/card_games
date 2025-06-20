use crate::sword::Sword;
use crate::card::Card;
use crate::card::Suit;
use crate::prng::Prng;
use crate::target::Target;
use crate::cpu;

use std::io::{self, Write};

#[derive(Debug)]
pub struct Board {
    pub log: bool,
    pub swords: Vec::<Sword>,
    pub target_deck: Vec::<Card>,
    pub discard_pile: Vec::<Card>,
    pub prng: Prng,
}

impl Board {
    pub fn new() -> Board {
        Board {
            log: true,
            swords: Vec::new(),
            target_deck: Vec::new(),
            discard_pile: Vec::new(),
            prng: Prng::new(),
        }
    }

    pub fn add_sword(&mut self, name: &str, is_human: bool) {
        let new_name = if name.len() == 0 {
            self.generate_name()
        } else {
            name
        };
        let sword = Sword::new(new_name, is_human);
        self.swords.push(sword);
        if is_human {
            self.log = true;
        }
    }

    pub fn generate_name(&mut self) -> &str {
        let all_names = vec!["Amakuni", "Yasutsuna", "Sanjo", "Awataguchi", "Rai", "Masamune", "Sadamune", "Muramasa", "Kanemitsu", "Chogi", "Horikawa", "Kunikane", "Kotetsu", "Suishinshi", "Kiyomaro", "Nobuhide"];
        let used_names: Vec<&str> = self.swords.iter().map(|s| s.name.as_str()).collect();

        let mut unused_names: Vec<&str> = all_names
            .iter()
            .copied()
            .filter(|n| !used_names.contains(n))
            .collect();

        return self.prng.choose(&unused_names);
    }

    pub fn generate_sword_name(&mut self) -> &str {
        let all_names = vec!["Godrend","Soulpiercer","Emberfang","Voidcleaver","Kingsunder","Stormvein","Dawnreaper","Moonfang","Ashrend","Twilight Fang","Blazefury","Frosthowl","Thundermarch","Earthshiver","Cindergloom","Mistfang","Stormlash","Sungale","Glacierbite","Infernal Bloom","Whisperfang","Bloodmourne","Gravekiss","Nightbane","Vilethorn","Charnelbrand","Wraithcleft","Oblivion’s Edge","Deathwake","Bane of Aether","Lightweaver","Aetherblade","Sunshard","Seraph’s Oath","Purityfang","Halcyon Edge","Skydawn","Gracepiercer","Blessed Vow","Starlit Grace","Spellrend","Manatide","Runebreaker","Arcspire","Chronoblade","Glyphfang","Echospire","Astralbite","Mythcarver","Sigilfang","Drakefang","Wyrmbane","Leviaclaw","Fenfang","Scalebreaker","Talonstrike","Roarbite","Diretusk","Serpentfang","Riftclaw","Starcleaver","Novaheart","Ecliptor","Voidkiss","Meteorfall","Cometbrand","Galaxblade","Celestash","Skygrave","Blackstar Edge","Thorn of the First King","Oathiron","Wyrmrest Blade","Highfang of Eldurim","Runed Fang of Ys","Crownrender","The Sword of Ten Kings","Watcher’s Fang","Tombfire Blade","The Forgebound Edge","Memoryfang","Whispershade","Hope’s End","Promisebreaker","Widow’s Grin","The Silent Edge","Veilpiercer","The Pale Song","Lament’s Reach","The Quiet Sting","Inkfang","Glassfang","Silkrazor","Chainlight","Mirrorcry","Rosespike","Featheredge","Grimlace","The Laughing Cut","Thornsmile","Bonebreaker","Meatcleaver of the Abyss","Ironhowl","Skullripper","Goreflood","Fleshrazor","Bloodcrag","Carrion Fang","Splitmaw","Gorefang the Red","Warborn Edge","Iron Vow","Shieldbreaker","Bannermaul","Grudgefang","Crimson Oath","Battlewail","Steelstorm","Wrathcleaver","Gauntfang","Hellrender","Ashbrand","Pyreclaw","Cinderreign","Furnacebite","Brimfang","Infernal Scar","Charblade","Scorchmaw","Blazemourn","The Black Maw","Reaper’s Crook","Soulvoid Blade","Gravenedge","The Daggerwind","Obliviscythe","Witherfang","Nulltide","Pale Bane","The Hollow Kiss","The Oathbreaker’s Fang","Covenblade","Chaincurse","Damnation’s Grin","Vowsunder","Sinreaver","Blightshard","Mark of Ruin","Serpent’s Hex","Fang of the Fallen God","Wintervein","Icereap","Frostscar","Glacierbrand","Chillrend","Hailfang","Frozen Grudge","Permaflesh","The Cold Sentence","Shiverfang","Dragonreaver","Fangsplitter","Wyrmscourge","Talondrink","Spirescourge","Wyvernrazor","The Hornsplit Fang","Burning Scales","Clawbane","Razeclaw","Crowncleaver","Thronetaker","Empirefall","Dominion’s Bite","Kingslayer’s Smile","The Sovereign Splitter","Red Throne Fang","Iron Willbreaker","Realmshatter","Chancellor’s Grudge","Gravemarrow","Cryptfang","Funerary Edge","Wailsteel","Skullgrin","Bloodwrit","Epitaph Blade","Grim Scriptor","Charnedge","Elegy of the Dead","Chainsunder","Oblivion Vortex Blade","Spinesplitter Omega","Ironrage Mk.XII","Grimreign Ultra","Blackfang Executioner","Ruinpulse","Apocalion","Venom Nova Blade","Final Sin"];
        let used_names: Vec<&str> = self.swords.iter().map(|s| s.name.as_str()).collect();

        let mut unused_names: Vec<&str> = all_names
            .iter()
            .copied()
            .filter(|n| !used_names.contains(n))
            .collect();

        return self.prng.choose(&unused_names);
    }


    pub fn log(&self, text: &str) {
        if self.log {
            println!("{}", text);
        }
    }

    pub fn draft_card_human(&self, sword_cards: &mut Vec::<Card>) -> Card {
        let mut buffer = String::new();
        loop {
            self.print_draft_cards(sword_cards);
            print!("What card would you like? ");
            io::stdout().flush();
            buffer.clear();
            io::stdin().read_line(&mut buffer).expect("Could not read from stdin");

            let card_tup = match sword_cards.iter().map(|c| c.to_string()).enumerate().filter(|c| c.1 == buffer.trim()).nth(0) {
                Some(c) => c,
                None => continue,
            };

            return sword_cards.swap_remove(card_tup.0);
        }
    }

    pub fn swing_decision_human(&self, sword: &Sword, target: &Target) -> bool {
        let mut buffer = String::new();
        loop {
            print!("Either swing (s) or meditate (m) or view cards (v): ");
            io::stdout().flush();
            buffer.clear();
            io::stdin().read_line(&mut buffer).expect("Could not read from stdin");

            let mut decision = false;
            match buffer.trim() {
                "s" | "swing" => {
                    decision = true;
                },
                "m" | "meditate" => {
                    decision = false;
                },
                "v" | "view cards" => {
                    let mut sorted_target_deck = self.target_deck.clone();
                    let mut sorted_discard_pile = self.discard_pile.clone();
                    sorted_target_deck.sort_unstable_by(|a, b| a.suit.cmp(&b.suit).then_with(|| {a.value.cmp(&b.value)}));
                    sorted_discard_pile.sort_unstable_by(|a, b| a.suit.cmp(&b.suit).then_with(|| {a.value.cmp(&b.value)}));
                    println!("Target deck cards: {}", sorted_target_deck.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","));
                    println!("Discard pile cards: {}", sorted_discard_pile.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","));
                    continue;
                },
                _ => continue,
            };

            return decision;
        }
    }

    pub fn pick_trophy_picker_human(&self, sword_idx: usize, target: &Target) -> usize {
        let mut buffer = String::new();
        loop {
            print!("Select your opponent to choose your trophy: ");
            io::stdout().flush();
            buffer.clear();
            io::stdin().read_line(&mut buffer).expect("Could not read from stdin");

            let picker_idx = match self.swords.iter().enumerate().filter(|s| s.1.name == buffer.trim()).nth(0) {
                Some(tup) => tup.0,
                None => {
                    println!("Error: Invalid opponent: {}", buffer.trim());
                    continue;
                },
            };

            return picker_idx;
        }
    }


    pub fn pick_trophy_human(&self, picker_idx: usize, target: &Target) -> usize {
        let mut buffer = String::new();
        loop {
            print!("You have been chosen to pick the trophy. Enter the target card to remove from the deck as the trophy: ");
            io::stdout().flush();
            buffer.clear();
            io::stdin().read_line(&mut buffer).expect("Could not read from stdin");

            let card_idx = match target.cards.iter().map(|c| c.to_string()).enumerate().filter(|c| c.1 == buffer.trim()).nth(0) {
                Some(c) => c.0,
                None => {
                    println!("Error: {} not in the Target, choose again", buffer.trim());
                    continue;
                },
            };

            return card_idx;
        }
    }

    pub fn draft(&mut self) {
        self.prng.shuffle(&mut self.swords);

        let mut all_cards = Vec::<Card>::new();
        for suit in [Suit::Sharpness, Suit::Durability, Suit::Honor, Suit::Balance].iter() {
            for v in 2..15 {
                all_cards.push(Card {suit: suit.clone(), value: v});
            }
        }
        self.prng.shuffle(&mut all_cards);

        let suit_card_count = self.swords.len() * 6 / 4;
        let mut sword_cards = Vec::<Card>::new();
        for suit in [Suit::Sharpness, Suit::Balance, Suit::Durability, Suit::Honor].iter() {
            for idx_card_tup in all_cards.iter().filter(|c| c.suit == *suit).enumerate() {
                if idx_card_tup.0 < suit_card_count {
                    sword_cards.push(idx_card_tup.1.clone());
                }
                else {
                    self.target_deck.push(idx_card_tup.1.clone());
                }
            }
        }
        self.prng.shuffle(&mut sword_cards);
        self.prng.shuffle(&mut self.target_deck);

        while sword_cards.len() < self.swords.len() * 6 {
            sword_cards.push(self.target_deck.pop().unwrap());
        }

        for turn_idx in (0..self.swords.len()).into_iter().chain((0..self.swords.len()).into_iter().rev()).cycle().take(self.swords.len() * 6) {
            let card = if self.swords[turn_idx].is_human {
                self.draft_card_human(&mut sword_cards)
            } else {
                let draft_card = cpu::draft_card_cpu(&self.swords[turn_idx], &mut sword_cards, &self.target_deck);
                self.log(&format!("{} forges {}", self.swords[turn_idx].name, &draft_card));
                draft_card
            };
            self.swords[turn_idx].add_card(card);
        }
    }

    pub fn create_half_target(&mut self) -> Target {
        let mut target = Target::new();

        while target.cards.len() < 2 {
            if self.target_deck.len() == 0 {
                self.log("Shuffling deck!");
                self.target_deck.append(&mut self.discard_pile);
                self.prng.shuffle(&mut self.target_deck);
            }
            target.cards.push(self.target_deck.pop().unwrap());
        }
        target
    }

    pub fn discard_target(&mut self, mut target: Target) {
        self.discard_pile.append(&mut target.cards);
    }

    pub fn resolve_swing(&mut self, sword_idx: usize, target: &mut Target) -> Option<usize> {
        let mut trophy_idx: Option<usize> = None;

        // Resolve Sharpness
        let sword_sharpness: u32 = self.swords[sword_idx].cards.iter().filter(|c| c.suit == Suit::Sharpness).map(|c| c.value).sum();
        let target_sharpness: u32 = target.cards.iter().filter(|c| c.suit == Suit::Sharpness).map(|c| c.value).sum();

        if target.is_cut == false {
            if sword_sharpness >= target_sharpness {
                target.is_cut = true;
                self.swords[sword_idx].trophies += 1;
                self.log(&format!("{} has cut the target!", self.swords[sword_idx].name));

                let picker_idx = if self.swords[sword_idx].is_human {
                    self.pick_trophy_picker_human(sword_idx, target)
                } else {
                    cpu::pick_trophy_picker_cpu(sword_idx, target, &self.swords, &self.target_deck, &self.discard_pile)
                };

                trophy_idx = if self.swords[picker_idx].is_human {
                    Some(self.pick_trophy_human(picker_idx, target))
                } else {
                    Some(cpu::pick_trophy_cpu(picker_idx, target, &self.swords, &self.target_deck, &self.discard_pile))
                };

                self.log(&format!("{} has chosen {} as {}'s trophy", self.swords[picker_idx].name, target.cards[trophy_idx.unwrap()], self.swords[sword_idx].name));
            } else {
                self.log(&format!("{}'s blade bounces off the target", self.swords[sword_idx].name));
            }
        }
        
        // Resolve Balance
        let sword_balance: u32 = self.swords[sword_idx].cards.iter().filter(|c| c.suit == Suit::Balance).map(|c| c.value).sum();
        let target_balance: u32 = target.cards.iter().filter(|c| c.suit == Suit::Balance).map(|c| c.value).sum();

        if target_balance > sword_balance {
            self.swords[sword_idx].stumbles = true;
            self.log(&format!("{} stumbles and loses their balance!", self.swords[sword_idx].name));
        }

        // Resolve Honor
        let sword_honor: u32 = self.swords[sword_idx].cards.iter().filter(|c| c.suit == Suit::Honor).map(|c| c.value).sum();
        let target_honor: u32 = target.cards.iter().filter(|c| c.suit == Suit::Honor).map(|c| c.value).sum();

        if target_honor > sword_honor {
            self.swords[sword_idx].meditations_remaining = 5;
            self.log(&format!("{} enters the meditation shrine!", self.swords[sword_idx].name));
        }

        // Resolve Durability
        let sword_durability: u32 = self.swords[sword_idx].cards.iter().filter(|c| c.suit == Suit::Durability).map(|c| c.value).sum();
        let target_durability: u32 = target.cards.iter().filter(|c| c.suit == Suit::Durability).map(|c| c.value).sum();

        if target_durability > sword_durability {
            let destroy_idx = self.prng.randint(0, self.swords[sword_idx].cards.len() as u32 - 1) as usize;
            self.log(&format!("{}'s blade is chipped and loses {}!", self.swords[sword_idx].name, self.swords[sword_idx].cards[destroy_idx]));
            self.swords[sword_idx].cards.remove(destroy_idx);

            if self.swords[sword_idx].cards.len() == 0 {
                self.log(&format!("{}'s sword is entirely destroyed.  They are eliminated.", self.swords[sword_idx].name));
            }
        }

        trophy_idx
    }


    pub fn play(&mut self) {
        self.log("");
        for sword in self.swords.iter() {
            let mut sword_cards: Vec<&Card> = sword.cards.iter().collect();
            sword_cards.sort_unstable_by(|a, b| a.suit.cmp(&b.suit).then_with(|| {a.value.cmp(&b.value)}  ));

            self.log(&format!("{} forged {}", sword.name, sword_cards.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(",")));
        }

        self.swords.sort_by(|a, b| b.cmp_balance(&a));

        while !self.swords.iter().any(|s| s.trophies >= 7) {
            let mut target = self.create_half_target();
            if self.log {
                self.print_standings();
                println!("Target: {}", target.cards.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","));
            }

            let mut swing_decisions: Vec<(usize, bool, bool, f64)> = Vec::new();
            for sword in self.swords.iter().enumerate() {
                let (decision, predicted_decision, trophy_chance) = if sword.1.meditations_remaining > 0 {
                    (false, false, 0.0)
                } else if sword.1.is_human {
                    let (prediction, cut_chance) = cpu::swing_decision_cpu(sword.1, &target, &swing_decisions, &self.target_deck, &self.discard_pile, self.swords.len());
                    let dec = self.swing_decision_human(sword.1, &target);
                    (dec, prediction, cut_chance)
                } else {
                    let (dec, cut_chance) = cpu::swing_decision_cpu(sword.1, &target, &swing_decisions, &self.target_deck, &self.discard_pile, self.swords.len());
                    (dec, dec, cut_chance)
                };
                swing_decisions.push((sword.0, decision, predicted_decision, trophy_chance));
            }

            self.log("");
            for s_dec in swing_decisions.iter() {
                if s_dec.1 {
                    self.log(&format!("{} swings!", self.swords[s_dec.0].name));
                } else {
                    self.log(&format!("{} meditates", self.swords[s_dec.0].name));
                    if self.swords[s_dec.0].meditations_remaining > 0 {
                        self.swords[s_dec.0].meditations_remaining -= 1; 
                    }
                }
            }

            if swing_decisions.iter().all(|d| d.1 == false) {
                self.log("All masters meditate; moving to next target");
                self.discard_target(target);
                continue;
            }

            target.merge_halves(self.create_half_target());

            self.log(&format!("Full target is: {}", target.cards.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(",")));

            let mut trophy_idx: Option<usize> = None;
            for s_dec in swing_decisions.iter() {
                if s_dec.1 {
                    match self.resolve_swing(s_dec.0, &mut target) {
                        Some(i) => trophy_idx = Some(i),
                        None => (),
                    }
                }
            }

            let mut sorted: Vec<_> = self.swords
                .iter()
                .enumerate()
                .map(|(i, sword)| {
                    let key = i as f32 + if sword.stumbles { 1.5 } else { 0.0 };
                    (key, sword)
                })
                .collect();
            sorted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            self.swords = sorted.into_iter().map(|(_, sword)| sword.clone()).collect();
            for sword in self.swords.iter_mut() {
                sword.stumbles = false;
            }

            match trophy_idx {
                Some(i) => {
                    target.cards.remove(i);
                },
                None => (),
            }

            self.discard_target(target);
        }

        if self.log {
            self.print_standings();
            let sword_name = self.generate_sword_name().to_string();
            let smith_name = self.swords.iter().filter(|s| s.trophies >= 7).nth(0).unwrap().name.to_string();
            println!("\n{} slashes through the final target to win the game!", smith_name);
            if self.swords.iter().filter(|s| s.trophies >= 7).nth(0).unwrap().is_human {
                println!("{} cheated! There's no other way they could have won. They are now disqualified.", smith_name);
            } else {
                println!("{}: \"By the blessing of the gods, I deem this sword to be known as {}!\"", smith_name, sword_name);
            }
        }


    }

    pub fn print_draft_cards(&self, cards: &Vec::<Card>) {
        println!("");
        for sword in self.swords.iter() {
            let mut sword_cards: Vec<&Card> = sword.cards.iter().collect();
            sword_cards.sort_unstable_by(|a, b| a.suit.cmp(&b.suit).then_with(|| {a.value.cmp(&b.value)}  ));

            println!("{} forged {}", sword.name, sword_cards.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","));
        }

        let mut sharpness_cards: Vec::<Card> = cards.iter().filter(|c| c.suit == Suit::Sharpness).map(|c| c.clone()).collect();
        sharpness_cards.sort_unstable_by(|a, b| a.value.cmp(&b.value));
        let mut balance_cards: Vec::<Card> = cards.iter().filter(|c| c.suit == Suit::Balance).map(|c| c.clone()).collect();
        balance_cards.sort_unstable_by(|a, b| a.value.cmp(&b.value));
        let mut durability_cards: Vec::<Card> = cards.iter().filter(|c| c.suit == Suit::Durability).map(|c| c.clone()).collect();
        durability_cards.sort_unstable_by(|a, b| a.value.cmp(&b.value));
        let mut honor_cards: Vec::<Card> = cards.iter().filter(|c| c.suit == Suit::Honor).map(|c| c.clone()).collect();
        honor_cards.sort_unstable_by(|a, b| a.value.cmp(&b.value));
        println!("Sharpness\tBalance\t\tDurability\tHonor");
        while sharpness_cards.len() > 0 || balance_cards.len() > 0 || durability_cards.len() > 0 || honor_cards.len() > 0 {
            match sharpness_cards.pop() {
                Some(c) => print!("{}\t\t", c),
                None => print!("\t\t"),
            }
            match balance_cards.pop() {
                Some(c) => print!("{}\t\t", c),
                None => print!("\t\t"),
            }
            match durability_cards.pop() {
                Some(c) => print!("{}\t\t", c),
                None => print!("\t\t"),
            }
            match honor_cards.pop() {
                Some(c) => println!("{}", c),
                None => println!(""),
            }
        }
    }

    pub fn print_standings(&self) {
        println!("");
        println!("Meditation shrine:");
        for sword_tup in self.swords.iter().enumerate().filter(|s| s.1.meditations_remaining > 0) {
            let mut sword_cards: Vec<&Card> = sword_tup.1.cards.iter().collect();
            sword_cards.sort_unstable_by(|a, b| a.suit.cmp(&b.suit).then_with(|| {a.value.cmp(&b.value)}  ));
            println!("{}'s blade is {}. Meditations remaining: {}. Balance rank: {}. Trophies: {}", sword_tup.1.name, sword_cards.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","), sword_tup.1.meditations_remaining, sword_tup.0 + 1, sword_tup.1.trophies);
        }

        println!("Balance rank:");
        for sword_tup in self.swords.iter().enumerate().filter(|s| s.1.meditations_remaining == 0) {
            let mut sword_cards: Vec<&Card> = sword_tup.1.cards.iter().collect();
            sword_cards.sort_unstable_by(|a, b| a.suit.cmp(&b.suit).then_with(|| {a.value.cmp(&b.value)}  ));
            println!("{}'s blade is {}. Balance rank: {}. Trophies: {}", sword_tup.1.name, sword_cards.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","), sword_tup.0 + 1, sword_tup.1.trophies);
        }
    }
}
