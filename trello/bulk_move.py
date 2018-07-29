# -*- encoding: utf-8 -*-

"""/r2d2/trello/src/bulk_move.py

Uses `py-trello` to manipulate boards, lists, and cards on the Meeshbhoombah
Trello.
"""

import os
from pprint import pprint


from trello import TrelloClient


def load_account_details(account):
    """ Loads in Trello keys and tokens from the environment

    Loads in environment vars to interact with Twilio accounts
    """
    if account is None:
        client = TrelloClient(
            api_key= os.environ.get('TRELLO_API_KEY'),
            api_secret=os.environ.get('TRELLO_SECERET_KEY'),
            token=os.environ.get('TRELLO_TOKEN'),
            token_secret=os.environ.get('TRELLO_SCECRET_TOKEN')
        )


if __name__ == "__main__":
    pprint(client.list_boards())

