#!/usr/bin/env python3

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

board = board.Board(a_hand, a_discard, a_rests, False, b_hand, b_discard, b_rests, False, position, True)

a_moves, a_weights, b_moves, b_weights = engine.calculate_equilibrium(board)

print('Player A:')
for tup in sorted(zip(a_moves, a_weights), key=lambda x: x[1], reverse=True):
    print(tup)

print('\nPlayer B:')
for tup in sorted(zip(b_moves, b_weights), key=lambda x: x[1], reverse=True):
    print(tup)

print('Player A plays {}'.format(random.choices(a_moves, weights=a_weights, k=1)))
print('Player B plays {}'.format(random.choices(b_moves, weights=b_weights, k=1)))
