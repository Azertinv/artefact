#!/usr/bin/python3

import sys
import json

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
        "rcmp":     Op("01T00yyxx", 1, 2),
        "rmov":     Op("01100yyxx", 1, 2),
        # 1 reg 1 imm arithmetics
        "iadd":     Op("TTTvvvvxx", 1, 2),
        "isub":     Op("TT1vvvvxx", 1, 2),
        "imul":     Op("TT0vvvvxx", 1, 2),
        "idiv":     Op("T1Tvvvvxx", 1, 2),
        "imod":     Op("T11vvvvxx", 1, 2),
        "iaddfz":   Op("T10vvvvxx", 1, 2),
        "isubfz":   Op("T0Tvvvvxx", 1, 2),
        # 1 reg 1 imm logic
        "shift":    Op("1T0vvvvxx", 1, 2),
        # 2 reg memory op
        "loadw":    Op("00TT1mmxx", 1, 2),
        "loadt":    Op("00TT0mmxx", 1, 2),
        "loadb":    Op("00TTTmmxx", 1, 2),
        "storew":   Op("00T11yymm", 1, 2),
        "storet":   Op("00T10yymm", 1, 2),
        "storeb":   Op("00T1Tyymm", 1, 2),
        # 1 reg 1 imm utils
        "icmp":     Op("T00vvvvxx", 1, 2),
        "setw":     Op("0000T01xx", 3, 2),
        "sett":     Op("0000T00xx", 2, 2),
        "setb":     Op("0000T0Txx", 2, 2),
        # 1 imm control flow
        "callabs":  Op("000000011", 3, 1),
        "callrel":  Op("00000001T", 2, 1),
        }

def parse_inst(head, tail, line, offset):
    if head not in opcodes:
        raise ParsingError(line,
                "Invalid opcode \"{}\" at line {}"
                .format(head, line))
    opcode = deepcopy(opcodes[head])
    opcode.line = line
    opcode.offset = offset
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
            result.append(parse_inst(token, lexed_code, line, offset))
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

def get_mem_reg(opcode):
    reg = opcode.args.pop(0)
    if reg[0] != '[' or reg[-1] != ']':
        raise CompileError(opcode.line,
                "Missing brackets at \"{}\" at line {}"
                .format(reg, opcode.line))
    if reg[1:-1] not in regs:
        raise CompileError(opcode.line,
                "Invalid register \"{}\" given at line {}"
                .format(reg[1:-1], opcode.line))
    return regs[reg[1:-1]]

def dec_to_bt_helper(dec):
    if dec == 0: return ""
    if (dec % 3) == 0: return "0" + dec_to_bt_helper(dec // 3)
    if (dec % 3) == 1: return "1" + dec_to_bt_helper(dec // 3)
    if (dec % 3) == 2: return "T" + dec_to_bt_helper((dec + 1) // 3)

def dec_to_bt(dec):
    result = dec_to_bt_helper(dec)
    result = result.ljust(18, '0')
    return result

def bt_to_dec(bt):
    if len(bt) == 0:
        return 0
    head = bt[0]
    if head == "0":
        result = 0
    elif head == "1":
        result = 1
    elif head == "T":
        result = -1
    return result + 3 * bt_to_dec(bt[1:])

def get_imm(opcode, labels):
    imm = opcode.args.pop(0)
    if imm[0] == "D": # decimal value
        try:
            value = int(imm[1:])
            imm = dec_to_bt(value)
        except ValueError:
            raise CompileError(opcode.line,
                    "Invalid immediate decimal value \"{}\" given at line {}"
                    .format(imm, opcode.line))
    if imm[0] == ":": # relative label offset
        if imm[1:] not in labels:
            raise CompileError(opcode.line,
                    "Label not found \"{}\" at line {}"
                    .format(imm, opcode.line))
        value = labels[imm[1:]].offset- opcode.offset
        if value < -3**9/2 or value > 3**9/2:
            raise CompileError(opcode.line,
                    "Relative label too far \"{}\" at line {}"
                    .format(imm, opcode.line))
        imm = dec_to_bt(value)
    if imm[0] == "@": # absolute label offset
        if imm[1:] not in labels:
            raise CompileError(opcode.line,
                    "Label not found \"{}\" at line {}"
                    .format(imm, opcode.line))
        imm = dec_to_bt(labels[imm[1:]].offset)
    return imm.ljust(18, '0')

def compile_final(parsed_code, labels):
    result = []
    for op in parsed_code:
        encoding = op.encoding
        if "xx" in encoding:
            encoding = encoding.replace("xx", get_reg(op))
        if "mm" in encoding:
            encoding = encoding.replace("mm", get_mem_reg(op))
        if "yy" in encoding:
            encoding = encoding.replace("yy", get_reg(op))
        if "vvvv" in encoding:
            encoding = encoding.replace("vvvv", get_imm(op, labels)[:4])
        result.append(encoding)
        if op.size > 1:
            imm = get_imm(op, labels)
            result.append(imm[0:9])
            if op.size == 3:
                result.append(imm[9:18])
    return result

def compile(code):
    lexed_code = lexer(code)
    parsed_code, labels = parse(lexed_code)
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
            'T0T100000','0000T0100','000001111',
            'TTTT11111','0000T0000','00000TTTT',
            '0000T0T00','000001111',
            '00TT10001','00TT00001','00TTT0001', # load
            '00T110001','00T100001','00T1T0001', # store
            '000000011','110000000','000000000', # callabs
            '00000001T','110T00000', # callrel
            '1T0T00000', # shift
            '01T000000', # rcmp
            'T0001T011', # icmp
            ]
    test_code = """
    main:
        nop
        not b
        push a
        pop a
    wazzaa:
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
        setw b 000001111TTTT11111
        sett b 00000TTTT
        setb b 000001111
        loadw c [b]
        loadt c [b]
        loadb c [b]
        storew [c] b
        storet [c] b
        storeb [c] b
        callabs @wazzaa
        callrel :label
        shift b D-1
        rcmp b b
        icmp f 01T
    """
    compiled_code = compile(test_code)
    if compiled_code != expected_compiled_code:
        print(compiled_code)
        print("Test failed: compiled code differ from expected compiled code")
        embed()
        raise Exception("Test failed: compiled code differ from expected compiled code")
    else:
        print("test passed")

def main():
    with open(sys.argv[1], 'r') as f:
        compiled_code = compile(f.read())
        result = {
            "regs": [],
            "data_chunks": [
                {
                    "memspace": 5,
                    "addr": 0,
                    "data": [bt_to_dec(x) for x in compiled_code]
                }
            ],
            "perm_chunks": [
                {
                    "memspace": 5,
                    "addr": 0,
                    "perm": [0xf]
                }
            ]
        }
        print(json.dumps(result))

if __name__ == "__main__":
    if len(sys.argv) == 2:
        main()
    else:
        test()
