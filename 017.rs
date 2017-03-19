/*

Project Euler Problem 17:

If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
Note: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

*/

fn main() {
    let _1 = 3;
    let _2 = 3;
    let _3 = 5;
    let _4 = 4;
    let _5 = 4;
    let _6 = 3;
    let _7 = 5;
    let _8 = 5;
    let _9 = 4;
    let _10 = 3;
    let _11 = 6;
    let _12 = 6;
    let _13 = 8;
    let _14 = 8;
    let _15 = 7;
    let _16 = 7;
    let _17 = 9;
    let _18 = 8;
    let _19 = 8;
    let _20 = 6;
    let _30 = 6;
    let _40 = 5;
    let _50 = 5;
    let _60 = 5;
    let _70 = 7;
    let _80 = 6;
    let _90 = 6;
    let _x00 = 7;
    let _x000 = 8;
    let _and = 3;

    let _1_9 = _1 + _2 + _3 + _4 + _5 + _6 + _7 + _8 + _9;

    let _10_19 = _10 + _11 + _12 + _13 + _14 + _15 + _16 + _17 + _18 + _19;

    let _1_99 = _1_9*9 + _10_19 + (_20 + _30 + _40 + _50 + _60 + _70 + _80 + _90)*10;

    let _x00_x99 = _x00*100 + _and*99 + _1_99;

    let _1_1000 = _1_99 + _1_9*100 + _x00_x99*9 + _1 + _x000;

    println!("{:}", _1_1000);

}
