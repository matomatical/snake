

class stack:
    """
    amortized O(1) stack
    """
    def __init__(self, items=[]):
        """
        items: optional iterator of initial contents
        """
        self.items = [item for item in items]
    def push(self, item):
        self.items.append(item)
    def pop(self):
        """
        error if empty
        """
        return self.items.pop()
    def peek(self):
        """
        error if empty
        """
        return self.items[-1]
    def empty(self):
        return (len(self.items) == 0)
    def __contains__(self, item):
        return (item in self.items)
    def __len__(self):
        return len(self.items)
    def __iter__(self):
        """lifo order, does not pop though"""
        return reversed(self.items)
    def __repr__(self):
        return f"stack({self.items})"

class deque:
    """
    amortized O(1) double-ended queue
    """
    def __init__(self, items=None):
        self.in_ = stack(items)
        self.out = stack()
    def push(self, item):
        self.in_.push(item)
    def _process(self):
        # assert empty(self.out)
        self.in_, self.out = self.out, stack(self.in_) # (flips the stack)
        # there may be a less instance-wasteful way?
    def pull(self):
        if self.out.empty():
            self._process()
        return self.out.pop()
    def __contains__(self, item):
        return (item in self.in_) or (item in self.out)
    def __len__(self):
        return len(self.in_) + len(self.out)
    def empty(self):
        return self.in_.empty() and self.out.empty()
    def __repr__(self):
        return f"deque({list(self.out) + list(stack(self.in_))})"
    
class v:
    """
    simple 2d integer vector
    """
    # TODO
