from enum import Enum, auto
import sys

class TokenType(Enum):
    NUMBER   = auto() # 1..9
    PLUS     = auto() # +
    MINUS    = auto() # -
    EOF      = auto() # EOF

class Token():
    def __init__(self, ty: TokenType, val: str):
        self.ty  = ty
        self.val = val

    def __repr__(self):
        return f"{self.val}"

class Lexer():
    def __init__(self, code: str):
        self.code   = code
        self.pos    = 0
        self.ch     = ''
        self.tokens = []
        
    def tokenize(self):
        while not self.is_at_end(self.pos):
            self.next_token()

        self.add_token(TokenType.EOF, "EOF")
        return self.tokens

    def next_token(self):
        self.advance()
        self.skip_whitespace()

        # print(f"char saat ini = {self.ch}")
        if self.ch == '+':
            self.add_token(TokenType.PLUS, '+')
        elif self.ch == '-':
            self.add_token(TokenType.MINUS, '-')
        elif self.ch.isnumeric:
            self.parse_int()
        else:
            print(f"Error: unknown characters '{self.ch}'", file=sys.stderr)

    def skip_whitespace(self):
        if self.ch == ' ' or self.ch == '\n' or self.ch == '\t' or self.ch == '\r':
            self.advance()


    def advance(self):
        # print(f"{len(self.code)}, {self.pos}")
        self.ch = self.code[self.pos] if not self.is_at_end(self.pos) else ''
        self.pos += 1

    def parse_int(self):
        num = ''

        while self.ch.isnumeric():

            num += self.ch
            self.advance()
            if self.ch == '.':
                num += self.ch
                self.advance()
            
        self.add_token(TokenType.NUMBER, num)
        
    def peek(self):
        if self.is_at_end(self.pos + 1):
            return ''
        
        return self.code[self.pos + 1]

    def add_token(self, ty: TokenType, val: str):
        self.tokens.append(Token(ty, val))

    def is_at_end(self, pos):
        if pos >= len(self.code):
            return True
        else:
            return False

        
def main():
    tokens = Lexer("122.123 + 123").tokenize()
    print(tokens)

if __name__ == "__main__":
    main()