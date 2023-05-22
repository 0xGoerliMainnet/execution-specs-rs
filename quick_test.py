
from ethereum.base_types import Bytes
from ethereum.frontier.trie import Trie, trie_set
from ethereum.frontier.fork_types import Bytes
from ethereum.frontier.trie import Trie, root, trie_set
from ethereum.utils.hexadecimal import (
    has_hex_prefix,
    hex_to_bytes,
    remove_hex_prefix,
)

def to_bytes(data: str) -> Bytes:
    if data is None:
        return b""
    if has_hex_prefix(data):
        return hex_to_bytes(data)

    return data.encode()

st: Trie[Bytes, int] = Trie(secured=False, default=b"")
# trie_set(st, b"do", b"verb")
# trie_set(st, b"ether", b"wookiedoo")
# trie_set(st, b"horse", b"stallion")
# trie_set(st, b"shaman", b"horse")
# trie_set(st, b"doge", b"coin")
# trie_set(st, b"etherb", "")
# trie_set(st, b"dog", b"puppy")
# trie_set(st, b"shamanb", "")
trie_set(st, b"abc", b"123" )
trie_set(st, b"abcd", b"abcd" )
trie_set(st, b"abc", b"abc")

# trie_set(st, b"hello", b"xyz")
# trie_set(st, b"hello", b"abcdefghijklmnopqrstuvwxyz")
#trie_set(st, b"help", b"xyz")
print(root(st))
print(st)

