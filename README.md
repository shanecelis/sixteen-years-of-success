# sixteen-years-of-success

![Les Stewart](assets/les-stewart.jpg)

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">Les Stewart (Australia) is the fastest person to have typed out the numbers one to one million on a typewriter.<br><br>It took 19,990 quarto sheets and 16 years to complete, finishing on December 7, 1998. </p>&mdash; Guinness World Records (@GWR) <a href="https://twitter.com/GWR/status/1480500919422701568?ref_src=twsrc%5Etfw">January 10, 2022</a></blockquote> 

This is dedicated to Les Stewart the man who spent 16 years writing out the
numbers from one to one million in English&mdash;[with one finger!](https://www.ctvnews.ca/halifax-man-spends-12-years-typing-out-numbers-to-1m-1.648060)

# Example

## One argument specifies the max

``` sh
> ./sixteen-years-of-success 5
one,
two,
three,
four,
five.
```

## Two arguments specifies the min and max

``` sh
> ./sixteen-years-of-success 5
five,
six,
seven,
eight,
nine,
ten.
```

## No arguments will go from one to one million

``` sh
> time ./sixteen-years-of-success > so-many-numbers.txt
./sixteen-years-of-success > so-many-numbers.txt  1.75s user 0.10s system 80% cpu 2.291 total
> ls -lh so-many-numbers.txt
-rw-r--r--@ 1 shane  staff    59M May  2 16:51 so-many-numbers.txt
```

# References

- [I joke on mastodon.](https://mastodon.gamedev.place/@shanecelis/112367041624311312)
