class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        letterBank = [0] * 26
        for letter in magazine:
            letterBank[ord(letter) - ord('a')] += 1
        for letter in ransomNote:
            idx = ord(letter) - ord('a')
            if letterBank[idx] == 0:
                returan False
            letterBank[idx] -= 1
        return True
