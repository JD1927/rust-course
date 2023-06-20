/*  ------------------------------
    Expression Evaluation infix to postfix
    ------------------------------

    -> Convert to postfix
    -> Evaluate expression from postfix

    Operands: Constant values, a,b,c
    Operators: *+-/^

    (a + b) = a b +
      infix = postfix

    * Infix to Postfix
        Rules: 
            1. Priorities of operators:
                a. + -
                b. * /
                c. ^
            2. If scanned operator is <> The top operator of the stack in priority then we pop the operators until we have low priority.
            3. If "(" -> push it to the stack
            4. If ")" -> pop elements until "("
            5. If operand then just add it to postfix expression
    
    Example: (33 + 45 / 3 * (2 + 9) - 50)
            Symbol      Stack       Postfix
    1.       (             (
    2.      33             (            33
    3.      +              (+           33
    4.      45             (+           33, 45
    5.      /              (+/          33, 45
    6.      3              (+/          33, 45, 3
    7.      *              (+*          33, 45, 3, /
    8.      (              (+*(         33, 45, 3, /
    9.      2              (+*(         33, 45, 3, /, 2
    10.     +              (+*(+        33, 45, 3, /, 2
    11.     9              (+*(+        33, 45, 3, /, 2, 9
    12.     )              (+*          33, 45, 3, /, 2, 9, +
    13.     -              (-           33, 45, 3, /, 2, 9, +, *, +
    14.     50             (-           33, 45, 3, /, 2, 9, +, *, +, 50
    15.     )                           33, 45, 3, /, 2, 9, +, *, +, 50, -


*/

fn main() {
    println!("Stack implementations!");
}

