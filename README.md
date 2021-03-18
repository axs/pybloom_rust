# pybloom_rust
python lib wrapped around rust bloom crate

maturin build

outputs to targets/


In [1]: import bloom

In [2]: a=bloom.PyBloomFilter()

In [3]: a.insert(2)

In [4]: a
Out[4]: <PyBloomFilter at 0x635cc90>

In [5]: a.contains(2)
Out[5]: True

In [6]: a.contains(23)
Out[6]: False
