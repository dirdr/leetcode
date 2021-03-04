/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) {
 *         val = x;
 *         next = null;
 *     }
 * }
 */
public class Solution {
    public ListNode getIntersectionNode(ListNode headA, ListNode headB) {
        if (headA == null || headB == null) {
            return null;
        }
        int lenA = lengthFinder(headA);
        int lenB = lengthFinder(headB);
        while (lenA != lenB) {
            if (lenA > lenB) {
                headA = headA.next;
                lenA--;
            }
            if (lenB > lenA) {
                headB = headB.next;
                lenB--;
            }
        }
        while (headA != null && headB != null) {
            if (headA == headB) {
                return headA;
            }
            headA = headA.next;
            headB = headB.next;
        }
        return null;
    }
    
    public int lengthFinder(ListNode n) {
        int i = 0;
        while (n.next != null) {
            i++;
            n = n.next;
        }
        return i;
    }
}

/**
 * Runtime: 2 ms, faster than 36.86% of Java online submissions for Intersection of Two Linked Lists.
 * Memory Usage: 52.2 MB, less than 10.12% of Java online submissions for Intersection of Two Linked Lists.
 */





