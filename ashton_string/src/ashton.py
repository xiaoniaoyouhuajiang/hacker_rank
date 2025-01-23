def ashtonString(s, k):
    # 将 k 转换为 0-based 索引
    kv = k - 1
    
    # 字符串长度
    N = len(s)
    
    # sr 是一个二维数组，用于存储后缀的排名
    # sr[i][j] 表示第 j 个后缀在第 i 次倍增排序中的排名
    sr = [[0 for _ in range(N)] for __ in range(17)]
    
    # 初始化第一层排名（字符的 ASCII 值减去 'a' 的 ASCII 值）
    sr[0] = [ord(c) - 97 for c in s]
    
    # L 是一个列表，用于存储后缀的排名和索引
    L = [0] * N
    
    # cnt 是倍增的步长
    cnt = 1
    
    # kf 是一个排序关键字函数，用于比较后缀的排名
    kf = lambda x: x[0] * (N + 1) + x[1]
    
    # 构建后缀数组
    for i in range(1, 17):
        # 对每个后缀，计算当前步长的排名
        for j in range(N):
            L[j] = (sr[i - 1][j], sr[i - 1][j + cnt] if j + cnt < N else -1, j)
        
        # 按照关键字排序
        L.sort(key=kf)
        
        # 更新当前步长的排名
        sr[i][L[0][2]] = 0
        cr = 0
        for j in range(1, N):
            if L[j - 1][0] != L[j][0] or L[j - 1][1] != L[j][1]:
                cr += 1
            sr[i][L[j][2]] = cr
        
        # 倍增步长
        cnt *= 2
        if cnt >= N:
            break
    
    # sa 是后缀数组，存储排序后的后缀的起始位置
    sa = [l[2] for l in L]
    
    # rank 数组记录每个后缀的排名
    rank = [0] * N
    
    # lcp 数组记录相邻后缀的最长公共前缀长度
    lcp = [0] * N
    
    # 计算 rank 数组
    for i in range(N):
        rank[sa[i]] = i
    
    # 计算 LCP 数组
    k_lcp = 0
    for i in range(N):
        if rank[i] == N - 1:  # 最后一个后缀没有下一个后缀
            k_lcp = 0
            continue
        j = sa[rank[i] + 1]  # 下一个后缀的起始位置
        # 计算 LCP
        while j + k_lcp < N and i + k_lcp < N and s[i + k_lcp] == s[j + k_lcp]:
            k_lcp += 1
        lcp[rank[i]] = k_lcp
        if k_lcp > 0:
            k_lcp -= 1
    
    # numprev 记录前一个后缀的 LCP 值
    numprev = 0
    
    # tri 是一个计算三角数的函数
    tri = lambda x: ((x + 1) * x) >> 1
    
    # 打印后缀数组和 LCP 数组（调试用）
    print(sa)
    print(lcp)
    
    # 查找第 k 个字符
    for i in range(N):
        mylen = N - sa[i]  # 当前后缀的长度
        # 当前后缀贡献的新子串数量 = (mylen * (mylen + 1)) // 2 - (numprev * (numprev + 1)) // 2
        tt = tri(mylen) - tri(numprev)
        if kv < tt:
            # 在当前后缀的子串中查找第 kv 个字符
            for j in range(1 + numprev, 1 + mylen):
                if kv < j:
                    return s[sa[i] + kv]
                kv -= j
        kv -= tt
        numprev = lcp[i]  # 更新前一个后缀的 LCP 值
    
    # 如果没有找到，返回空字符串
    return ''