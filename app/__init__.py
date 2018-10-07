from flask import Flask
from config import Config

app = Flask(__name__)
app.config.from_object(Config)


# check if connected to Infura node
from web3.auto.infura import w3
assert w3.isConnected(), 'Unable to connect to Infura.'

from web3 import Web3
web3 = Web3(Web3.HTTPProvider('https://ropsten.infura.io/v3/' + app.config['INFURA_API_KEY']))

from app import routes

