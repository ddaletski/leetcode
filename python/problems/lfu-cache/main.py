from dataclasses import dataclass
from typing import Any


@dataclass
class ValNode:
    freq_node: Any
    prev: Any = None
    next: Any = None
    key: Any = None

    def remove(self) -> None:
        print(f"remove {self.key} with freq {self.freq_node.freq}")

        prev = self.prev
        next = self.next

        if next is None and prev is None:
            print(f"last val with freq {self.freq_node.freq}")
            self.freq_node.remove()
        elif prev is None:
            print(f"no prev key before {self.key}")
            next.prev = None
            print(f"head before: {self.freq_node.head.key}")
            self.freq_node.head = next
            print(f"head after: {self.freq_node.head.key}")
        elif next is None:
            print(f"no next key after {self.key}")
            prev.next = None
            self.freq_node.tail = prev
        else:
            print(f"{self.key} is between {prev.key}, {next.key}")
            prev.next = next
            next.prev = prev

    def increment(self) -> None:
        print(f"increment {self.key}")

        freq_node = self.freq_node
        current_freq = freq_node.freq
        next_freq_node = freq_node.next

        self.remove()

        if freq_node.root.root is None:
            print("AHA")

        if next_freq_node is None:
            print(f"creating new freq {current_freq + 1} for key {self.key}")
            next_freq_node = FreqNode(
                freq_node.cache, current_freq + 1, prev=freq_node)
            next_freq_node.push_node(self)
            freq_node.next = next_freq_node
        elif next_freq_node.freq == current_freq + 1:
            print(
                f"pushing key {self.key} to the next freq {next_freq_node.freq}")
            next_freq_node.push_node(self)
        else:
            print(
                f"creating new freq {current_freq + 1} for key {self.key} between {freq_node.freq} and {next_freq_node.freq}")
            new_node = FreqNode(freq_node.cache, current_freq + 1, prev=freq_node,
                                next=next_freq_node)
            new_node.push_node(self)
            freq_node.next = new_node
            next_freq_node.prev = new_node


@dataclass
class FreqNode:
    cache: Any
    freq: int
    prev: Any = None
    next: Any = None
    head: ValNode = None
    tail: ValNode = None

    def push(self, key) -> ValNode:
        print(f"pushing val {key}")
        if self.tail is None:
            self.head = ValNode(self, key=key)
            self.tail = self.head
        else:
            self.tail.next = ValNode(self, self.tail, key=key)
            self.tail = self.tail.next

        return self.tail

    def push_node(self, node: ValNode):
        print(f"pushing node {node.key}: {node.freq_node.freq}")
        tail = self.tail

        if self.tail is None:
            self.tail = node
            self.head = node
            node.prev = None
        else:
            tail.next = node
            node.prev = tail
            self.tail = node

        node.next = None
        node.freq_node = self

    def pop(self) -> ValNode:
        key = self.head.key

        self.head.remove()

        return key

    def remove(self) -> None:
        cache = self.cache

        if self.prev is None and self.next is None:
            cache.tree = None
        elif self.prev is None:
            cache.tree = self.next
            self.next.prev = None
        elif self.next is None:
            self.prev.next = None
        else:
            next = self.next
            prev = self.prev
            prev.next = next
            next.prev = prev


class LFUCache:
    def __init__(self, capacity: int):
        self.tree = None
        self.mapping = dict()
        self.values = dict()
        self.capacity = capacity
        self.size = 0

    def get(self, key: int) -> int:
        print(f"get {key}")
        node = self.mapping.get(key)
        if node is None:
            return -1
        else:
            node.increment()
            return self.values[node.key]

    def put(self, key: int, value: int) -> None:
        print(f"put {key}: {value}")
        existing = self.mapping.get(key)
        if existing is None:
            if self.size == self.capacity:
                print(f"capacity reached")
                print(f"root = {self.tree.head.key}")
                removed_key = self.tree.pop()
                del self.values[removed_key]
                del self.mapping[removed_key]
                self.size -= 1

            if self.tree is None or self.tree.freq != 1:
                self.tree = FreqNode(self, 1, next=self.tree)

            node = self.tree.push(key)
            self.mapping[key] = node
            self.values[key] = value
            self.size += 1
        else:
            # existing.increment()
            self.values[key] = value
