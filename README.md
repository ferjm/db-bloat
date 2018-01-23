
```bash
❯ bloaty target/release/with-rusqlite
     VM SIZE                                FILE SIZE
 --------------                          --------------
  46.8%   297Ki __TEXT,__text              297Ki  47.0%
  19.9%   126Ki String Table               126Ki  20.0%
  14.6%  92.9Ki Symbol Table              92.9Ki  14.7%
   3.9%  25.1Ki __TEXT,__eh_frame         25.1Ki   4.0%
   3.6%  22.7Ki __TEXT,__const            22.7Ki   3.6%
   2.4%  15.2Ki Export Info               15.2Ki   2.4%
   2.2%  13.8Ki __DATA,__const            13.8Ki   2.2%
   1.6%  10.1Ki __TEXT,__cstring          10.1Ki   1.6%
   1.0%  6.26Ki [Unmapped]                9.29Ki   1.5%
   1.0%  6.37Ki __TEXT,__gcc_except_tab   6.37Ki   1.0%
   0.5%  2.96Ki [None]                         0   0.0%
   0.5%  2.88Ki __TEXT,__unwind_info      2.88Ki   0.5%
   0.4%  2.68Ki Lazy Binding Info         2.68Ki   0.4%
   0.4%  2.59Ki __DATA,__common                0   0.0%
   0.3%  1.76Ki [10 Others]               1.32Ki   0.2%
   0.2%  1.42Ki Function Start Addresses  1.42Ki   0.2%
   0.2%  1.17Ki __TEXT,__stub_helper      1.17Ki   0.2%
   0.2%  1.02Ki Rebase Info               1.02Ki   0.2%
   0.2%    1000 __DATA,__data               1000   0.2%
   0.1%     972 Indirect Symbol Table        972   0.1%
   0.1%     944 __DATA,__la_symbol_ptr       944   0.1%
 100.0%   636Ki TOTAL                      633Ki 100.0%
```

```bash
❯ bloaty target/release/with-diesel
     VM SIZE                                FILE SIZE
 --------------                          --------------
  48.6%   373Ki __TEXT,__text              373Ki  48.6%
  19.8%   152Ki String Table               152Ki  19.8%
  13.1%   100Ki Symbol Table               100Ki  13.1%
   4.5%  34.7Ki __TEXT,__eh_frame         34.7Ki   4.5%
   3.5%  27.3Ki __TEXT,__const            27.3Ki   3.6%
   2.6%  20.2Ki Export Info               20.2Ki   2.6%
   1.9%  14.7Ki __DATA,__const            14.7Ki   1.9%
   1.5%  11.5Ki __TEXT,__gcc_except_tab   11.5Ki   1.5%
   1.3%  10.1Ki __TEXT,__cstring          10.1Ki   1.3%
   0.9%  6.55Ki __TEXT,__unwind_info      6.55Ki   0.9%
   0.4%  3.21Ki [Unmapped]                6.22Ki   0.8%
   0.3%  2.59Ki __DATA,__common                0   0.0%
   0.3%  2.39Ki Lazy Binding Info         2.39Ki   0.3%
   0.2%  1.73Ki Function Start Addresses  1.73Ki   0.2%
   0.2%  1.50Ki [10 Others]                  728   0.1%
   0.1%  1.08Ki __TEXT,__stub_helper      1.08Ki   0.1%
   0.1%  1.08Ki Rebase Info               1.08Ki   0.1%
   0.1%    1000 __DATA,__data               1000   0.1%
   0.1%     900 Indirect Symbol Table        900   0.1%
   0.1%     872 __DATA,__la_symbol_ptr       872   0.1%
   0.1%     654 __TEXT,__stubs               654   0.1%
 100.0%   768Ki TOTAL                      767Ki 100.0%
```
