class Solution:
    def countGoodNumbers(self, n: int) -> int:
        MOD = 10**9 + 7
        even_positions = (n + 1) // 2
        odd_positions = n // 2
        
        def fast_pow(base, exp, mod):
            result = 1
            while exp > 0:
                if exp % 2 == 1:
                    result = (result * base) % mod
                base = (base * base) % mod
                exp //= 2
            return result
        
        return (fast_pow(5, even_positions, MOD) * fast_pow(4, odd_positions, MOD)) % MOD
