#!/usr/bin/python3

from copy import deepcopy

class ParsingError(Exception):
    def __init__(self, line, message):
        self.line = line
        self.message = message
        super().__init__(self.message)

class CompileError(Exception):
    def __init__(self, line, message):
        self.line = line
        self.message = message
        super().__init__(self.message)

def lexer(code):
    result = []
    for l in code.splitlines():
        result.extend(l.split())
        result.append("\n")
    return result

class Label:
    def __init__(self, line, offset):
        self.line = line
        self.offset = offset
    def __repr__(self):
        return "Label(line: {}, offset: {})".format(self.line, self.offset)

def is_label(token):
    return token.endswith(u":")
def is_label_arg(token):
    return token.startswith(u":")

class Op:
    def __init__(self, encoding, size, args_num):
        self.size = size
        self.args_num = args_num
        self.args = []
        self.encoding = encoding
    def __repr__(self):
        return "Op({}, size: {}, args: {})".format(self.encoding, self.size, self.args_num)

opcodes = {
        "nop":  Op("000000000", 1, 0),
        "radd": Op("01TTTyyxx", 1, 2),
        }

def parse_inst(head, tail, line):
    if head not in opcodes:
        raise ParsingError(line,
                "Invalid opcode \"{}\" at line {}"
                .format(head, line))
    opcode = deepcopy(opcodes[head])
    opcode.line = line
    for _ in range(opcode.args_num):
        opcode.args.append(tail.pop(0))
    return opcode

def parse(lexed_code):
    labels = {}
    offset = 0
    line = 1
    result = []
    while len(lexed_code) > 0:
        token = lexed_code.pop(0)
        if is_label(token):
            label = token[:-1]
            if label in labels:
                raise ParsingError(line,
                        "Label {} already exist at line {}"
                        .format(label, labels[label].line))
            labels[token[:-1]] = Label(line, offset)
        elif token == "\n":
            line += 1
        else:
            result.append(parse_inst(token, lexed_code, line))
            offset += result[-1].size
    return (result, labels)

regs = {"pc": "TT", "sp": "T0", "fl": "T1",
        "a":  "0T", "b":  "00", "c":  "01",
        "d":  "1T", "e":  "10", "f":  "11",}
def get_reg(opcode):
    reg = opcode.args.pop(0)
    if reg not in regs:
        raise CompileError(opcode.line,
                "Invalid register \"{}\" given at line {}"
                .format(reg, opcode.line))
    return regs[reg]

def compile_final(parsed_code, labels):
    result = []
    for op in parsed_code:
        encoding = op.encoding
        if "xx" in encoding:
            encoding = encoding.replace("xx", get_reg(op))
        if "yy" in encoding:
            encoding = encoding.replace("yy", get_reg(op))
        result += encoding

def main():
    with open("test.asm", 'r') as f:
        lexed_code = lexer(f.read())
        parsed_code, labels = parse(lexed_code)
        compile_final(parsed_code, labels)
        print(parsed_code)
        embed()

from IPython import embed

if __name__ == "__main__":
    main()
