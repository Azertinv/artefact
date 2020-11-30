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
        return "Op({}, size: {}, args: {})".format(
                self.encoding, self.size, self.args_num)

opcodes = {
        "nop":      Op("000000000", 1, 0),
        # 1 reg logic
        "not":      Op("0000010xx", 1, 1),
        # 1 reg stack
        "push":     Op("000001Txx", 1, 1),
        "pop":      Op("0000011xx", 1, 1),
        # 2 reg arithmetics
        "radd":     Op("01TTTyyxx", 1, 2),
        "rsub":     Op("01TT1yyxx", 1, 2),
        "rmul":     Op("01TT0yyxx", 1, 2),
        "rdiv":     Op("01T1Tyyxx", 1, 2),
        "rmod":     Op("01T11yyxx", 1, 2),
        "raddfz":   Op("01T10yyxx", 1, 2),
        "rsubfz":   Op("01T0Tyyxx", 1, 2),
        # 2 reg logic
        "rand":     Op("011TTyyxx", 1, 2),
        "ror":      Op("011T1yyxx", 1, 2),
        "rxor":     Op("011T0yyxx", 1, 2),
        # 2 reg utils
        "rmov":     Op("01100yyxx", 1, 2),
        # 1 reg 1 imm arithmetics
        "iadd":     Op("TTTvvvvxx", 1, 2),
        "isub":     Op("TT1vvvvxx", 1, 2),
        "imul":     Op("TT0vvvvxx", 1, 2),
        "idiv":     Op("T1Tvvvvxx", 1, 2),
        "imod":     Op("T11vvvvxx", 1, 2),
        "iaddfz":   Op("T10vvvvxx", 1, 2),
        "isubfz":   Op("T0Tvvvvxx", 1, 2),
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

def dec_to_bt_helper(dec):
    if dec == 0: return ""
    if (dec % 3) == 0: return "0" + dec_to_bt_helper(dec // 3)
    if (dec % 3) == 1: return "1" + dec_to_bt_helper(dec // 3)
    if (dec % 3) == 2: return "T" + dec_to_bt_helper((dec + 1) // 3)

def dec_to_bt(dec):
    result = dec_to_bt_helper(dec)
    result = result.ljust(18, '0')
    return result

def get_inimm(opcode):
    imm = opcode.args.pop(0)
    if imm[0] == "D": # decimal value
        try:
            value = int(imm[1:])
            imm = dec_to_bt(value)
        except ValueError:
            raise CompileError(opcode.line,
                    "Invalid immediate decimal value \"{}\" given at line {}"
                    .format(imm, opcode.line))
    return imm[:4].ljust(4, '0')

def compile_final(parsed_code, labels):
    result = []
    for op in parsed_code:
        encoding = op.encoding
        if "xx" in encoding:
            encoding = encoding.replace("xx", get_reg(op))
        if "yy" in encoding:
            encoding = encoding.replace("yy", get_reg(op))
        if "vvvv" in encoding:
            encoding = encoding.replace("vvvv", get_inimm(op))
        result.append(encoding)
    return result

def compile(code):
    lexed_code = lexer(code)
    parsed_code, labels = parse(lexed_code)
    print(labels)
    print(parsed_code)
    compiled_code = compile_final(parsed_code, labels)
    return compiled_code

def test():
    expected_compiled_code = [
            '000000000','000001000','000001T0T',
            '00000110T','01TTT0100','01TT10100',
            '01TT00100','01T1T0100','01T110100',
            '01T100100','01T0T0100','011TT0100',
            '011T10100','011T00100','011000100',
            'TTT01T00T','TT11T0000','TT01T1000',
            'T1T1T1T00','T11100000','T10100000',
            'T0T100000']
    test_code = """
    main:
        nop
        not b
        push a
        pop a
    wazzaaaaa:
        radd b c
        rsub b c
        rmul b c
        rdiv b c
        rmod b c
        raddfz b c
        rsubfz b c
        rand b c
        ror b c
        rxor b c
        rmov b c
    label:
        iadd a 01T
        isub b D-2
        imul b D7
        idiv b 1T1T
        imod b 1
        iaddfz b 1
        isubfz b 1
    """
    compiled_code = compile(test_code)
    if compiled_code != expected_compiled_code:
        print(compiled_code)
        print(expected_compiled_code)
        print("Test failed: compiled code differ from expected compiled code")
        embed()
        # raise Exception("Test failed: compiled code differ from expected compiled code")

def main():
    with open("test.asm", 'r') as f:
        compiled_code = compile(f.read())
        print(compiled_code)

from IPython import embed

if __name__ == "__main__":
    # main()
    test()
