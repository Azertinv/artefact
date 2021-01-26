loop:
    iadd b D1
    icmp a D1
    cjumprel T 1T :end
    rmov c a
    idiv c D2
    imul c D2
    rcmp c a
    cjumprel 0 TT :even
odd:
    imul a D3
    iadd a D1
    jump :loop
even:
    idiv a D2
    jump :loop
end:
    jump :end
