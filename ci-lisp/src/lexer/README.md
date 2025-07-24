There are three lexers.

The "Base" Layer:
  - pushes everything to cur_word and never flushes it (stupid)

The "Core" Layer:
  - curly brackets
  - flush words
  - quotes
  - let everything else fall through to base layer

The "Sugar" Layer:
  - infix syntax
  - lets everything else fall through to core layer
