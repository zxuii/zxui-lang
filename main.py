from enum import Enum, auto
from dataclasses import dataclass

# ------------ Debugging

LOG = False

# ------------ logger

def logger(msg='', newline=True):
    if LOG:
        if newline:
            print("[LOGGER]",msg)
        else:
            print("[LOGGER]",msg, end='')

# ------------ Tokens

class TokenType(Enum):
    IDENTIFIER = auto() # IDENTIFIER
    NUMBER     = auto() # NUMBER
    
    LET        = auto() # let
    FUN        = auto() # fun
    RETURN     = auto() # return
    
    PLUS       = auto() # +
    MINUS      = auto() # -
    ASTERISK   = auto() # *
    SLASH      = auto() # /
    
    SEMICOLON  = auto() # ;
    LPAREN     = auto() # (
    RPAREN     = auto() # )
    LBRACE     = auto() # {
    RBRACE     = auto() # }
    COMMA      = auto() # ,
    EQUAL      = auto() # =

    EOF        = auto() # EOF
    PROGRAM    = auto() # PROGRAM

class Token():
    def __init__(self, ty: TokenType, val: str, line: int, col: int):
        self.ty   = ty
        self.val  = val
        self.line = line
        self.col  = col

    def __repr__(self):
        return f"{self.ty}({self.val})"
        
KEYWORDS = {
    "let": TokenType.LET,
    "fun": TokenType.FUN,
    "return": TokenType.RETURN
}

def tok_in_char(ty):
    if   ty == TokenType.IDENTIFIER: return 'identifier' 
    elif ty == TokenType.NUMBER:     return 'number'
    elif ty == TokenType.LET:        return 'let'
    elif ty == TokenType.FUN:        return 'fun'
    elif ty == TokenType.RETURN:     return 'return'
    elif ty == TokenType.PLUS:       return '+'
    elif ty == TokenType.MINUS:      return '-'
    elif ty == TokenType.ASTERISK:   return '*'
    elif ty == TokenType.SLASH:      return '/'
    elif ty == TokenType.SEMICOLON:  return ';'
    elif ty == TokenType.LPAREN:     return '('
    elif ty == TokenType.RPAREN:     return ')'
    elif ty == TokenType.LBRACE:     return '{'
    elif ty == TokenType.RBRACE:     return '}'
    elif ty == TokenType.COMMA:      return ','
    elif ty == TokenType.EQUAL:      return '='
    elif ty == TokenType.EOF:        return 'eof'
    elif ty == TokenType.PROGRAM:    return 'program'
    else:                            return f"unkown '{ty}'"


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
        elif self._is(';'):
            self.add_token_advance(TokenType.SEMICOLON, ';')
        elif self._is('('):
            self.add_token_advance(TokenType.LPAREN, '(')
        elif self._is(')'):
            self.add_token_advance(TokenType.RPAREN, ')')
        elif self._is('{'):
            self.add_token_advance(TokenType.LBRACE, '{')
        elif self._is('}'):
            self.add_token_advance(TokenType.RBRACE, '}')
        elif self._is(','):
            self.add_token_advance(TokenType.COMMA, ',')
        elif self._is('='):
            self.add_token_advance(TokenType.EQUAL, '=')
        elif self.is_int(self.ch):
            self.parse_number()
        elif self.is_alpha():
            self.parse_ident_or_key()
        
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
    
    def is_key(self, keyword: str):
        key = ''
        for i in keyword:
            if self.ch == i:
                key += self.ch
                self.advance()
        return True if key == keyword else False                

    def is_alpha(self):
        return 'a' <= self.ch <= 'z' or 'A' <= self.ch <= 'Z' or self._is('_')

    def is_alnum(self):
        return self.is_alpha() or self.is_int(self.ch)

    def get_ident(self):
        ident = ''
        while self.is_alnum():
            ident += self.ch
            self.advance()
        return ident

    def parse_ident_or_key(self):
        ident = self.get_ident()
        ty = KEYWORDS.get(ident, TokenType.IDENTIFIER)
        self.add_token(ty, ident)

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
class Block(Node):
    stmts: list

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

@dataclass
class FunDecl(Node):
    fun: Node
    params: list
    block: Node

@dataclass
class FunCall(Node):
    fun: Node
    args: list

@dataclass
class Fun(Node):
    ty: Token

@dataclass
class Return(Node):
    expr: Node

@dataclass
class VarDecl(Node):
    var: Node
    expr: Node

@dataclass
class VarAssign(Node):
    var: Node
    expr: Node

@dataclass
class Var(Node):
    ty: Token


# senagja taurh disini biar serasi aja walaupun ini tempatnya untuk ASt hahahaha
@dataclass
class Function:
    decl: FunDecl
    closure: Environment

@dataclass
class NativeFunction:
    name: str
    arity: int
    fn: object

# ------------ Parsers

class ParseError(Exception):
    pass

class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos    = 0
        self.ct     = None
        self.advance()

        self.inside_fun = False

        # self.advance(TokenType.PROGRAM)
        
        # print(self.ct)

    def advance(self):
        if not self.is_at_end():
            self.ct = self.tokens[self.pos]
            self.pos += 1
    
    def peek(self):
        if not self.is_at_end():
            return self.tokens[self.pos]

    def consume(self, ty):
        if self.ct and self.ct.ty == ty:
            self.advance()
        else:
            self.error(expect=ty)

    def error(self, msg=None, expect=None):
        if not expect:
            expect_str = "unknown"
        elif isinstance(expect, list):
            expect_str = "' or '".join(tok_in_char(e) for e in expect)
        else:
            expect_str = tok_in_char(expect)
        
        if self.ct is None or msg is None or self.ct.ty == TokenType.EOF:
            raise ParseError(f"Unexpected End of File. expected '{expect_str}' at {self.ct.line}:{self.ct.col}")
        if msg:
            raise ParseError(msg + f" at {self.ct.line}:{self.ct.col}")
        raise ParseError(f"Unexpected token '{self.ct.val}'. expected '{expect_str}' at {self.ct.line}:{self.ct.col}")
    def parse_program(self):
        self.consume(TokenType.PROGRAM)

        # woops, ternyata di sini tempat yang lebih baik
        if self.ct.ty == TokenType.EOF:
            return Program(NoOp())
        
        # print(self.ct)
        return Program(self.parse_block())
    
    def parse_block(self):
        # print(self.ct)
        stmts = []

        while self.ct.ty not in [TokenType.EOF, TokenType.RBRACE]:
            stmts.append(self.parse_stmt())
            
            if self.ct.ty == TokenType.SEMICOLON:
                self.consume(TokenType.SEMICOLON)

        return Block(stmts)

    def parse_stmt(self):
        if self.ct.ty == TokenType.FUN:
            return self.parse_fun_decl()
        elif self.ct.ty == TokenType.LET:
            return self.parse_var_decl()
        elif self.ct.ty == TokenType.LBRACE:
            self.consume(TokenType.LBRACE)
            node = self.parse_block()
            self.consume(TokenType.RBRACE)
            return node
        elif self.ct.ty == TokenType.IDENTIFIER:
            next_tok = self.peek()
            if next_tok.ty == TokenType.EQUAL:
                return self.parse_var_assign()
            else:
                return self.parse_expr()
        elif self.ct.ty == TokenType.RETURN:
            if self.inside_fun:
                return self.parse_return()
            else:
                self.error("Return statement must be inside some function.")
        else:
            return self.parse_expr()
    
    def parse_fun_decl(self):
        self.consume(TokenType.FUN)
        fun = Fun(self.ct)
        self.consume(TokenType.IDENTIFIER)
        self.consume(TokenType.LPAREN)
        fun_params = self.parse_params()
        self.consume(TokenType.RPAREN)
        self.consume(TokenType.LBRACE)
        self.inside_fun = True
        fun_block = self.parse_block()
        self.inside_fun = False
        self.consume(TokenType.RBRACE)
        return FunDecl(fun, fun_params, fun_block)
    
    def parse_fun_call(self):
        fun = Fun(self.ct)
        self.consume(TokenType.IDENTIFIER)
        self.consume(TokenType.LPAREN)
        fun_args = self.parse_args()
        self.consume(TokenType.RPAREN)
        return FunCall(fun, fun_args)

    def parse_args(self):
        fun_args = []
        if self.ct.ty != TokenType.RPAREN:
            fun_args.append(self.parse_expr())
            while self.ct.ty == TokenType.COMMA:
                self.consume(TokenType.COMMA)
                if self.ct.ty != TokenType.RPAREN:
                    fun_args.append(self.parse_expr())

        return fun_args

    def parse_params(self):
        fun_params = []
        if self.ct.ty == TokenType.IDENTIFIER:
                fun_params.append(self.ct)
                self.consume(TokenType.IDENTIFIER)
            
        while self.ct.ty == TokenType.COMMA:
            self.consume(TokenType.COMMA)
            if self.ct.ty == TokenType.IDENTIFIER:
                fun_params.append(self.ct)
                self.consume(TokenType.IDENTIFIER)
        return fun_params

    def parse_return(self):
        self.consume(TokenType.RETURN)

        while self.ct in [TokenType.SEMICOLON, TokenType.RBRACE, TokenType.EOF]:
            return Return(NoOp())
        
        return Return(self.parse_expr())

    def parse_var_decl(self):
        self.consume(TokenType.LET)
        var = Var(self.ct)
        self.consume(TokenType.IDENTIFIER)
        self.consume(TokenType.EQUAL)
        return VarDecl(var, self.parse_expr())
    
    def parse_var_assign(self):
        var = Var(self.ct)
        self.consume(TokenType.IDENTIFIER)
        self.consume(TokenType.EQUAL)
        return VarAssign(var, self.parse_expr())

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
                  | fun_call or IDENTIFIER
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
        elif tok.ty == TokenType.IDENTIFIER:
            next_tok = self.peek()
            if next_tok.ty == TokenType.LPAREN:
                return self.parse_fun_call()
            
            self.consume(TokenType.IDENTIFIER)
            return Var(tok)
        else:
            self.error([TokenType.PLUS, TokenType.MINUS, TokenType.NUMBER, TokenType.LPAREN, TokenType.LBRACE, TokenType.FUN])
    def parse(self):
        node = self.parse_program()
        # print(node)
        # if not self.is_at_end():
        #     assert True, "unreachable"
        return node

    def is_at_end(self):
        return self.ct is not None and self.ct.ty == TokenType.EOF

# ------------ Environment

class Environment:
    def __init__(self, enclosing=None):
        self.enclosing = enclosing
        self.values = {}

    def define(self, name, value):
        logger(f"ENV: added {name} with value {value}")
        self.values[name] = value

    def get(self, name):
        if name in self.values:
            logger(f"ENV: get the {name} with value {self.values[name]}")
            return self.values[name]
        if self.enclosing is not None:
            logger(f"ENV: get the {name}")
            return self.enclosing.get(name)
        
        raise RuntimeError(f"Undefined variable '{name}' in current scope.")
    
    def assign(self, name, value):
        if name in self.values:
            logger(f"ENV: assigned {name} from {self.values[name]} to ", False)
            self.values[name] = value
            logger(self.values[name])
            return
        if self.enclosing is not None:
            logger(f"ENV: assigned {name}", True)
            self.enclosing.assign(name, value)
            return
        
        raise RuntimeError(f"Undefined variable '{name}' in current scope.")
    
# ------------ Interpeter

class ReturnSignal(Exception):
    def __init__(self, val):
        self.val = val

class Interpreter:
    def __init__(self):
        self.env = Environment()
        self.define_natives()

    def define_natives(self):
        self.env.define("println", NativeFunction("println", -1, self.native_println))

    def native_println(self, args):
        print(*args)
        return 0

    def visit(self, node):
        method_name = f"visit_{type(node).__name__}"
        method      = getattr(self, method_name, self.generic_visit)
        return method(node)
    
    def generic_visit(self, node):
        raise RuntimeError(f"No visit_{type(node).__name__} method")
    
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
                raise RuntimeError(f"Division by zero at {node.op.line}:{node.op.col}")
            return left / right
        else:
            raise RuntimeError(f"Unkown binary operator '{node.op.val}' at {node.op.line}:{node.op.col}")

    def visit_UnaryOp(self, node: UnaryOp):
        val = self.visit(node.expr)

        if node.op.ty == TokenType.PLUS:
            return +val
        elif node.op.ty == TokenType.MINUS:
            return -val
        else:
            raise RuntimeError(f"Unkown unary operator '{node.op.val}' at {node.op.line}:{node.op.col}")
    
    def visit_Number(self, node: Number):
        return float(node.ty.val)
    
    def visit_Block(self, node: Block):
        local_env = Environment(self.env)
        
        prev_env = self.env

        self.env = local_env

        for stmt in node.stmts:
            self.visit(stmt)

        self.env = prev_env

    def visit_VarDecl(self, node: VarDecl):
        self.env.define(node.var.ty.val, self.visit(node.expr))

    def visit_VarAssign(self, node: VarAssign):
        self.env.assign(node.var.ty.val, self.visit(node.expr))

    def visit_Var(self, node: Var):
        return self.env.get(node.ty.val)

    def visit_FunDecl(self, node: FunDecl):
        fun = Function(node, self.env)
        self.env.define(node.fun.ty.val, fun)

    def visit_FunCall(self, node: FunCall):
        fun = self.env.get(node.fun.ty.val)
        args = [self.visit(arg) for arg in node.args]

        if isinstance(fun, Function):

            if len(args) != len(fun.decl.params):
                raise RuntimeError(f"function '{node.fun.ty.val}' need {len(fun.decl.params)} arguments but get {len(args)}")
            
            call_env = Environment(fun.closure)
            for param, arg in zip(fun.decl.params, args):
                self.env.define(param.val, arg)

            prev_env = self.env
            self.env = call_env
            return_val = None
            try:
                self.visit(fun.decl.block)
            except ReturnSignal as r:
                return_val = r.val
            finally:
                self.env = prev_env

            return return_val
        elif isinstance(fun, NativeFunction):
            if fun.arity != -1 and len(args) != fun.arity:
                raise RuntimeError(f"function '{node.fun.ty.val}' need {len(fun.decl.params)} arguments but get {len(args)}")
            return fun.fn(args)
        else:
            raise RuntimeError(f"'{node.fun.ty.val}' is not a function at {node.fun.ty.line}:{node.fun.ty.col}")
    
    def visit_Return(self, node: Return):
        raise ReturnSignal(self.visit(node.expr))

# ------------ Mains

def main():
    # try:
    with open("example.zxui", "r") as file:
        content = file.read()

    tokens = Lexer(content).tokenize()
    logger("TOKENS")
    for i, t in enumerate(tokens):
        logger(f"{i}: {t}")
    ast    = Parser(tokens).parse()
    logger("AST")
    logger(ast)
    result = Interpreter().interpret(ast)
    logger("RESULT")
    logger(result)

if __name__ == "__main__":
    main()