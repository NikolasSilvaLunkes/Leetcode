class Solution(object):
    def getRow(self, rowIndex):
        current = [1]
        rowIndex+=2
        for i in range(rowIndex):
            last = current
            current = []
            for j in range(i):
                if j in range(1,i-1):
                    current.append(last[j-1]+last[j])
                    
                else:
                    current.append(1)
        return(current)