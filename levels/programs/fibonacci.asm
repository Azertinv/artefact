start:
    setw a D0
    setw b D1
loop:
    radd b a
    radd a b
    jump :loop
