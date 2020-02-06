#!/usr/bin/env python3

class StringOrInt:
    def __init__(self):
        self.string_variant = None
        self.int_variant = None

    def __str__(self):
        return "hi"

x = StringOrInt()
print(x)
