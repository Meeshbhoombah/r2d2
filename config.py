import os

class Config(object):
    SECRET_KEY = os.environ.get('SECRET_KEY') or 'you-will-never-guess'

    # Private Network Ports
    NODE_ONE_PORT = 4201
    NODE_ONE_PORT = 4202

