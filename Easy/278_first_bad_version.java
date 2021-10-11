/* The isBadVersion API is defined in the parent class VersionControl.
      boolean isBadVersion(int version); */

public class Solution extends VersionControl {
    public int firstBadVersion(int n) { 
        int l = 0;
        int r = n;
        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (isBadVersion(mid) && !isBadVersion(mid-1)) {
                return mid;
            } 
            if (isBadVersion(mid)) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        return -1;    
    }
}

/**
* Runtime: 32 ms, faster than 6.39% of Java online submissions for First Bad Version.
* Memory Usage: 37.6 MB, less than 29.75% of Java online submissions for First Bad Version.
*/
