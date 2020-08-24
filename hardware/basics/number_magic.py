kittens = '00001010101011111110111101001011111000101101101111001100'
kittens = kittens[::-1]
magic = []
for i in range(0,56):
    magic.append('A')

magic[0:10] = kittens[46:56]
magic[22:42] = kittens[26:46]
magic[10:22] = kittens[14:26]
magic[42:56] = kittens[0:14]


str_magic = ''.join(magic)

memory = []
for x in range(0,8):
    memory.append('A'*7)

memory[0] = str_magic[49:56]
memory[5] = str_magic[42:49]

memory[6] = str_magic[35:42]
memory[2] = str_magic[28:35]

memory[4] = str_magic[21:28]
memory[3] = str_magic[14:21]

memory[7] = str_magic[7:14]
memory[1] = str_magic[0:7]


#print(memory)
memory_str = ''
for i in memory:
    for a in i:
        memory_str += a

#print(memory_str)


indexes = [0,5,2,7,4,1,6,3]

mem_order = '' 
for i in indexes:
    mem_order += chr(int(memory[i][::-1],2))

print(mem_order)


