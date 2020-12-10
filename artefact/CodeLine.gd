extends HBoxContainer

# store the last computed size
var size = 0

onready var address = $Address
onready var data0 = $Data/Data0
onready var data1 = $Data/Data1
onready var data2 = $Data/Data2

# return the number of bytes consumed by the disassembler
func update_from_buffer(data: PoolIntArray, offset: int, addr: int) -> int:
	address.value = addr
	data0.value = data[offset]
	data1.visible = false
	data2.visible = false
	size = 1
	return 1
