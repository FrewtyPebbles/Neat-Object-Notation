import neat_notation
import os
os.environ['FOO'] = 'bar'
print(neat_notation.load("tests/config.neat"))