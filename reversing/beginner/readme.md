# beginner

_reversing, easy_

## initial review

We're provided a single binary called a.out

```text
❯ ./a.out
Flag: hello
FAILURE
```

Running it shows a flag prompt and it says "FAILURE" when I type hello.  

```c
ulong main(void)

{
  int iVar1;
  uint uVar2;
  undefined auVar3 [16];
  undefined local_38 [16];
  char local_28 [4];
  
  printf("Flag: ");
  __isoc99_scanf(&DAT_0010200b,local_38);
  auVar3 = pshufb(local_38,SHUFFLE);
  local_28 = SUB164(auVar3,0) + ADD32._0_4_ ^ SUB164(XOR,0);
  iVar1 = strncmp(local_38,local_28,0x10);
  if (iVar1 == 0) {
    uVar2 = strncmp(local_28,EXPECTED_PREFIX,4);
    if (uVar2 == 0) {
      puts("SUCCESS");
      goto LAB_00101112;
    }
  }
  uVar2 = 1;
  puts("FAILURE");
LAB_00101112:
  return (ulong)uVar2;
}
```

The program reads 15 chars into memory and then applies 3 SIMD operations.  If the input and modified strings are the same and the modified string starts with CTF{ then it says SUCCESS.  It looks like our goal is to construct a 15 char string matching CTF{.\*} that can have those operations applied to it without changing it.  The right operand of each instruction is a constant 16 byte array and they are below.  
xor: `76 58 b4 49 8d 1a 5f 38 d4 23 f8 34 eb 86 f9 aa`
add32: `ef be ad de ad de e1 fe 37 13 37 13 66 74 63 67`
shuffle: `02 06 07 01 05 0b 09 0e 03 0f 04 08 0a 0c 0d 00`

```python
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
```

This was our solver script.  The careful observer (aka if you executed it) may notice that the flag it gives isn't correct.  The solved flag is `CTF{S1NEf0rM3!}` and the correct flag is `CTF{S1MDf0rM3!}`.  Honestly I have no idea why, we just found that the original flag wasn't correct and guessed the correct numbers from context.  