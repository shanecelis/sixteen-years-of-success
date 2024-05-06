# sixteen-years-of-success

![Les Stewart](assets/les-stewart.jpg)

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">Les Stewart (Australia) is the fastest person to have typed out the numbers one to one million on a typewriter.<br><br>It took 19,990 quarto sheets and 16 years to complete, finishing on December 7, 1998. </p>&mdash; Guinness World Records (@GWR) <a href="https://twitter.com/GWR/status/1480500919422701568?ref_src=twsrc%5Etfw">January 10, 2022</a></blockquote> 

This is dedicated to Les Stewart the man who spent 16 years writing out the
numbers from one to one million in English&mdash;[with one finger!](https://www.ctvnews.ca/halifax-man-spends-12-years-typing-out-numbers-to-1m-1.648060)

> "I just did what I was best at and if you do what you’re good at, then you’re halfway there."&mdash;[Les Stewart](https://www.news18.com/viral/meet-the-australian-man-who-spent-16-years-typing-numbers-one-to-one-million-8781004.html#)

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
> ./sixteen-years-of-success 5 10
five,
six,
seven,
eight,
nine,
ten.
```

## No arguments will go from one to one million

> Seven manual typewriters, 1000 ink ribbons, 19,890 pages, 16 years and seven months later, he finished with the lines:

```
nine hundred and ninety-nine thousand, nine hundred and ninety-nine.
one million.
```

And this tool:

``` sh
> ./sixteen-years-of-success | tail -2
nine hundred and ninety-nine thousand, nine hundred and ninety-nine,
one million.
```

# References

- [I joke on mastodon](https://mastodon.gamedev.place/@shanecelis/112367041624311312)
- [Typewriter odyssey half over](https://www.cbc.ca/news/canada/typewriter-odyssey-half-over-1.404232)
- [World Record for Typing Numbers in Words](https://www.recordholders.org/en/records/typing.html)
- [Halifax man spends 12 years typing out numbers to 1M](https://www.ctvnews.ca/halifax-man-spends-12-years-typing-out-numbers-to-1m-1.648060)
- [Meet The Australian Man Who Spent 16 Years Typing Numbers One To One Million](https://www.news18.com/viral/meet-the-australian-man-who-spent-16-years-typing-numbers-one-to-one-million-8781004.html#)

