# Beaver
This project aims to execute multiple multiplications by two parties. For example, to perform n multiplications, 2^n-1 correlated numbers, known as the Beaver tuple, should be shared between the two parties in advance.

During the online step, n values are revealed, and we extend these n values to a Beaver tuple by calling the extendfrom() function. After that, the Muls() function can be invoked to perform the desired functionality.
0.1 n values are shared between the two parties and the parties intend to calculate the shares of the product of n numbers securely.
0.2 Call Beaver::gen(n) to generate a Beaver tuple and the call split() to get two shares, distribute the two shares to the two parties. 
1. Each value hold by a party is masked by the beaver tuple's elements.
2. The two party exchange messages and obtain n values.
3. Prepare for the multiplication by calling extendfrom() to get a Beaver tuple called delta
4. Each party execute Muls(). The 1st parameter is delta, the second parameter is the beaver tuple shared in the offline step, the third parameter is a bool value to indicate the party's role.
