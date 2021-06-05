"""
Block word remove application.
Convert all the block words to pinyin.
Usage: python3 de-block.py <directory>
"""

import sys
import os
from pypinyin import pinyin

LIST = open("list.txt", "r", encoding="utf-8").read().split()

def main(args):
    if args[1][-1] != "\\" and args[1][-1] != "/":
        loc = args[1] + "/"
    else:
        loc = args[1]
    print("[!] Get directory path:", loc)
    files = os.listdir(loc)
    print("[!] Read files", files)
    for f in files:
        if f[-4:] != ".txt":
            continue
        print("[*] Get content from file", f)
        value = open(loc+f, "r", encoding="utf-8").read()
        for w in LIST:
            py = pinyin(w, v_to_u=True)
            de = "".join([str(x) for x in py])
            if value.find(w) != -1:
                print("[+] Find block word", w)
                if de == w:
                    de = ("".join([str(x)+" " for x in w]))[:-1]
                print("[+] Replace with", de)
                value = value.replace(w, de)
        print("[*] Write to file", loc+f)
        out = open(loc+f, "w", encoding="utf-8")
        print(value, file=out, end="")
        out.close()
        print("[*] Replaced", f, "done.")

if __name__ == "__main__":
    print("[!] Application running...")
    main(sys.argv)
    print("[!] Application exited.")