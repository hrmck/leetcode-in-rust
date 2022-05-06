#
# @lc app=leetcode id=589 lang=python3
#
# [589] N-ary Tree Preorder Traversal
#

# @lc code=start
"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""
class Solution:
    def preorder(self, root: 'Node') -> List[int]:
        if not root:
            return []
        output = []
        stack = [root]
        while stack:
            temp = stack.pop()
            output.append(temp.val)
            stack.extend(temp.children[::-1])
        return output
# @lc code=end

