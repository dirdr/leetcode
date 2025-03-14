class Solution:
    def isPalindrome(self, s: str) -> bool:
        clean = ''.join(char for char in s if char.isalnum()).lower()
        for (ch, chRev) in zip(clean, clean[::-1]):
            if ch != chRev:
                return False
        return True
