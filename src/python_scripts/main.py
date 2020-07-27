import re
import string
import random
import libmyrustlib   #  <-- Import the Rust implemented module (myrustlib.so)


def get_dict_value():
    data = {"developer": "Andres", "edad": 21}
    tofind = "developer"
    valueFromRust = libmyrustlib.get_value_from_rust(data, tofind)
    print(f"Rust returned {valueFromRust} from {tofind}")


get_dict_value()


# implementation for counting doubles (with rust)
""" def count_doubles(val):
    total = 0
    for c1, c2 in zip(val, val[1:]):
        if c1 == c2:
            total += 1
    return total


double_re = re.compile(r'(?=(.)\1)')


def count_doubles_regex(val):
    return len(double_re.findall(val))


val = ''.join(random.choice(string.ascii_letters) for i in range(1000000))


def test_pure_python(benchmark):
    benchmark(count_doubles, val)


def test_regex(benchmark):
    benchmark(count_doubles_regex, val)


def test_rust(benchmark):   #  <-- Benchmark the Rust version
    benchmark(libmyrustlib.count_doubles, val) """