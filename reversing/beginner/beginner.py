from binascii import unhexlify


flag = ["_" for x in range(16)]

known = [0xe, 0xf, 0x0, 0x1, 0x2, 0x3]


flag[0x0] = "C"
flag[0x1] = "T"
flag[0x2] = "F"
flag[0x3] = "{"
flag[0xe] = "}"
flag[0xf] = '\0'

xor = unhexlify("76 58 b4 49 8d 1a 5f 38 d4 23 f8 34 eb 86 f9 aa".replace(" ",""))
add32 = unhexlify("ef be ad de ad de e1 fe 37 13 37 13 66 74 63 67".replace(" ",""))
shuffle = unhexlify("02 06 07 01 05 0b 09 0e 03 0f 04 08 0a 0c 0d 00".replace(" ",""))


def reverse(index):
	char = ord(flag[index])
	for i in range(256):
		if ((i + add32[index]) % 256) ^ xor[index] == char:
			 return chr(i)


while len(known) > 0:
	curr = known.pop(0)
	into = shuffle[curr]
	if flag[into] != "_":
		continue
	solved = reverse(curr)
	flag[into] = solved
	known.append(into)

print("".join(flag))