from enum import Enum, auto
from dataclasses import dataclass
from pprint import pprint

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
    PROGRAM  = auto() # PROGRAM

class Token():
    def __init__(self, ty: TokenType, val: str):
        self.ty  = ty
        self.val = val

    def __repr__(self):
        return f"{self.ty}({self.val})"

# ------------ Lexers

class Lexer():
    def __init__(self, code: str):
        self.code   = code
        self.pos    = 0
        self.ch     = ''
        self.tokens = []
        self.advance()

        
    def tokenize(self):
        self.add_token(TokenType.PROGRAM, "PROGRAM")

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

@dataclass
class Node:
    pass

@dataclass
class Program(Node):
    block: Node

@dataclass
class BinOp(Node):
    left: Node
    op: Token
    right: Node

@dataclass
class UnaryOp(Node):
    op: Token
    expr: Node

@dataclass
class Int(Node):
    ty: Token

    # @property
    # def val(self):
    #     return self.ty.val

# ------------ Parsers

class ParseError(Exception):
    pass

class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos    = 0
        self.ct     = None
        self.advance()

        # self.advance(TokenType.PROGRAM)
        
        # print(self.ct)

    def advance(self):
        if not self.is_at_end():
            self.ct = self.tokens[self.pos]
            self.pos += 1
        else:
            self.ct = None
    def consume(self, ty):
        if self.ct and self.ct.ty == ty:
            self.advance()
        else:
            self.error()

    def error(self):
        raise ParseError(f"Unexpected token: '{self.ct}")

    def parse_program(self):
        self.consume(TokenType.PROGRAM)
        # print(self.ct)
        return Program(self.parse_expr())

    def parse_expr(self):
        node = self.parse_term()

        while self.ct and self.ct.ty in [TokenType.PLUS, TokenType.MINUS]:
            tok = self.ct
            if tok.ty == TokenType.PLUS:
                self.consume(TokenType.PLUS)
            elif tok.ty == TokenType.MINUS:
                self.consume(TokenType.MINUS)

            node = BinOp(node, tok, self.parse_term())
        return node

    def parse_term(self):
        """term : ((ASTERISK | SLASH) factor)*
        """
        node = self.parse_factor()
        # print(node)


        while self.ct and self.ct.ty in [TokenType.ASTERISK, TokenType.SLASH]:
            tok = self.ct
            if tok.ty == TokenType.ASTERISK:
                self.consume(TokenType.ASTERISK)
            elif tok.ty == TokenType.SLASH:
                self.consume(TokenType.SLASH)

            node = BinOp(node, tok, self.parse_factor())
        return node

    def parse_factor(self):
        """factor : PLUS factor
                  | MINUS factor
                  | INT
                  | LPAREN expr RPAREN
        """

        tok = self.ct
        # print(tok)
        if tok.ty == TokenType.PLUS:
            self.consume(TokenType.PLUS)
            # print("PLUS")
            node = UnaryOp(tok, self.parse_factor())
            return node
        elif tok.ty == TokenType.MINUS:
            self.consume(TokenType.MINUS)
            node = UnaryOp(tok, self.parse_factor())
            return node
        elif tok.ty == TokenType.INT:
            # print("INT")
            self.consume(TokenType.INT)
            return Int(tok)
        elif tok.ty == TokenType.LPAREN:
            self.consume(TokenType.LPAREN)
            node = self.parse_expr()
            self.consume(TokenType.RPAREN)
            return node
        else:
            self.error()
    def parse(self):
        node = self.parse_program()
        # print(node)
        # if not self.is_at_end():
        #     assert True, "unreachable"
        return node

    def is_at_end(self):
        return self.tokens[self.pos].ty == TokenType.EOF

def main():
    tokens = Lexer("1+1").tokenize()
    ast    = Parser(tokens).parse()
    pprint(tokens)
    pprint(ast)

if __name__ == "__main__":
    main()