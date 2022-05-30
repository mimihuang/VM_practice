# VM_practice

The purpose of this project is for me to delve deeper into virtual machines and practice to implement virtual machines.

This Virtual Machine, like the JVM, is a stack-based interpreter which is different from a compiler. To explain, an interpreter directly executes the program rather than translating it into another language first and then executing subsequently. For example, C has a compiler generating a .exe file first and then running the file, while Python does not. After we create a Python program, it is executed on the fly. Arguments are first pushed on the stack, the bytecode executed and the result left on the stack.

This virtual machine contains not only a stack but also a heap and eight registers. Moreover, it can deal with singed/unsigned integers and float numbers.

This virtual machine is based on Tarek's small VM with some tweaks. 
One can check their original codes here: https://github.com/tarekwiz/smallvm
