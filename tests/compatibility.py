import unittest
import lzstring
import lzma_pyo3
import random
import string


class CompatibilityTest(unittest.TestCase):

    def test_compress_match(self):
        N1 = 100
        N2 = 1000
        x = lzstring.LZString()
        sequences = []
        for i in range(N2):
            sequences.append(''.join(random.choice(string.ascii_uppercase + string.digits) for _ in range(N1)))

        for i in sequences:
            c1 = x.compressToBase64(i).strip("=")
            c2 = lzma_pyo3.compressToBase64(i).strip("=")
            self.assertEqual(c1.strip("="), c2.strip("="))
            self.assertEqual(x.decompressFromBase64(c2), lzma_pyo3.decompressFromBase64(c1))


if __name__ == '__main__':
    unittest.main()
