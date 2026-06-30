from enum import Enum, auto
import sys

# ------------ Tokens

class TokenType(Enum):
    INT      = auto() # 1..9
    PLUS     = auto() # +
    MINUS    = auto() # -
    ASTERISK = auto() # *
    SLASH    = auto() # /
    LPAREN   = auto() # (
    RPAREN   = auto() # )
    EOF      = auto() # EOF

class Token():
    def __init__(self, ty: TokenType, val: str):
        self.ty  = ty
        self.val = val

    def __repr__(self):
        return f"{self.val}"

# ------------ Lexers

class Lexer():
    def __init__(self, code: str):
        self.code   = code
        self.pos    = 0
        self.ch     = ''
        self.tokens = []
        self.advance()

        
    def tokenize(self):
        while not self.is_empty():
            self.next_token()

        self.add_token(TokenType.EOF, "EOF")
        return self.tokens

    def next_token(self):
        self.skip_whitespace()

        if self.is_empty():
            return

        # print(f"char saat ini = {self.ch}")
        if self._is('+'):
            self.add_token_advance(TokenType.PLUS, '+')
        elif self._is('-'):
            self.add_token_advance(TokenType.MINUS, '-')
        elif self._is('*'):
            self.add_token_advance(TokenType.ASTERISK, '*')
        elif self._is('/'):
            self.add_token_advance(TokenType.SLASH, '/')
        elif self._is('('):
            self.add_token_advance(TokenType.LPAREN, '(')
        elif self._is(')'):
            self.add_token_advance(TokenType.RPAREN, ')')
        elif self.is_int():
            self.parse_int()
        else:
            raise SyntaxError(f"Unknown characters '{self.ch}'")

    def skip_whitespace(self):
        while self.ch in [' ', '\n', '\t', '\r']:
            self.advance()

    def peek(self):
        if self.pos >= len(self.code):
            return ''
        return self.code[self.pos]

    def advance(self):
        # print(f"{len(self.code)}, {self.pos}")
        if self.pos >= len(self.code):
            self.ch = ''
        else:
            self.ch = self.code[self.pos]
        self.pos += 1
        # self.ch = self.code[self.pos] if not self.is_at_end(self.pos) else ''
        # self.pos += 1

    def parse_int(self):
        num = ''
        while self.is_int():
            num += self.ch
            self.advance()
            
        self.add_token(TokenType.INT, num)
    
    def _is(self, ch: str):
        return self.ch == ch

    def is_int(self):
        return '0' <= self.ch <= '9'
    
    def add_token(self, ty: TokenType, val: str):
        self.tokens.append(Token(ty, val))

    def add_token_advance(self, ty: TokenType, val: str):
        self.add_token(ty, val)
        self.advance()

    def is_empty(self):
        return self._is('')

# ------------ ASTs



def main():
    tokens = Lexer("123 + (512 / 5)").tokenize()
    print(tokens)

if __name__ == "__main__":
    main()