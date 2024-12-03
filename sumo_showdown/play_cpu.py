#!/usr/bin/env python3

import card
import board
import engine
import sys
import json
import random

def read_file_to_json(file_path):
    try:
        with open(file_path, 'r') as file:
            data = file.read()
            json_data = json.loads(data)
            return json_data
    except FileNotFoundError:
        print(f"The file at {file_path} was not found.")
    except json.JSONDecodeError:
        print(f"Error decoding JSON from the file at {file_path}.")
    except Exception as e:
        print(f"An error occurred: {e}")

if len(sys.argv) != 2:
    print('Args given: {}'.format(len(sys.argv)))
    print('Usage:\n./my_script.py path/to/file.json')
    sys.exit(1)

position = read_file_to_json(sys.argv[1])

a_hand = position['Player_A']['Hand']
a_discard = position['Player_A']['Discard']
a_rests = position['Player_A']['Rests']
a_human = position['Player_A']['Human']
b_hand = position['Player_B']['Hand']
b_discard = position['Player_B']['Discard']
b_rests = position['Player_B']['Rests']
b_human = position['Player_B']['Human']
position = position['Position']

board = board.Board(a_hand, a_discard, a_rests, a_human, b_hand, b_discard, b_rests, b_human, position, True)

print(sorted([(c, engine.evaluate_card(c, board.b_hand)) for c in board.a_hand], key=lambda x: x[1]))
print(sorted([(c, engine.evaluate_card(c, board.a_hand)) for c in board.b_hand], key=lambda x: x[1]))

while not board.a_win and not board.b_win:
    board.a_human = False
    board.cpu_output = False
    a_moves, a_weights, b_moves, b_weights = engine.calculate_equilibrium(board)
    board.a_human = a_human
    board.cpu_output = True
    print(board)

    if board.a_human:
        response = ''
        action = ''
        valid_actions = ('board', 'play', 'rest')
        while action not in valid_actions:
            response = input('Valid commands: board, play, rest.\nExamples:\nboard\nplay AH,KS\nplay 5C\nrest\nWhat would you like to do? ')
            action = response.split()[0]
            if action == 'board':
                print(board)
            elif action == 'play':
                move_card_strs = [s.strip() for s in ''.join(response.split()[1:]).split(',')]
                if all(map(lambda x: x in [str(c) for c in board.a_hand], move_card_strs)):
                    move_cards = [card.Card(s) for s in move_card_strs]
                    cpu_cards = random.choices(b_moves, weights=b_weights, k=1)[0]
                    print('Player B plays {}'.format(cpu_cards))
                    board.resolve_moves(move_cards, cpu_cards)
                else:
                    print('Invalid input! Cards not found in hand: {}'.format(list(filter(lambda x: x not in [str(c) for c in board.a_hand], move_card_strs))))
            elif action == 'rest':
                cpu_cards = random.choices(b_moves, weights=b_weights, k=1)[0]
                print('Player B plays {}'.format(cpu_cards))
                board.resolve_moves([], cpu_cards)
            else:
                print('Not a valid command: {}'.format(response))
    else:
        a_move = random.choices(a_moves, weights=a_weights, k=1)[0]
        b_move = random.choices(b_moves, weights=b_weights, k=1)[0]
        print('Player A plays {}'.format(a_move))
        print('Player B plays {}'.format(b_move))
        board.resolve_moves(a_move, b_move)
            
if board.a_win:
    print(board)
    print('Player A wins!')
else:
    print(board)
    print('Player B wins!')

