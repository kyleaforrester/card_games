#!/usr/bin/env python3

import card
from board import Board
import engine
import sys
import json
import random

GAME_COUNT = 1000

def generate_sumos(card_deck):
    a_sumo = []
    b_sumo = []
    while not any(map(lambda x: x.suit == 'C', a_sumo)) or not any(map(lambda x: x.suit == 'C', b_sumo)):
        deck = list(card_deck.keys())
        random.shuffle(deck)
        deck = sorted(deck, key=lambda x: card_deck[x])
        a_sumo = []
        b_sumo = []
        a_sum = 0
        b_sum = 0
        for c in deck:
            if a_sum == 50 and b_sum == 50:
                break
            if random.randint(1,2) == 1:
                if c.value + a_sum <= 50:
                    a_sumo.append(c)
                    a_sum += c.value
                elif c.value + b_sum <= 50:
                    b_sumo.append(c)
                    b_sum += c.value
            else:
                if c.value + b_sum <= 50:
                    b_sumo.append(c)
                    b_sum += c.value
                elif c.value + a_sum <= 50:
                    a_sumo.append(c)
                    a_sum += c.value

    for c in a_sumo + b_sumo:
        card_deck[c] += 1
    return (a_sumo, b_sumo)
    
def play_game(board):
    rounds = 0
    while not board.a_win and not board.b_win and rounds < 100:
        a_moves, a_weights, b_moves, b_weights = engine.calculate_equilibrium(board)
        a_move = random.choices(a_moves, weights=a_weights, k=1)[0]
        b_move = random.choices(b_moves, weights=b_weights, k=1)[0]
        board.resolve_moves(a_move, b_move)
        rounds += 1
                

card_deck = {}
for s in ('C', 'D', 'H', 'S'):
    for v in ('A', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K'):
        card_deck[card.Card(v + s)] = 0

card_scores = {}
for c in list(card_deck.keys()):
    # To keep track of wins, losses, and draws
    card_scores[c] = [0,0,0]

# Play this many games
draws = 0
push_wins = 0
throw_wins = 0
sumo_sizes = {}
for i in range(GAME_COUNT):
    a_sumo, b_sumo = generate_sumos(card_deck)

    a_hand = list(map(lambda x: str(x), a_sumo))
    b_hand = list(map(lambda x: str(x), b_sumo))

    print('Starting game {}: {} vs {}'.format(i, a_hand, b_hand))
    board = Board(a_hand, [], 0, 0, b_hand, [], 0, 0, 2, False)
    play_game(board)

    # Initialize W,L,D for each sized sumo if not already present
    if len(a_sumo) not in sumo_sizes:
        sumo_sizes[len(a_sumo)] = [0,0,0]
    if len(b_sumo) not in sumo_sizes:
        sumo_sizes[len(b_sumo)] = [0,0,0]

    if board.push_win:
        push_wins += 1
    elif board.throw_win:
        throw_wins += 1

    if board.a_win:
        print('Game {} ended. Player A wins!'.format(i))
        for c in a_sumo:
            card_scores[c][0] += 1
        for c in b_sumo:
            card_scores[c][1] += 1
        sumo_sizes[len(a_sumo)][0] += 1
        sumo_sizes[len(b_sumo)][1] += 1
    elif board.b_win:
        print('Game {} ended. Player B wins!'.format(i))
        for c in a_sumo:
            card_scores[c][1] += 1
        for c in b_sumo:
            card_scores[c][0] += 1
        sumo_sizes[len(b_sumo)][0] += 1
        sumo_sizes[len(a_sumo)][1] += 1
    else:
        print('Game {} ended. Draw!'.format(i))
        draws += 1
        for c in a_sumo + b_sumo:
            card_scores[c][2] += 1
        sumo_sizes[len(a_sumo)][2] += 1
        sumo_sizes[len(b_sumo)][2] += 1

for c in sorted(list(card_scores.items()), key=lambda x: (x[0].suit, x[0].value)):
    print('{}: {} W, {} L, {} D, {} Percentage'.format(c[0], c[1][0], c[1][1], c[1][2], round(100 * (c[1][0] + 0.5*c[1][2]) / (c[1][0] + c[1][1] + c[1][2]), 2) if c[1][0] + c[1][1] + c[1][2] > 0 else 0))

for s in ['C', 'D', 'H', 'S']:
    suit_results = list(map(lambda y: y[1], filter(lambda x: x[0].suit == s, card_scores.items())))
    percent_list = [(x[0] + 0.5*x[2]) / (x[0] + x[1] + x[2]) for x in suit_results if x[0] + x[1] + x[2] > 0]
    avg_percent = sum(percent_list) / len(percent_list)
    print('{} avg win percent: {}'.format(s, avg_percent))

for s in sorted(list(sumo_sizes.items()), key=lambda x: x[0]):
    print('Sumo Size {}: {} W, {} L, {} D, {} Total, {} Percentage'.format(s[0], s[1][0], s[1][1], s[1][2], s[1][0] + s[1][1] + s[1][2], round(100 * (s[1][0] + 0.5 * s[1][2])/(s[1][0] + s[1][1] + s[1][2]), 2)))

print('Push win percentage: {}%'.format(round(push_wins * 100 / GAME_COUNT, 2)))
print('Throw win percentage: {}%'.format(round(throw_wins * 100 / GAME_COUNT, 2)))
print('Draw percentage: {}%'.format(round(draws * 100 / GAME_COUNT, 2)))
