# -*- encoding: utf-8 -*-

"""client.py

Uses `py-trello` to manipulate boards, lists, and cards on the Meeshbhoombah
Trello
"""

import os
from pprint import pprint


from trello import TrelloClient


client = TrelloClient(
    api_key= os.environ.get('TRELLO_API_KEY'),
    api_secret=os.environ.get('TRELLO_SECERET_KEY'),
    token=os.environ.get('TRELLO_TOKEN'),
    token_secret=os.environ.get('TRELLO_SCECRET_TOKEN')
)


# move all ideas to idea board
pprint(client.list_boards())

