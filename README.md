# sixteen-years-of-success

This is dedicated to [the man who spent 16
years](https://mastodon.gamedev.place/@shanecelis/112367041624311312) writing
out the numbers from one to one million in English.

# Example

``` sh
> ./sixteen-years-of-success 10
Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten.
```

## No argument will go to one million

``` sh
> time ./sixteen-years-of-success > so-many-numbers.txt
./sixteen-years-of-success > so-many-numbers.txt  1.75s user 0.10s system 80% cpu 2.291 total
> ls -lh so-many-numbers.txt
-rw-r--r--@ 1 shane  staff    59M May  2 16:51 so-many-numbers.txt
```
