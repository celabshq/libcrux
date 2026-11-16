class Module:
    def __init__(self, ring):
        self.ring = ring

    def __bit_unpack(self, input_bytes, m, n, alg, packed_len, *args, is_ntt=False):
        poly_bytes = [
            input_bytes[i : i + packed_len]
            for i in range(0, len(input_bytes), packed_len)
        ]
        matrix = [
            [alg(poly_bytes[n * i + j], *args, is_ntt=is_ntt) for j in range(n)]
            for i in range(m)
        ]
        return self(matrix)

    def bit_unpack_t0(self, input_bytes, m, n, is_ntt=False):
        packed_len = 416
        algorithm = self.ring.bit_unpack_t0
        return self.__bit_unpack(input_bytes, m, n, algorithm, packed_len, is_ntt=False)

    def bit_unpack_t1(self, input_bytes, m, n, is_ntt=False):
        packed_len = 320
        algorithm = self.ring.bit_unpack_t1
        return self.__bit_unpack(input_bytes, m, n, algorithm, packed_len, is_ntt=False)

    def bit_unpack_s(self, input_bytes, m, n, eta, is_ntt=False):
        # Level 2 and 5 parameter set
        if eta == 2:
            packed_len = 96
        # Level 3 parameter set
        elif eta == 4:
            packed_len = 128
        else:
            raise ValueError("Expected eta to be either 2 or 4")
        algorithm = self.ring.bit_unpack_s
        return self.__bit_unpack(
            input_bytes, m, n, algorithm, packed_len, eta, is_ntt=False
        )

    def bit_unpack_w(self, input_bytes, m, n, gamma_2, is_ntt=False):
        # Level 2 parameter set
        if gamma_2 == 95232:
            packed_len = 192
        # Level 3 and 5 parameter set
        elif gamma_2 == 261888:
            packed_len = 128
        else:
            raise ValueError("Expected gamma_2 to be either (q-1)/88 or (q-1)/32")
        algorithm = self.ring.bit_unpack_w
        return self.__bit_unpack(
            input_bytes, m, n, algorithm, packed_len, gamma_2, is_ntt=False
        )

    def bit_unpack_z(self, input_bytes, m, n, gamma_1, is_ntt=False):
        # Level 2 parameter set
        if gamma_1 == (1 << 17):
            packed_len = 576
        # Level 3 and 5 parameter set
        elif gamma_1 == (1 << 19):
            packed_len = 640
        else:
            raise ValueError("Expected gamma_1 to be either 2^17 or 2^19")
        algorithm = self.ring.bit_unpack_z
        return self.__bit_unpack(
            input_bytes, m, n, algorithm, packed_len, gamma_1, is_ntt=False
        )

    def __repr__(self):
        return f"Module over the commutative ring: {self.ring}"

    def __str__(self):
        return f"Module over the commutative ring: {self.ring}"

    def __eq__(self, other):
        return self.ring == other.ring

    def __call__(self, matrix_elements):
        if not isinstance(matrix_elements, list):
            raise TypeError(f"Elements of a module are matrices, with elements .")

        if isinstance(matrix_elements[0], list):
            for element_list in matrix_elements:
                if not all(isinstance(aij, self.ring.element) for aij in element_list):
                    raise TypeError(
                        f"All elements of the matrix must be elements of the ring: {self.ring}"
                    )
            return Module.Matrix(self, matrix_elements)

        elif isinstance(matrix_elements[0], self.ring.element):
            if not all(isinstance(aij, self.ring.element) for aij in matrix_elements):
                raise TypeError(
                    f"All elements of the matrix must be elements of the ring: {self.ring}"
                )
            return Module.Matrix(self, [matrix_elements])

        else:
            raise TypeError(
                f"Elements of a module are matrices, built from elements of the base ring."
            )

    class Matrix:
        def __init__(self, parent, matrix_elements):
            self.parent = parent
            self.rows = matrix_elements
            self.m = len(matrix_elements)
            self.n = len(matrix_elements[0])
            if not self.check_dimensions():
                raise ValueError("Inconsistent row lengths in matrix")

        def get_dim(self):
            return self.m, self.n

        def check_dimensions(self):
            return all(len(row) == self.n for row in self.rows)

        def transpose(self):
            new_rows = [list(item) for item in zip(*self.rows)]
            return self.parent(new_rows)

        def transpose_self(self):
            self.m, self.n = self.n, self.m
            self.rows = [list(item) for item in zip(*self.rows)]
            return self

        def reduce_coefficents(self):
            for row in self.rows:
                for ele in row:
                    ele.reduce_coefficents()
            return self

        def to_montgomery(self):
            for row in self.rows:
                for ele in row:
                    ele.to_montgomery()
            return self

        def from_montgomery(self):
            for row in self.rows:
                for ele in row:
                    ele.from_montgomery()
            return self

        def scale(self, other):
            """
            Multiply each element of the matrix by a polynomial or integer
            """
            if not (
                isinstance(other, self.parent.ring.Polynomial) or isinstance(other, int)
            ):
                raise TypeError(
                    "Can only multiply elements with polynomials or integers"
                )

            matrix = [[other * ele for ele in row] for row in self.rows]
            return self.parent(matrix)

        def check_norm_bound(self, bound):
            for row in self.rows:
                if any(p.check_norm_bound(bound) for p in row):
                    return True
            return False

        def power_2_round(self, d):
            """
            Applies `power_2_round` on every element in the
            Matrix to create two matrices.
            """
            m1_elements = [[0 for _ in range(self.n)] for _ in range(self.m)]
            m0_elements = [[0 for _ in range(self.n)] for _ in range(self.m)]

            for i in range(self.m):
                for j in range(self.n):
                    m1_ele, m0_ele = self[i][j].power_2_round(d)
                    m1_elements[i][j] = m1_ele
                    m0_elements[i][j] = m0_ele

            return self.parent(m1_elements), self.parent(m0_elements)

        def decompose(self, alpha):
            """
            Applies `power_2_round` on every element in the
            Matrix to create two matrices.
            """
            m1_elements = [[0 for _ in range(self.n)] for _ in range(self.m)]
            m0_elements = [[0 for _ in range(self.n)] for _ in range(self.m)]

            for i in range(self.m):
                for j in range(self.n):
                    m1_ele, m0_ele = self[i][j].decompose(alpha)
                    m1_elements[i][j] = m1_ele
                    m0_elements[i][j] = m0_ele

            return self.parent(m1_elements), self.parent(m0_elements)

        def __bit_pack(self, algorithm, *args):
            return b"".join(algorithm(poly, *args) for row in self.rows for poly in row)

        def bit_pack_t1(self):
            algorithm = self.parent.ring.Polynomial.bit_pack_t1
            return self.__bit_pack(algorithm)

        def bit_pack_t0(self):
            algorithm = self.parent.ring.Polynomial.bit_pack_t0
            return self.__bit_pack(algorithm)

        def bit_pack_s(self, eta):
            algorithm = self.parent.ring.Polynomial.bit_pack_s
            return self.__bit_pack(algorithm, eta)

        def bit_pack_w(self, gamma_2):
            algorithm = self.parent.ring.Polynomial.bit_pack_w
            return self.__bit_pack(algorithm, gamma_2)

        def bit_pack_z(self, gamma_1):
            algorithm = self.parent.ring.Polynomial.bit_pack_z
            return self.__bit_pack(algorithm, gamma_1)

        def to_ntt(self):
            for row in self.rows:
                for ele in row:
                    ele.to_ntt()
            return self

        def from_ntt(self):
            for row in self.rows:
                for ele in row:
                    ele.from_ntt()
            return self

        def copy_to_ntt(self):
            matrix = [[ele.copy_to_ntt() for ele in row] for row in self.rows]
            return self.parent(matrix)

        def copy_from_ntt(self):
            matrix = [[ele.copy_from_ntt() for ele in row] for row in self.rows]
            return self.parent(matrix)

        def high_bits(self, alpha, is_ntt=False):
            matrix = [
                [ele.high_bits(alpha, is_ntt=is_ntt) for ele in row]
                for row in self.rows
            ]
            return self.parent(matrix)

        def low_bits(self, alpha, is_ntt=False):
            matrix = [
                [ele.low_bits(alpha, is_ntt=is_ntt) for ele in row] for row in self.rows
            ]
            return self.parent(matrix)

        def __getitem__(self, i):
            return self.rows[i]

        def __eq__(self, other):
            return other.rows == self.rows

        def __add__(self, other):
            if not isinstance(other, Module.Matrix):
                raise TypeError("Can only add matrcies to other matrices")
            if self.parent != other.parent:
                raise TypeError("Matricies must have the same base ring")
            if self.get_dim() != other.get_dim():
                raise ValueError("Matrices are not of the same dimensions")

            new_elements = []
            for i in range(self.m):
                new_elements.append(
                    [a + b for a, b in zip(self.rows[i], other.rows[i])]
                )
            return self.parent(new_elements)

        def __radd__(self, other):
            return self.__add__(other)

        def __iadd__(self, other):
            self = self + other
            return self

        def __sub__(self, other):
            if not isinstance(other, Module.Matrix):
                raise TypeError("Can only subtract matrcies from other matrices")
            if self.parent != other.parent:
                raise TypeError("Matricies must have the same base ring")
            if self.get_dim() != other.get_dim():
                raise ValueError("Matrices are not of the same dimensions")

            new_elements = []
            for i in range(self.m):
                new_elements.append(
                    [a - b for a, b in zip(self.rows[i], other.rows[i])]
                )
            return self.parent(new_elements)

        def __rsub__(self, other):
            return self.__sub__(other)

        def __isub__(self, other):
            self = self - other
            return self

        def __matmul__(self, other):
            """
            Denoted A @ B
            """
            if not isinstance(other, Module.Matrix):
                raise TypeError("Can only multiply matrcies with other matrices")
            if (self.parent) != other.parent:
                print(self.parent, "\n", other.parent)
                raise TypeError("Matricies must have the same base ring")
            if self.n != other.m:
                raise ValueError("Matrices are of incompatible dimensions")

            new_elements = [
                [
                    sum(a * b for a, b in zip(A_row, B_col))
                    for B_col in other.transpose().rows
                ]
                for A_row in self.rows
            ]
            return self.parent(new_elements)

        def __repr__(self):
            if len(self.rows) == 1:
                return str(self.rows[0])
            max_col_width = []
            for n_col in range(self.n):
                max_col_width.append(max(len(str(row[n_col])) for row in self.rows))
            info = "]\n[".join(
                [
                    ", ".join(
                        [f"{str(x):>{max_col_width[i]}}" for i, x in enumerate(r)]
                    )
                    for r in self.rows
                ]
            )
            return f"[{info}]"
