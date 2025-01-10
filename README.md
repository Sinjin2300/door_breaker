# door_breaker
A program written in Rust to determine all possible code combinations when you only know some of the data.

## Why?
I wrote this because my apartment has one of [these](https://www.locklypro.com/products) smart locks and I wanted to see how hard it would be to enumerate the possible passwords with limited information. In the lock the passcode is the same but the numbad has a 'range' of values that change and you just select the set containing the digit you want to press.

## Usage
The program takes an input of a filepath to a txt file that contains the information that is known about the code. There are some rules about formatting the data.
###  Formatting
- All lines **must begin and end** with braces: [1,2,3,4]
- A range of values is denoted with parenthensis: (1,2,3)
- A wildcard can be used for a range 0-9: *

### Example Data
```
[1, 2, (3,4), *]
[(1,2), 2, 3, 5]
```

## Example Usage 
```
> door_breaker -i testdata1.txt

-Avaliable Combinations-
[9,2,3,1,4,2]
[9,2,3,2,4,2]
```
testdata1.txt
```
[(0,1,9), *, 3 , (1,2), 4, 2]
[9, 2, 3 ,(1,2), 4, 2]
[(0,1,9), (2,7,8), (1,2,3) , (1,2), 4, 2]
```
