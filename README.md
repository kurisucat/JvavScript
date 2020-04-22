# JvavScript
A powerful, high performance and fully Turing-complete language.

## Installion
```shell
git clone https://github.com/JVAV-Lang/JvavScript.git 
cd JvavScript 
cargo build --release 
```
**You can downlaod the Interpreter from the release page.**

## Language Design
JvavScript consists of 8 commands, listed below.
|Character|Meaning|
| --- | --- |
| >	| increment the data pointer (to point to the next cell to the right).|
| <	| decrement the data pointer (to point to the next cell to the left). |
| + | increment (increase by one) the byte at the data pointer. |
| - | decrement (decrease by one) the byte at the data pointer. |
| . | output the byte at the data pointer. |
| , | accept one byte of input, storing its value in the byte at the data pointer.|
| [ | if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.|
| ] | if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.|

(Alternatively, the ] command may instead be translated as an unconditional jump to the corresponding [ command, or vice versa; programs will behave the same but will run more slowly, due to unnecessary double searching.)

[ and ] match as parentheses usually do: each [ matches exactly one ] and vice versa, the [ comes first, and there can be no unmatched [ or ] between the two.

## Example 
**Some examples were written by daniel b cristofani.**
### Adding two values
The JvavScript is very easy to learn.

As a first, simple example, the following code snippet will add the current cell's value to the next cell: Each time the loop is executed, the current cell is decremented, the data pointer moves to the right, that next cell is incremented, and the data pointer moves left again. This sequence is repeated until the starting cell is 0.

```brainfuck
[->+<]
```

This can be incorporated into a simple addition program as follows:

```brainfuck
++       Cell c0 = 2
> +++++  Cell c1 = 5

[        Start your loops with your cell pointer on the loop counter (c1 in our case)
< +      Add 1 to c0
> -      Subtract 1 from c1
]        End your loops with the cell pointer on the loop counter

At this point our program has added 5 to 2 leaving 7 in c0 and 0 in c1
but we cannot output this value to the terminal since it is not ASCII encoded!

To display the ASCII character "7" we must add 48 to the value 7
48 = 6 * 8 so let's use another loop to help us!

++++ ++++  c1 = 8 and this will be our loop counter again
[
< +++ +++  Add 6 to c0
> -        Subtract 1 from c1
]
< .        Print out c0 which has the value 55 which translates to "7"!
```

### Hello World
```
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

### Factorial 
**This program doesn't terminate; you will have to kill it.**
```
>++++++++++>>>+>+[>>>+[-[<<<<<[+<<<<<]>>[[-]>[<<+>+>-]<[>+<-]<[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>[-]>>>>+>+<<<<<<-[>+<-]]]]]]]]]]]>[<+>-]+>>>>>]<<<<<[<<<<<]>>>>>>>[>>>>>]++[-<<<<<]>>>>>>-]+>>>>>]<[>++<-]<<<<[<[>+<-]<<<<]>>[->[-]++++++[<++++++++>-]>>>>]<<<<<[<[>+>+<<-]>.<<<<<]>.>>>>]
```

### Random
```
>>>++[
    <++++++++[
        <[<++>-]>>[>>]+>>+[
            -[->>+<<<[<[<<]<+>]>[>[>>]]]
            <[>>[-]]>[>[-<<]>[<+<]]+<<
        ]<[>+<-]>>-
    ]<.[-]>>
]
```
- "Random" byte generator using the Rule 30 automaton.
- Doesn't terminate; you will have to kill it.
- To get x bytes you need 32x+4 cells.
- Turn off any newline translation!

### Insertion sort
```
>>+>,[
    <[
        [>>+<<-]>[<<+<[->>+[<]]>>>[>]<<-]<<<
    ]>>[<<+>>-]<[>+<-]>[>>]<,
]<<<[<+<]>[>.>]
```
This program sorts bytes of input using insertion sort.

## License
MIT

