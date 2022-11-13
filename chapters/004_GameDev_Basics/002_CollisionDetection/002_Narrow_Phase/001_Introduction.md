Narrow-Phase Collision Detection: did it really collide?
-------------------------------------------------------

First of all, we need to see how we can make sure that two objects really collide with each other.

Sometimes this presents a (quite common) problem when it comes to precision: computers have no knowledge of infinity (due to their finiteness, see [computers are (not) precise](#precision_issues)). This means that we may need to give some leeway and define an "acceptable error" in our calculations, thus we will create a "small enough value" (which in math is represented by the greek letter "epsilon": $\epsilon$) and change our algorithms accordingly.
