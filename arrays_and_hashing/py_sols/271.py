# t271
# https://leetcode.com/problems/encode-and-decode-strings/description/
# https://neetcode.io/problems/string-encode-and-decode

def main():
    class Solution:
        def encode(self, strs: list[str]) -> str:
            all_strs = ""
            all_len = len(strs)
            lens = [0 for _ in range(all_len)]
            for now_id, now_str in enumerate(strs):
                all_strs += now_str
                lens[all_len - now_id - 1] = len(now_str)

            str_lens = ""
            for now_l in lens:
                str_lens += f",{str(now_l)}"

            return all_strs + str_lens


        def decode(self, s: str) -> list[str]:
            all_len = len(s)
            latest_id = all_len
            pr_id = 0

            result = []
            for ch_id, ch in enumerate(reversed(s)):
                if ch == ',':
                    now_len = int(s[all_len - ch_id:latest_id])
                    result.append(s[pr_id:pr_id+now_len])

                    latest_id = all_len - ch_id - 1
                    pr_id += now_len
                    
                    if pr_id == latest_id:
                        break

            return result



    sol = Solution()
    en1 = sol.encode(["neet", "code", "love", "you"])
    en2 = sol.encode(["we", "say", ":", "yes"])

    de1 = sol.decode(en1)
    de2 = sol.decode(en2)

    print(en1)
    print(en2)
    print(de1)
    print(de2)

if __name__ == "__main__":
    main()

