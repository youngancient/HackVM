
// --- Start of BasicTest ---

// push constant 10

@10
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop local 0

@0
D=A
@LCL
D=D+M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

// push constant 21

@21
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 22

@22
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop argument 2

@2
D=A
@ARG
D=D+M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

// pop argument 1

@1
D=A
@ARG
D=D+M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

// push constant 36

@36
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop this 6

@6
D=A
@THIS
D=D+M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

// push constant 42

@42
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 45

@45
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop that 5

@5
D=A
@THAT
D=D+M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

// pop that 2

@2
D=A
@THAT
D=D+M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

// push constant 510

@510
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop temp 6

@SP
M=M-1
@SP
A=M
D=M
@11
M=D

// push local 0

@LCL
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// push that 5

@THAT
D=M
@5
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// add

@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1

// push argument 1

@ARG
D=M
@1
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// sub

@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D-M
@SP
M=M+1

// push this 6

@THIS
D=M
@6
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// push this 6

@THIS
D=M
@6
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// add

@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1

// sub

@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D-M
@SP
M=M+1

// push temp 6

@11
D=A
@SP
A=M
M=D
@SP
M=M+1

// add

@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1


// --- End of BasicTest ---


// --- Start of PointerTest ---

// push constant 3030

@3030
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop pointer 0

@SP
M=M-1
@SP
A=M
D=M
@THIS
M=D

// push constant 3040

@3040
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop pointer 1

@SP
M=M-1
@SP
A=M
D=M
@THAT
M=D

// push constant 32

@32
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop this 2

@2
D=A
@THIS
D=D+M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

// push constant 46

@46
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop that 6

@6
D=A
@THAT
D=D+M
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D

// push pointer 0

@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1

// push pointer 1

@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1

// add

@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1

// push this 2

@THIS
D=M
@2
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// sub

@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D-M
@SP
M=M+1

// push that 6

@THAT
D=M
@6
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// add

@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1


// --- End of PointerTest ---