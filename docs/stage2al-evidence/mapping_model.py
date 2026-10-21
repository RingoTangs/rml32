"""Restricted static deduction model, NOT an executed reference VM or Rust runtime."""


def hash_key(key, table):
    if type(key) is int:
        if not -(2**31) <= key < 2**31:
            raise ValueError('Int must be i32')
        return key >> 4
    if type(key) is not bytes:
        raise ValueError('only Int/Bytes keys')
    prefix = key.split(b'\0', 1)[0][:32]
    if not prefix:
        return 0
    hi, lo = table[prefix[0]], 0
    for byte in prefix[1:]:
        hi, lo = table[hi ^ byte], table[lo ^ byte]
    return (hi << 8) | lo


class Mapping:
    """Success paths only, attrib=1, valid ordinary Bytes; no OOM/GC model."""
    def __init__(self, table, hint=0):
        if type(hint) is not int or hint < 0:
            raise ValueError('nonnegative allocation hint')
        size = min(hint, 0x300000)
        capacity = 8 if size <= 8 else 1 << size.bit_length()
        self.table = table
        self.buckets = [[] for _ in range(capacity)]
        self.unfilled = capacity * 80 // 100
        self.count = 0

    def grow(self):
        old = len(self.buckets)
        if old * 2 > 0x400000:
            raise ValueError('native growth limit; failure not simulated')
        self.buckets.extend([] for _ in range(old))
        self.unfilled = old * 80 // 100
        for i in range(old - 1, -1, -1):
            low, high = [], []
            for node in self.buckets[i]:
                if hash_key(node[0], self.table) & old:
                    if not high:
                        self.unfilled -= 1
                    high.insert(0, node)
                else:
                    low.append(node)
            if self.buckets[i] and not low:
                self.unfilled += 1
            self.buckets[i], self.buckets[i + old] = low, high

    def insert(self, key, value):
        h = hash_key(key, self.table)
        bucket = self.buckets[h & (len(self.buckets) - 1)]
        for node in bucket:
            if type(node[0]) is type(key) and node[0] == key:
                node[1] = value
                return
        if not bucket:
            self.unfilled -= 1
            if self.unfilled == 0:
                self.grow()
                bucket = self.buckets[h & (len(self.buckets) - 1)]
        if self.count + 1 > 0x300000:
            raise ValueError('native count limit; failure not simulated')
        bucket.insert(0, [key, value])
        self.count += 1

    def delete(self, key):
        bucket = self.buckets[hash_key(key, self.table) & (len(self.buckets) - 1)]
        for i, node in enumerate(bucket):
            if type(node[0]) is type(key) and node[0] == key:
                bucket.pop(i)
                self.count -= 1
                if not bucket:
                    self.unfilled += 1
                return

    def order(self):
        return [node[0] for bucket in self.buckets for node in bucket]


class Cursor:
    def __init__(self, owner):
        self.owner = owner
        self.bucket = self.subindex = self.ordinal = 0

    def advance(self, binding):
        ordinal = self.ordinal
        self.ordinal = (ordinal + 1) & 0xffffffff
        skip = self.subindex
        self.subindex = (skip + 1) & 0xffffffff
        # Native TEST/JG treats a wrapped negative old subindex as zero skips.
        skip = max(0, skip if skip < 2**31 else skip - 2**32)
        while self.bucket < len(self.owner.buckets):
            nodes = self.owner.buckets[self.bucket]
            if skip < len(nodes):
                key, value = nodes[skip]
                output = binding if type(binding) is list and len(binding) == 3 else [None]*3
                signed = ordinal if ordinal < 2**31 else ordinal - 2**32
                output[:] = key, value, signed
                return True, output
            self.bucket += 1
            skip = 0
            self.subindex = 1
        return False, binding
