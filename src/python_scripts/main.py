import re
import string
import random
import libmyrustlib
import numpy as np
from timeit import default_timer as timer

# TESTING RUST VS PYTHON PERFORMANCE

def bubbleSort(arr): 
    n = len(arr) 
    for i in range(n): 
        swapped = False
        for j in range(0, n-i-1): 

            if arr[j] > arr[j+1] : 
                arr[j], arr[j+1] = arr[j+1], arr[j] 
                swapped = True
        if swapped == False: 
            break

arr = list(np.random.randint(0,20000, 20000))

def test_bubble_sort1():
    libmyrustlib.sort_list(arr)

def test_bubble_sort2():
    bubbleSort(arr)

print(f"List with -> {len(arr)} elements")

start = timer()
test_bubble_sort1()
end = timer()
print(f"Bubble sort from Rust takes {end - start}") 

start = timer()
test_bubble_sort2()
end = timer()
print(f"Bubble sort from Python takes {end - start}") 









# implementation for counting doubles (with rust)
""" 


def get_dict_value():
    data = {"developer": "Andres", "edad": 21}
    tofind = "developer"
    valueFromRust = libmyrustlib.get_value_from_rust(data, tofind)
    print(f"Rust returned {valueFromRust} from {tofind}")



def count_doubles(val):
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