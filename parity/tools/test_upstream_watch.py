import unittest

import upstream_watch as uw

OLD = """
// comment with MESHOPTIMIZER_API fake(int x);
enum { meshopt_SimplifyLockBorder = 1 << 0 };
struct meshopt_Bounds { float radius; };
MESHOPTIMIZER_API size_t meshopt_kept(unsigned int* out, const unsigned int* in, size_t n);
MESHOPTIMIZER_API size_t meshopt_changes(unsigned int* out, size_t n);
MESHOPTIMIZER_API void meshopt_gone(void);
"""

NEW = """
enum { meshopt_SimplifyLockBorder = 1 << 0, meshopt_SimplifyNew = 1 << 5 };
struct meshopt_Bounds { float radius; float extra; };
MESHOPTIMIZER_API size_t meshopt_kept(unsigned int* out,
    const unsigned int* in, size_t n);
MESHOPTIMIZER_API size_t meshopt_changes(unsigned int* out, size_t n, float scale);
MESHOPTIMIZER_API void meshopt_fresh(const float* v, size_t n);
template <typename T> inline void meshopt_fresh(T* v, size_t n);
"""


class SignatureDiff(unittest.TestCase):
    def test_signature_diff(self):
        d = uw.diff_headers(OLD, NEW)
        funcs = d["functions"]
        self.assertEqual([r["name"] for r in funcs["added"]],
                         ["meshopt_fresh [c++]", "meshopt_fresh [c]"])
        self.assertEqual([r["name"] for r in funcs["removed"]], ["meshopt_gone [c]"])
        self.assertEqual([r["name"] for r in funcs["changed"]], ["meshopt_changes [c]"])
        self.assertIn("float scale", funcs["changed"][0]["new"])
        self.assertEqual([r["name"] for r in d["types"]["changed"]], ["meshopt_Bounds"])
        self.assertEqual([r["name"] for r in d["constants"]["added"]], ["meshopt_SimplifyNew"])
        # Reflowing a declaration over two lines is not a change; commented-out text is ignored.
        self.assertNotIn("meshopt_kept [c]", [r["name"] for r in funcs["changed"]])
        self.assertNotIn("fake", str(d))


if __name__ == "__main__":
    unittest.main()
