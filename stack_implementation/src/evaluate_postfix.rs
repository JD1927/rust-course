/*  ------------------------------
    Expression Evaluation infix to postfix
    ------------------------------

    Rules: 
        1. If operand then push it to the stack
        2. If operation pop two elements perform operations and then push

    Example: 33, 45, 3, /, 2, 9, +, *, +, 50, -
            Symbol      Stack
    1.        33          33
    2.        45          33, 45
    3.        3           33, 45, 3
    4.        /           33, 15                      op1=45, op2=3 -> op1 / op2 = 15
    5.        2           33, 15, 2,
    6.        9           33, 15, 2, 9
    7.        +           33, 15, 11                  op1=2, op2=9 -> op1 + op2 = 11
    8.        *           33, 165                     op1=15, op2=11 -> op1 * op2 = 165
    9.        +           198                         op1=33, op2=165 -> op1 + op2 = 198
    10.       50          198, 50
    11.       -           148                         op1=198, op2=50 -> op1 - op2 = 148


*/

fn main() {
    println!("Stack implementations!");
}

