# March updates

## 2022-03-03

Basically resetting the project. I did my experiments and now I think I'm ready to start working on the code again.

## 2022-03-04

Today I worked all day on the code (of course I took some rest too).  
I'm still working on making the parser, but the base features of the lexer is working.  
I hope I can finish the parser in the next days, still I don't have much time.

## 2022-03-15

- ### part 1

A friend of mine found a bug:  
if we write in input: 1+1, we will get [integer, integer] as tokens.  
if we write in input: 1 + 1 or 1 +1 we will get [integer, plus, integer] as tokens.  
i think that it's intuitive what we want to happen, doesn't matter how many spaces we need to get [integer, plus, integer] as tokens.  
I'm also currently working on the bug, but since I can't always work on it, idk when it'll be fixed.

- ### part 2

The bug is fixed, I'll be working on the parser in the next days.

## 2022-03-16

Note: Everytime I commit something, it is just when I'm in school and lessions are not important.  
Added some comments for clarity, I usually don't so any suggestion is appreciated.
I'll probably start working on the parser, wish me luck.

## 2022-03-17

I found a bug in the lexer, another time. The strings will make both the interpreter and the compiler loop forever.  
I'll be working on it for now, I'll also add an Error struct to the lexer.  
The base for the lexer should be done, I'll now work on adding the actual grammar.

## 2022-03-18

Yikes, late update for today.
I'm switching to a lexer generator like Logos, this will be a lot easier and more efficient.  
I did my experience with hand-written lexers, and I'm pretty sure i learned a lot from it.

Now the time as come, I can finally start working on the parser.

## 2022-03-19

Today I just chilled it out, I didn't really want to code today.

## 2022-03-22

I did some research parser and AST trees are hard to code in rust. The project will be on hiatus for some time until I
learn about ASTs. Also, I got a gig at a resturant, so I'm going to be working on that project for a while.

## 2022-03-23

I forgot to mention that I'll work on the code to make some refactorings overtime.  
I'll also make a new release for the kind of finished lexer.  
I must also thank some friends that helped me with the lexer and logos.
