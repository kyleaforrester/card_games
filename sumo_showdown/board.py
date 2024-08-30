import card
import engine

class Board:
    def __init__(self, a_hand, a_discard, a_rests, a_human, b_hand, b_discard, b_rests, b_human, position, cpu_output):
        # For players a and b
        # self.a_hand
        # self.a_discard
        # self.a_rests
        # self.b_hand
        # self.b_discard
        # self.b_rests
        # self.position
        # self.a_human
        # self.b_human
        # self.a_win
        # self.b_win
        # self.cpu_output

        self.a_hand = [card.Card(c) for c in a_hand]
        self.b_hand = [card.Card(c) for c in b_hand]

        self.a_discard = [card.Card(c) for c in a_discard]
        self.b_discard = [card.Card(c) for c in b_discard]

        self.a_rests = a_rests
        self.b_rests = b_rests

        self.position = position

        self.a_win = False
        self.b_win = False

        self.a_human = a_human
        self.b_human = b_human

        self.cpu_output = cpu_output

    def get_human_discards(self, player):
        discards = []
        if player == 'a':
            while len(discards) < min(4, len(self.a_hand)) and all(map(lambda x: x in a_hand, discards)):
                response = input('Your hand: {}\nEnter the 4 cards to discard from your hand in csv format: '.format(self.a_hand))
                discards = [card.Card(s.strip()) for s in response.split(',')]
        elif player == 'b':
            while len(discards) < min(4, len(self.b_hand)) and all(map(lambda x: x in b_hand, discards)):
                response = input('Your hand: {}\nEnter the 4 cards to discard from your hand in csv format: '.format(self.b_hand))
                discards = [card.Card(s.strip()) for s in response.split(',')]
        return discards

    def get_cpu_discards(self, player):
        discards = []
        if player == 'a':
            discards = sorted(self.a_hand, key=lambda x: engine.evaluate_card(x, self.b_hand + self.b_discard))[:4]
        elif player == 'b':
            discards = sorted(self.b_hand, key=lambda x: engine.evaluate_card(x, self.a_hand + self.a_discard))[:4]

        if self.cpu_output:
            print('Discarding {}'.format(','.join([c.to_string() for c in discards])))
        return discards

    def get_human_recycle_card(self, player, cards):
        r_card = None
        if player == 'a':
            eligible_cards = self.a_discard + list(filter(lambda x: x.suit != 'D', cards))
        elif player == 'b':
            eligible_cards = self.b_discard + list(filter(lambda x: x.suit != 'D', cards))

        while len(eligible_cards) > 0 and (r_card is None or r_card not in [str(c) for c in eligible_cards]):
            r_card = input('Your eligible recycling cards: {}\nEnter what card you wish to recycle: '.format(eligible_cards))
        return card.Card(r_card)

    def get_cpu_recycle_card(self, player, cards):
        r_card = None
        if player == 'a':
            eligible_cards = sorted(self.a_discard + list(filter(lambda x: x.suit != 'D', cards)), key=lambda x: engine.evaluate_card(x, self.b_hand + self.b_discard))
        elif player == 'b':
            eligible_cards = sorted(self.b_discard + list(filter(lambda x: x.suit != 'D', cards)), key=lambda x: engine.evaluate_card(x, self.a_hand + self.a_discard))

        nonsalt_eligible_cards = [c for c in eligible_cards if c.suit != 'D']
        if len(nonsalt_eligible_cards) > 0:
            r_card = nonsalt_eligible_cards[-1]
        elif len(eligible_cards) > 0:
            r_card = eligible_cards[-1]
        
        if self.cpu_output:
            if r_card is not None:
                print('Recycling card {}'.format(r_card.to_string()))
            else:
                print('Recycling card None')
        return r_card

    def cmp_moves(self, a_cards, b_cards):
        if len(a_cards) == 0 and len(b_cards) == 0:
            return None
        elif len(a_cards) > 0 and (len(b_cards) == 0 or (sum(map(lambda x: x.value, b_cards)) < sum(map(lambda x: x.value, a_cards))) or (sum(map(lambda x: x.value, b_cards)) == sum(map(lambda x: x.value, a_cards)) and max(a_cards, key=lambda x: x.value).value > max(b_cards, key=lambda x: x.value).value)):
            return 'a'
        else:
            return 'b'

    def resolve_moves(self, a_cards, b_cards):
        a_cards = list(a_cards)
        b_cards = list(b_cards)
        for c in a_cards:
            self.a_hand.remove(c)
        for c in b_cards:
            self.b_hand.remove(c)

        # Rest
        if len(a_cards) == 0:
            self.a_rests += 1
            if self.a_rests >= 2:
                self.a_rests = 0
                self.a_hand += self.a_discard
                self.a_discard = []
        if len(b_cards) == 0:
            self.b_rests += 1
            if self.b_rests >= 2:
                self.b_rests = 0
                self.b_hand += self.b_discard
                self.b_discard = []

        a_throw = list(filter(lambda x: x.suit == 'S', a_cards))
        b_throw = list(filter(lambda x: x.suit == 'S', b_cards))
        a_push = list(filter(lambda x: x.suit == 'C', a_cards))
        b_push = list(filter(lambda x: x.suit == 'C', b_cards))
        a_salt = list(filter(lambda x: x.suit == 'D', a_cards))
        b_salt = list(filter(lambda x: x.suit == 'D', b_cards))
        a_slap = list(filter(lambda x: x.suit == 'H', a_cards))
        b_slap = list(filter(lambda x: x.suit == 'H', b_cards))

        throw_result = self.cmp_moves(a_throw, b_throw)
        push_result = self.cmp_moves(a_push, b_push)
        salt_result = self.cmp_moves(a_salt, b_salt)
        slap_result = self.cmp_moves(a_slap, b_slap)

        # Throw
        if throw_result == 'a' and push_result == 'b' and sum(map(lambda x: x.value, a_throw)) >= sum(map(lambda x: x.value, b_push)):
            self.a_win = True
        elif throw_result == 'b' and push_result == 'a' and sum(map(lambda x: x.value, b_throw)) >= sum(map(lambda x: x.value, a_push)):
            self.b_win = True

        # Push
        if push_result == 'a':
            self.position += 1
            if self.position >= 5 and self.b_win == False:
                self.a_win = True
        elif push_result == 'b':
            self.position -= 1
            if self.position < 0 and self.a_win == False:
                self.b_win = True

        # Salt
        if salt_result == 'a':
            for i in range(2):
                if self.a_human:
                    recycle_card = self.get_human_recycle_card('a', a_cards)
                else:
                    recycle_card = self.get_cpu_recycle_card('a', a_cards)
                if recycle_card is not None:
                    self.a_hand.append(recycle_card)
                    if recycle_card in self.a_discard:
                        self.a_discard.remove(recycle_card)
                    elif recycle_card in a_cards:
                        a_cards.remove(recycle_card)
        elif salt_result == 'b':
            for i in range(2):
                if self.b_human:
                    recycle_card = self.get_human_recycle_card('b', b_cards)
                else:
                    recycle_card = self.get_cpu_recycle_card('b', b_cards)
                if recycle_card is not None:
                    self.b_hand.append(recycle_card)
                    if recycle_card in self.b_discard:
                        self.b_discard.remove(recycle_card)
                    elif recycle_card in b_cards:
                        b_cards.remove(recycle_card)

        # Slap
        if slap_result == 'a':
            if self.b_human:
                discards = self.get_human_discards('b')
            else:
                discards = self.get_cpu_discards('b')
            for c in discards:
                self.b_hand.remove(c)
                self.b_discard.append(c)
        elif slap_result == 'b':
            if self.a_human:
                discards = self.get_human_discards('a')
            else:
                discards = self.get_cpu_discards('a')
            for c in discards:
                self.a_hand.remove(c)
                self.a_discard.append(c)

        for c in a_cards:
            self.a_discard.append(c)
        for c in b_cards:
            self.b_discard.append(c)

    def get_moves(self):
        # Returns a list with player A as the first element and player B as the second.
        # Each player is a list of tuples of potential moves to play. Rest is the empty tuple.
        player_moves = []
        for hand in (self.a_hand, self.b_hand):
            moves = []
            for i in range(len(hand) - 1):
                eligible_partners = [c for c in hand[i+1:]]
                for j in range(len(eligible_partners)):
                    moves.append((hand[i], eligible_partners[j]))
            for c in hand:
                moves.append((c,))
            moves.append(())
            player_moves.append(moves)

        return player_moves

    def __str__(self):
        string = 'Player A win? {} Player B win? {}\n'.format(self.a_win, self.b_win)
        string += 'Player A Hand: {}\n'.format(sorted(self.a_hand, key=lambda x: (x.suit, x.value)))
        string += 'Player A Discard: {}\n'.format(self.a_discard)
        string += 'Player A Rests: {}\n'.format(self.a_rests)
        string += 'Player B Hand: {}\n'.format(sorted(self.b_hand, key=lambda x: (x.suit, x.value)))
        string += 'Player B Discard: {}\n'.format(self.b_discard)
        string += 'Player B Rests: {}\n'.format(self.b_rests)
        string += 'Position: {}'.format(self.position)
        return string

    def __repr__(self):
        return __str__(self)
