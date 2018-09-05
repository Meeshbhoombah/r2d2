# -*- encoding: utf-8 -*-

"""Moves cards from the `Inbox` to a given `card address` from the CLI."""


import sys
import json
import requests
from pprint import pprint


class TrelloClient:

    
    BASE_URL = 'https"//trello.com/1/'
    
    
    def __init__(self, secrets = None):
        if not secrets:
            # Use keys loaded into environment from `trello.secrets`
            secrets = {
                'TRELLO_API_KEY'   : os.environ.get('TRELLO_KEY'),
                'TRELLO_API_TOKEN' : os.environ.get('TRELLO_TOKEN')
            }
        
        self.params = { 
            'key'   : secrets['TRELLO_API_KEY'],
            'token' : secrets['TRELLO_API_TOKEN']
        }


    def all_boards(self):
        """Returns a list of all active boards"""


    def bulk_move(items, start, position, stop):
        """Move given cards from given `start` address to `stop` address"""

    
def main(args):


if __name__ == '__main__':
    pprint(sys.argv)

