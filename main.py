from enum import Enum, auto
from dataclasses import dataclass
from pprint import pprint

# ------------ Tokens

class TokenType(Enum):
    NUMBER      = auto() # 1..9
    PLUS     = auto() # +
    MINUS    = auto() # -
    ASTERISK = auto() # *
    SLASH    = auto() # /
    LPAREN   = auto() # (
    RPAREN   = auto() # )
    EOF      = auto() # EOF
    PROGRAM  = auto() # PROGRAM

class Token():
    def __init__(self, ty: TokenType, val: str, line: int, col: int):
        self.ty   = ty
        self.val  = val
        self.line = line
        self.col  = col

    def __repr__(self):
        return f"{self.ty}({self.val})"
    

def tok_in_char(ty):
    if   ty == TokenType.NUMBER:    return 'number'
    elif ty == TokenType.PLUS:      return '+'
    elif ty == TokenType.MINUS:     return '-'
    elif ty == TokenType.ASTERISK:  return '*'
    elif ty == TokenType.SLASH:     return '/'
    elif ty == TokenType.LPAREN:    return '('
    elif ty == TokenType.RPAREN:    return ')'
    elif ty == TokenType.EOF:       return 'eof'
    elif ty == TokenType.PROGRAM:   return 'program'
    else:                           return f"unkown '{ty}'"


# ------------ Lexers

class Lexer():
    def __init__(self, code: str):
        self.code   = code
        self.line   = 1
        self.col    = 0
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
        elif self.is_int(self.ch):
            self.parse_number()
        else:
            raise SyntaxError(f"Unexpected characters '{self.ch}' at {self.line}:{self.col}")

    def skip_whitespace(self):
        while self.ch in [' ', '\n', '\t', '\r']:
            self.advance()

    def peek(self):
        if self.pos >= len(self.code):
            return ''
        return self.code[self.pos]

    def advance(self):
        # print(f"{len(self.code)}, {self.pos}")
        if self.ch == '\n':
            self.line += 1
            self.col   = 1
        else:
            self.col += 1
            
        if self.pos >= len(self.code):
            self.ch = ''
        else:
            self.ch = self.code[self.pos]
        self.pos += 1
        # self.ch = self.code[self.pos] if not self.is_at_end(self.pos) else ''
        # self.pos += 1

    def parse_number(self):
        num = ''
        while self.is_int(self.ch):
            # if self.peek() == '.' and '.' not in num:
            #     num += self.ch
            #     self.advance()
            num += self.ch
            self.advance()

        if self._is('.') and self.is_int(self.peek()):
            num += self.ch
            self.advance()
            while self.is_int(self.ch):
                num += self.ch
                self.advance()

            
        self.add_token(TokenType.NUMBER, num)
    
    def _is(self, ch: str):
        return self.ch == ch

    def is_int(self, ch):
        return '0' <= ch <= '9'
    
    def add_token(self, ty: TokenType, val: str):
        self.tokens.append(Token(ty, val, self.line, self.col))

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
class NoOp(Node):
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
class Number(Node):
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

    def consume(self, ty):
        if self.ct and self.ct.ty == ty:
            self.advance()
        else:
            self.error(ty)

    def error(self, expect=None):
        if not expect:
            expect_str = "unknown"
        elif isinstance(expect, list):
            expect_str = "' or '".join(tok_in_char(e) for e in expect)
        else:
            expect_str = tok_in_char(expect)
        
        if self.ct is None or self.ct.ty == TokenType.EOF:
            raise ParseError(f"Unexpected End of File. expected '{expect_str}' at {self.ct.line}:{self.ct.col}")
        raise ParseError(f"Unexpected token '{self.ct.val}'. expected '{expect_str}' at {self.ct.line}:{self.ct.col}")
    def parse_program(self):
        self.consume(TokenType.PROGRAM)

        # woops, ternyata di sini tempat yang lebih baik
        if self.ct.ty == TokenType.EOF:
            return Program(NoOp())
        
        # print(self.ct)
        return Program(self.parse_expr())

    def parse_expr(self):
        node = self.parse_term()

        while self.ct.ty in [TokenType.PLUS, TokenType.MINUS]:
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


        while self.ct.ty in [TokenType.ASTERISK, TokenType.SLASH]:
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
                  | NUMBER
                  | LPAREN expr RPAREN
        """

        tok = self.ct
        # print(tok)
        
        # handle kasus spesial kalau ada file kosong (hanya berisi PROGRAM dan EOF)
        # if self.tokens[0].ty == TokenType.PROGRAM and self.tokens[1].ty == TokenType.EOF:
        #     return Program(Node)

        if tok.ty == TokenType.PLUS:
            self.consume(TokenType.PLUS)
            # print("PLUS")
            node = UnaryOp(tok, self.parse_factor())
            return node
        elif tok.ty == TokenType.MINUS:
            self.consume(TokenType.MINUS)
            node = UnaryOp(tok, self.parse_factor())
            return node
        elif tok.ty == TokenType.NUMBER:
            # print("NUMBER")
            self.consume(TokenType.NUMBER)
            return Number(tok)
        elif tok.ty == TokenType.LPAREN:
            self.consume(TokenType.LPAREN)
            node = self.parse_expr()
            self.consume(TokenType.RPAREN)
            return node
        else:
            self.error([TokenType.PLUS, TokenType.MINUS, TokenType.NUMBER, TokenType.LPAREN])
    def parse(self):
        node = self.parse_program()
        # print(node)
        # if not self.is_at_end():
        #     assert True, "unreachable"
        return node

    def is_at_end(self):
        return self.ct is not None and self.ct.ty == TokenType.EOF

# ------------ Interpeter

class InterpreterError(Exception):
    pass

class Interpreter:
    def visit(self, node):
        method_name = f"visit_{type(node).__name__}"
        method      = getattr(self, method_name, self.generic_visit)
        return method(node)
    
    def generic_visit(self, node):
        raise InterpreterError(f"No visit_{type(node).__name__} method")
    
    def interpret(self, node: Program):
        return self.visit(node)

    def visit_Program(self, node: Program):
        return self.visit(node.block)
    
    def visit_NoOp(self, node: NoOp):
        return None

    def visit_BinOp(self, node: BinOp):
        left  = self.visit(node.left)
        right = self.visit(node.right)

        if node.op.ty == TokenType.PLUS:
            return left + right
        elif node.op.ty == TokenType.MINUS:
            return left - right
        elif node.op.ty == TokenType.ASTERISK:
            return left * right
        elif node.op.ty == TokenType.SLASH:
            if right == 0.0:
                raise InterpreterError(f"Division by zero at {node.op.line}:{node.op.col}")
            return left / right
        else:
            raise InterpreterError(f"Unkown binary operator '{node.op.val}' at {node.op.line}:{node.op.col}")

    def visit_UnaryOp(self, node: UnaryOp):
        val = self.visit(node.expr)

        if node.op.ty == TokenType.PLUS:
            return +val
        elif node.op.ty == TokenType.MINUS:
            return -val
        else:
            raise InterpreterError(f"Unkown unary operator '{node.op.val}' at {node.op.line}:{node.op.col}")
        
    def visit_Number(self, node: Number):
        val = float(node.ty.val)
        return int(node.ty.val) if val.is_integer() else val

# ------------ Mains

def main():
    tokens = Lexer("4").tokenize()
    ast    = Parser(tokens).parse()
    # pprint(tokens)
    # pprint(ast)
    result = Interpreter().interpret(ast)
    print(result)

if __name__ == "__main__":
    main()