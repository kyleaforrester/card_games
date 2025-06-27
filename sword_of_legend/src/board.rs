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

    pub fn generate_insult(&mut self) -> &str {
        let insults = vec!["Your blade folds more than silk in the spring wind.","I mistook your sword for a farmer’s plow.","The gods weep when your steel is quenched.","Your edge fears flesh more than a monk fears wine.","A child could break your sword over their knee — if they didn’t laugh first.","Your hamon is as crooked as your ambitions.","Your steel rings like a cracked bell.","Even rust refuses to cling to your blade.","I’ve seen soup spoons with greater temper.","Your sword wavers like your resolve.","You hammer iron, but you never forge soul.","The fire dies in shame when it touches your steel.","Even the anvil turns away when you approach.","Your forge breathes smoke, not spirit.","My apprentice's first failure sings clearer than your finest blade.","You strike as if afraid to wake the kami.","I smell fear in the folds of your steel.","You quench cowardice.","Your hands are steady, but your heart is dull.","You forge weapons, I forge legends.","Bamboo holds truer than your iron.","I’d trust a broken branch before your tang.","Your steel is river mud with ambition.","A drunken blacksmith with wet charcoal could outdo you.","You temper your swords in bathwater, I in stormlight.","Even sand offers more resistance.","Your steel sings like a widow—broken and shrill.","If your blade were any softer, it would bend to a whisper.","Your metal whines. Mine speaks.","I forge from the mountain’s heart. You from its shadow.","Your name will be forgotten before your sword dulls.","You will be remembered only in the warnings of others.","When my blade is passed down, yours will be buried.","You sign your blades. I sign history.","Even dogs won't sniff at your forge scraps.","Your work is a footnote to my masterpiece.","Your ancestors beg me for mercy on your behalf.","You chase legacy. I leave it in my wake.","One day, your grandson will claim my name.","You will be remembered only for who you could not surpass.","Your sword dances like a leaf — light and lifeless.","Your soul does not reach your steel.","The cherry blossom falls with more strength than your cut.","Even the moon would rather reflect on my blade than yours.","Your blade lacks spirit, like ink without poetry.","I’ve seen clouds sharper than your edge.","Your katana is a lullaby; mine, a thunderclap.","The fox spirits laugh at your folds.","When I forge, even Fuji bows. When you forge, it sleeps.","Your sword has no silence in it — only noise.","You did your best. I did better.","A poor sword teaches more than a poor swordsmith.","The gods give fire to all, but not hands to use it.","You walk the path of a smith, but you do not leave footprints.","We both swing hammers — only one echoes through time.","Your forge smokes, mine speaks.","Your sword has weight. Mine has purpose.","Even your perfection tastes of failure.","You carry the pride of a master and the product of an apprentice.","A blade may gleam — until it meets another.","Your sword is best left in its sheath, for both your sake and theirs.","If your blade met mine, it would shatter from shame.","Even ghosts would not fear your steel.","I fear no man who wields your sword.","Your sword would not pierce silk on a still day.","My blade has tasted war. Yours still tastes of oil.","Your edge is blunt, like your ambition.","No warrior would draw your blade twice.","Your sword fits a samurai’s hand like lies fit your mouth.","I offer death in a single stroke. You offer disappointment.","Your sword is for show. Mine is for silence.","A mirror reflects more honestly than your polish.","You made a sculpture. I made a weapon.","It is not the gold on the hilt, but the soul in the blade.","Yours sparkles. Mine severs.","You polish to hide the truth. I polish to reveal it.","A geisha could wear your blade as an ornament.","Your katana catches eyes, not breath.","Form without cut is vanity.","I do not chase beauty. It comes begging.","You cannot cut, so you cut corners.","I make swords to stop wars. You make them to impress drunk lords.","My steel bites. Yours apologizes.","You can name your sword — no one else will.","Even your scabbard rejects your blade.","Your blade is neither feared nor praised — only ignored.","Every clang from your forge is a warning to true craftsmen.","You grind steel like bones, but lack the marrow.","My blade dances. Yours stumbles.","A thousand folds, and still no edge.","If I am the mountain, you are the shadow cast behind it.","Your greatest blade will serve my lowest pupil.","When your line ends, mine will forge on.","Your sword will be melted and reforged — in someone else’s name.","If you were my apprentice, I’d have spared the world your shame.","I will outlast you in metal and memory.","Your sword will one day be held by fools, mine by legends.","The emperor may never know your name, but he will know my work.","In a thousand years, only one of us will be remembered.","When you die, may your sword weep with relief."];

        return self.prng.choose(&insults);
    }

    pub fn generate_haiku(&mut self) -> &str {
        let haikus = vec!["Yoru no Kaze ('Night Wind'):\nWhispers in moonlight\nNo blood, only wind remains\nThe blade sleeps again","Kurotsuki ('Black Moon'):\nDarkness rides the curve\nCold crescent above still breath\nOne cut, endless night","Shizukesa ('Stillness'):\nSilence falls again\nBefore the blossom touches\nMy edge already","Chi no Hana ('Blood Blossom'):\nPetals never fall\nSo red upon the white snow\nSpring comes with a scream","Inazuma ('Lightning'):\nA flash in the dusk\nNot seen before it is felt\nThunder walks behind","Kurokami ('Black Hair'):\nSheathed in her long hair\nThe blade rests across her lap\nDeath wears no armor","Yami no Koe ('Voice of Darkness'):\nIt does not whisper\nDarkness does not need a tongue\nOnly a sharp will","Hakanai Ame ('Ephemeral Rain'):\nRaindrops on the steel\nEach one a fleeting moment\nAnd then—just silence","Tengoku no Kizu ('Heaven's Scar'):\nSky opened for me\nIt wept through the metal fold\nNow it remembers","Tsurugi no Suna ('Sword of Sand'):\nBuilt from broken time\nGrains sharpened into a blade\nDeserts remember","Kuroi Tsuru ('Black Crane'):\nWings never flutter\nA single line through still air\nThen, nothing but red","Utakata ('Transient Foam'):\nWhat was once a man\nScattered like seafoam at dawn\nA breath—now forgotten","Hibana ('Spark'):\nOne spark, then the flame\nA village gone in silence\nHe stood, blade still hot","Yukigeshō ('Snow Makeup'):\nPowder on her cheek\nA red mark forms beneath it\nSo warm in the snow","Kaminari ('Thunder'):\nDrums in the mountain\nHe draws once—clouds split in fear\nThunder never lies","Tsukikage ('Moon’s Shadow'):\nShe never speaks loud\nBut follows every heartbeat\nShadow of the moon","Shōmetsu ('Annihilation'):\nNo word can remain\nWhere my edge makes its first kiss\nAsh is the last breath","Kagerō ('Heat Haze'):\nYou thought you saw me\nBut that shimmer was your end\nNow the sun is red","Hisame ('Cold Rain'):\nA thousand small deaths\nFall upon the soldier’s helm\nEach drop finds its mark","Yūrei ('Ghost'):\nNo footsteps to hear\nNo glint before it passes\nA ghost wields the void","Kujaku no Me ('Peacock’s Eye'):\nBeauty’s final gaze\nDazzling and unblinking still\nDeath with grace and pride","Makuragami ('Pillow Hair'):\nShe let her hair down\nAnd never rose up again\nSteel sang her a song","Shikkoku ('Jet Black'):\nNot even starlight\nCould see its arc through the dark\nBut the blood gleamed bright","Kazanami ('Volcanic Wave'):\nIt came with no sound\nAnd left only molten bone\nNo need for a name","Hoshikuzu ('Stardust'):\nThe stars don’t fall fast\nBut when they do, mountains weep\nOne blade brought the sky","Urami ('Grudge'):\nHe never forgot\nThe blood owed by a father\nNow paid with steel light","Tensei ('Heavenly Quiet'):\nNo wind dares to breathe\nWhen this blade leaves its cradle\nEven gods kneel down","Oni no Namida ('Demon’s Tear'):\nHe cried one last time\nA tear slid down the sharp edge\nThe oni slept then","Shiori no Ha ('Bookmark Blade'):\nEach soul I have marked\nTurns the page of history\nA red-stained chapter","Gin no Ame ('Silver Rain'):\nCut once in moonlight\nAnd see silver droplets fall\nToo bright to be tears","Ketsueki no Michi ('Path of Blood'):\nFootsteps made of red\nEach one deeper than the last\nThe path walks itself","Seijaku no Ha ('Blade of Silence'):\nNo scream ever came\nThe wind alone told the tale\nOf one clean, bright arc","Kagehime ('Shadow Princess'):\nShe danced with the dusk\nAnd laid her kiss on their necks\nNever to return","Shiranami ('White Wave'):\nCresting in moonlight\nOne final wave breaks their line\nThe sea stands silent","Tsuyu no Yaiba ('Blade of Dew'):\nSo soft on the grass\nYet sharper than a promise\nGone before the dawn","Kuroyuki ('Black Snow'):\nSnow should not fall dark\nYet ash floats gently downward\nThey sleep beneath it","Hōka ('Flame Blossom'):\nThe cherry tree burns\nRed blooms fall like dying hands\nSpring died screaming flame","Fukō ('Misfortune'):\nNo one draws this blade\nUnless all hope has been lost\nOr vengeance remains","Shin’en ('Abyss'):\nStare long if you dare\nThe blade does not blink or yield\nThe void carries it","Akatsuki ('Dawn'):\nNight clings to my steel\nBut it knows the dawn will come\nJust as I arrive","Kurayami ('Deep Darkness'):\nEven fire hides\nWhen this blade enters the room\nDarkness has its own","Namida Ame ('Rain of Tears'):\nSo many have wept\nYet it cuts them all the same\nRain hides nothing now","Meikyo ('Clear Mirror'):\nYour soul looks within\nAnd sees what you try to hide\nThen I set it free","Umi no Yami ('Sea of Darkness'):\nThe tide never stops\nIt pulls down men and cities\nSo too does this blade","Shōgeki ('Shock'):\nThe wind stands in awe\nAs I draw it with one breath\nAll falls still at once","Kuchi no Nai ('Without a Mouth'):\nThis sword does not speak\nBut you will hear its meaning\nWhen your voice is gone","Jōnetsu ('Passion'):\nRed lips, red blade both\nLeave stains that can’t be undone\nSome loves do not fade","Kōri no Tsubasa ('Wings of Ice'):\nIt strikes like snowfall\nBut with the bite of winter\nStill and absolute","Yūwaku ('Temptation'):\nShe drew it with grace\nAnd the world bent to her will\nEven the steel sighed","Harusame ('Spring Rain'):\nSoftest of the cuts\nBut nothing blooms after it\nOnly the wet earth","Ashioto ('Footstep'):\nBefore you can turn\nThe sound has already passed\nSo have your last thoughts","Omoide ('Memory'):\nEach soul left its mark\nBut only I can recall\nThe names on my steel","Zankoku ('Cruelty'):\nEven mercy weeps\nWhen this blade offers its kiss\nClean, but never kind","Kōun ('Good Fortune'):\nOnly one escapes\nAnd calls it a lucky turn\nThe rest feed the luck","Hagane no Hana ('Flower of Steel'):\nIt bloomed in their chest\nPetals of sharp shimmering\nFate is often fair","Chinmoku ('Silence'):\nI said nothing once\nAnd ten men fell where they stood\nNow I speak in cuts","Gekkō ('Moonlight'):\nYou never saw me\nJust a glimmer in the dark\nThen—your shadow gone","Sasayaki ('Whisper'):\nNot even wind hears\nThe way my blade seeks its path\nA whisper, then peace","Ruriiro ('Lapis Hue'):\nBlue like the seafoam\nBefore it turns red with blood\nThe tide does not lie","Kanzashi ('Hairpin'):\nTucked in the black hair\nIt shines with a deadly charm\nAnd takes more than hearts","Tōmei ('Invisible'):\nTry to name its path\nAnd you'll find you're already\nLying in its wake","Ishi no Koe ('Voice of Stone'):\nStone spoke once to me\nNow I shape it into death\nMy blade tells its tale","Kōri no Namida ('Tear of Ice'):\nIt falls without sound\nAnd melts on the cheek of death\nColder than regret","Kazan ('Volcano'):\nStillness before flame\nOne cut cracks the earth in two\nA mountain broken","Tōka ('Distant Fire'):\nFar across the hill\nA light flickers in the dark\nNo, a sword has drawn","Sabishii Ha ('Lonely Blade'):\nNo hand may hold it\nNone return from wielding it\nSo it sleeps alone","Yoru no Namida ('Tears of Night'):\nNight never forgets\nThe ones it must carry home\nI help it remember","Unmei no Toge ('Thorn of Fate'):\nEven fate can bleed\nThis thorn chose a different path\nAnd I followed it","Kagehōō ('Shadow Phoenix'):\nBurned once into ash\nIt returns, blade in its wing\nA rebirth through war","Kodoku ('Solitude'):\nI forged it alone\nAnd alone it fights with me\nWe understand death","Hi no Tōri ('Path of Fire'):\nOne red streak across\nThe battlefield—just one line\nAnd all was silent","Kurobara ('Black Rose'):\nEach petal a wound\nEach thorn a whispered farewell\nThe rose blooms in blood","Tsukiyo ('Moonlit Night'):\nIt draws no shadow\nEven the moon bows to it\nWhite steel in the dark","Kuroi Ame ('Black Rain'):\nAsh falls like the rain\nFrom rooftops that scream in flame\nIt ends with my blade","Seishin ('Spirit'):\nI gave it my breath\nIt now moves before I speak\nWe are one in war","Nami no Uta ('Song of the Waves'):\nThe sea sang to me\nAnd I forged it into steel\nNow the sea bleeds too","Jigoku no Hana ('Hellflower'):\nA bloom from fire’s root\nSweet and terrible to see\nSmells of burnt honor","Tsubaki ('Camellia'):\nRed as the spring bloom\nSoft in name, but not in bite\nPetals fall in blood","Gisei ('Sacrifice'):\nOne blade, one promise\nA thousand gave it their name\nIt remembers all","Kazanami ('Fire Wave'):\nA wave made of flame\nRolls across the battlefield\nWith one silent stroke","Kōri no Tsume ('Claw of Ice'):\nNo flame can resist\nWhere its edge finds its new home\nCold wins every time","Sokuten ('Sky Sever'):\nOnce the sky was whole\nThen my sword traced its new path\nThe stars never healed","Byakuya ('White Night'):\nEven in bright snow\nThe red shows up far too well\nAnd will not be washed","Mumei ('Nameless'):\nNo name can contain\nWhat this sword has come to do\nIt needs none to speak","Yuragi ('Waver'):\nIt trembles in hand\nBut not from fear, only fate\nDeath holds steady grip","Kaze no Koe ('Voice of Wind'):\nThe wind speaks to me\nAnd asks what I plan to cut\nIt already knows","Kōri no Kokoro ('Heart of Ice'):\nNo warmth ever came\nFrom the hands that shaped this blade\nOnly perfect chill","Yami no Hana ('Dark Flower'):\nIt bloomed in shadow\nFed on blood, watered by tears\nStill, it was lovely","Kibishi ('Severe'):\nNot a curve too soft\nNot a whisper left in it\nOnly one command","Reikon ('Soul'):\nThis blade has a soul\nAnd it devours all others\nTo feed its own voice","Hishō ('Soaring Death'):\nIt rises with wind\nThen falls before you can blink\nDeath flies with no wings","Kuroi Tsubasa ('Black Wing'):\nIt cuts like a crow\nOver fields already stilled\nIts shadow is sharp","Kieta Yume ('Vanished Dream'):\nA dream once held tight\nCut away with silent grace\nNow no one recalls","Unmei no Ito ('Thread of Fate'):\nYou pulled the wrong thread\nAnd everything came undone\nEven your last breath","Aka Hōō ('Red Phoenix'):\nIt does not burn down\nIt rises with every strike\nWings soaked in scarlet","Tsumi ('Sin'):\nEach cut a prayer said\nIn apology or guilt\nBut none reach the gods","Yūrei no Namida ('Ghost’s Tear'):\nIt falls from no eye\nBut still wets the steel with grief\nCut by sorrow’s weight","Kagegiri ('Shadowcut'):\nYou see your shadow\nThen it leaps away from you\nMy sword moves faster","Arashi ('Storm'):\nNot wind, nor thunder\nOnly the flash between them\nThat is when I strike","Ketsuzoku ('Bloodline'):\nThis blade remembers\nEach ancestor who has bled\nAnd it still is thirsty"];

        return self.prng.choose(&haikus);
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
        for i in 0..(((self.swords.len()-1)/4)+1) {
            for suit in [Suit::Sharpness, Suit::Durability, Suit::Honor, Suit::Balance].iter() {
                for v in 2..15 {
                    all_cards.push(Card {suit: suit.clone(), value: v});
                }
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
            self.swords[sword_idx].meditations_remaining = self.swords.len() as u32 + 1;
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

            let mut swing_decisions: Vec<(usize, bool, f64, f64)> = Vec::new();
            for sword in self.swords.iter().enumerate() {
                let (decision, swing_chance, trophy_chance) = if sword.1.meditations_remaining > 0 || sword.1.cards.len() == 0 {
                    (false, 0.0, 0.0)
                } else if sword.1.is_human {
                    let (swing_chance, cut_chance) = cpu::swing_decision_cpu(sword.1, &target, &swing_decisions, &self.target_deck, &self.discard_pile, self.swords.len());
                    let dec = self.swing_decision_human(sword.1, &target);
                    (dec, swing_chance, cut_chance)
                } else {
                    let (swing_chance, cut_chance) = cpu::swing_decision_cpu(sword.1, &target, &swing_decisions, &self.target_deck, &self.discard_pile, self.swords.len());
                    let dec = if swing_chance > self.prng.random() {
                        true
                    } else {
                        false
                    };
                    (dec, swing_chance, cut_chance)
                };
                swing_decisions.push((sword.0, decision, swing_chance, trophy_chance));
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
            let insult = self.generate_insult().to_string();
            let haiku = self.generate_haiku().to_string();
            let smith_name = self.swords.iter().filter(|s| s.trophies >= 7).nth(0).unwrap().name.to_string();
            println!("");
            if self.swords.iter().filter(|s| s.trophies >= 7).nth(0).unwrap().is_human {
                println!("{} cheated! There's no other way they could have won. They are now disqualified.", smith_name);
            } else {
                println!("{}: \"{}\"", smith_name, insult);
                println!("{}: \"{}\"", smith_name, haiku);
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

        let meditating_swords_tup: Vec<(usize, &Sword)> = self.swords.iter().enumerate().filter(|s| s.1.meditations_remaining > 0).collect::<Vec<_>>();
        if meditating_swords_tup.len() > 0 {
            println!("Meditation shrine:");
        }
        for sword_tup in meditating_swords_tup.iter() {
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
