from hashlib import shake_128, shake_256

"""
hashlib has implemented shake_128, shake_256
but they haven't designed it so you can read
bytes properly... every call generates all
bytes without updating

shake_128.digest(1) == shake_128.digest(1)

and we have no shake_128.read() :(

So, here's a wrapper which calls to 
shake_128.digest and collects a bunch of bytes
which we can then read through.
"""


class Shake:
    def __init__(self, algorithm, block_length):
        self.algorithm = algorithm
        self.block_length = block_length
        self.read_blocks = 0
        self.read_data = b""

    def absorb(self, input_bytes):
        """
        Initialise the XOF with the seed
        and reset other init.
        """
        self.read_blocks = 0
        self.read_data = b""
        self.xof = self.algorithm(input_bytes)

    def digest(self, input_bytes, length):
        """
        Sometimes we just want n bytes, so rather than read
        them slowly, we can just pull them straight out.
        """
        return self.algorithm(input_bytes).digest(length)

    def get_n_blocks(self, n):
        """
        Requests n blocks from Shake and stores them
        Ignores any bytes previously read
        """
        byte_count = self.block_length * (self.read_blocks + n)
        xof_data = self.xof.digest(byte_count)
        self.read_blocks += n
        # return last n blocks of bytes
        self.read_data = xof_data[-self.block_length * n :]

    def read(self, n):
        # Make sure there are enough bytes to read
        if n > len(self.read_data):
            self.get_n_blocks(5 * n)
        send = self.read_data[:n]
        self.read_data = self.read_data[n:]
        return send


Shake128 = Shake(shake_128, 168)
Shake256 = Shake(shake_256, 136)
