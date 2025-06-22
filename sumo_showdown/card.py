import sys

class Card:
    def __init__(self, card_str):
        #self.value
        #self.suit
        value = card_str[:-1]
        suit = card_str[-1]

        card_values = {
            'A': 1,
            '2': 2,
            '3': 3,
            '4': 4,
            '5': 5,
            '6': 6,
            '7': 7,
            '8': 8,
            '9': 9,
            '10': 10,
            'J': 11,
            'Q': 12,
            'K': 13,
        }

        if value in card_values:
            self.value = card_values[value]
        else:
            print('Unknown Card value: {}'.format(value))
            sys.exit(1)

        if suit not in ('C', 'D', 'H', 'S'):
            print('Unknown Card suit: {}'.format(suit))
            sys.exit(1)
        self.suit = suit

    def to_string(self):
        card_strings = {
            1: 'A',
            2: '2',
            3: '3',
            4: '4',
            5: '5',
            6: '6',
            7: '7',
            8: '8',
            9: '9',
            10: '10',
            11: 'J',
            12: 'Q',
            13: 'K'
        }

        return card_strings[self.value] + self.suit

    def __eq__(self, other):
        if self.value == other.value and self.suit == other.suit:
            return True
        return False

    def __hash__(self):
        return hash((self.value, self.suit))

    def __str__(self):
        return self.to_string()

    def __repr__(self):
        return self.to_string()
