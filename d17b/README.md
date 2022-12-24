I have not implemented the solution yet, but you can use the code from part a to calculate it with some simple math. The
idea is to find cycles in which the stack of rocks repeats. There are cycles of
length `input_length  * number_of_shapes`. We can then find the height and rocks gained per cycle by computing the start
and end of such a cycle. You still have to add the offset afterwards.

Your formula should look something like `floor(10^12 / delta_rocks) * delta_height + offset`.
