extends MarginContainer

var max_i = 10000

func _process(delta):
	if delta > 1.0 / 60.0 * 1.01:
		max_i -= 1 + max_i/100
	else:
		max_i += 1 + max_i/100
#	print(max_i*60)
	$Artefact.run(max_i)
