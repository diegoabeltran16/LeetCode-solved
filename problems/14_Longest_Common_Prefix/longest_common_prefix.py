class Solution(object):
    def longestCommonPrefix(self, strs):
        """
        Finds the longest common prefix among a list of strings.

        :type strs: List[str]
        :rtype: str

        Approach:
        ----------
        1. Assume the first string is the common prefix.
        2. For each subsequent string:
            - While it does not start with the current prefix:
                - Remove the last character from the prefix.
                - If prefix becomes empty, return "" (no common prefix).
        3. Return the resulting prefix.

        This is efficient because it quickly reduces the prefix size only when needed,
        rather than comparing each character individually across all strings.

        Example:
        --------
        Input: ["flower","flow","flight"]
        Process:
          prefix = "flower"
          Check "flow":
            "flower" not a prefix of "flow" -> cut to "flowe"
            "flowe" not a prefix -> cut to "flow"
            "flow" is a prefix -> continue
          Check "flight":
            "flow" not a prefix -> cut to "flo"
            "flo" not a prefix -> cut to "fl"
            "fl" is a prefix -> continue

        Output: "fl"
        """
        if not strs:
            return ""

        prefix = strs[0]

        for s in strs[1:]:
            while not s.startswith(prefix):
                prefix = prefix[:-1]
                if not prefix:
                    return ""

        return prefix
