
# Continued Fraction

The (simple) continued fraction representation of a real number r is an expression obtained by an iterative process of representing r as the sum of its integer part and the reciprocal of another number, then writing this other number as the sum of its integer part and another reciprocal, and so on. In other words, a continued fraction representation of r is of the form 

![alt text][img1]

where a<sub>0</sub>, a<sub>1</sub>, a<sub>2</sub>, ...  are integers and a<sub>1</sub>, a<sub>2</sub>, ... > 0. We call the a_ i-values _partial quotients_. For example, in the continued fraction representation of 5.4, the partial quotients are a<sub>0</sub> = 5, a<sub>1</sub> = 2, a<sub>2</sub> = 2. This representation of a real number has several applications in theory and practice. If r is a rational number, the partial quotients are eventually all zero, so we only need to consider a finite number of partial quotients.

Given two rational numbers in continued fraction representation, your task is to perform the four elementary arithmetic operations on these numbers and display the results in continued fraction representation.

## Input

The input consists of a single test case. The test case consists of three lines. The first line contains two integers n<sub>1</sub> and n<sub>2</sub>, where 1  <= n_ i  <= 9 is the number of partial quotients of rational number r_ i for 1  <= i  <= 2. The second line contains the partial quotients of r<sub>1</sub> and the third line contains the partial quotients of r<sub>2</sub>. The absolute values of the quotients are not more than 10 and you may assume that r<sub>1</sub> > r<sub>2</sub> > 0.

## Output

Display the partial quotients of the continued fraction representations of r<sub>1</sub>+r<sub>2</sub>, r<sub>1</sub>-r<sub>2</sub>, r<sub>1</sub> X r<sub>2</sub>, and r<sub>1</sub> / r<sub>2</sub>, in order, each in a line. Consecutive partial quotients on each line are separated by a single space. Do not print any trailing zero partial quotients.

<table  summary="sample data">

<tbody>

<tr>

<th>Sample Input 1</th>

<th>Sample Output 1</th>

</tr>

<tr>

<td>

<pre>4 3
5 1 1 2
5 2 2
</pre>

</td>

<td>

<pre>11
0 5
30 4 6
1 27
</pre>

</td>

</tr>

</tbody>

</table>

[img1]: img1.png