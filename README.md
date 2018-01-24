Codesize comparison between [rusqlite](https://crates.io/crates/rusqlite) and [diesel](https://crates.io/crates/diesel) using [bloaty](https://github.com/google/bloaty) and [cargo bloat](https://github.com/RazrFalcon/cargo-bloat).
## With rusqlite
### 1 query
```bash
❯ bloaty target/release/with-rusqlite
     VM SIZE                                FILE SIZE
 --------------                          --------------
  46.9%   298Ki __TEXT,__text              298Ki  47.1%
  19.9%   126Ki String Table               126Ki  20.0%
  14.6%  92.9Ki Symbol Table              92.9Ki  14.7%
   4.0%  25.2Ki __TEXT,__eh_frame         25.2Ki   4.0%
   3.6%  22.7Ki __TEXT,__const            22.7Ki   3.6%
   2.4%  15.2Ki Export Info               15.2Ki   2.4%
   2.2%  13.8Ki __DATA,__const            13.8Ki   2.2%
   1.6%  10.1Ki __TEXT,__cstring          10.1Ki   1.6%
   0.9%  5.83Ki [Unmapped]                8.85Ki   1.4%
   1.0%  6.38Ki __TEXT,__gcc_except_tab   6.38Ki   1.0%
   0.5%  2.88Ki __TEXT,__unwind_info      2.88Ki   0.5%
   0.4%  2.68Ki Lazy Binding Info         2.68Ki   0.4%
   0.4%  2.66Ki [None]                         0   0.0%
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
❯ cargo bloat --release -n 10
 File  .text     Size Name
38.6%  80.9% 244.7KiB [978 Others]
 1.5%   3.1%   9.4KiB _stats_arena_print
 1.2%   2.5%   7.6KiB _je_stats_print
 1.2%   2.4%   7.3KiB std::sys_common::backtrace::output
 1.1%   2.4%   7.2KiB _je_mallocx
 1.1%   2.2%   6.7KiB _je_arena_boot
 0.9%   1.9%   5.8KiB _je_malloc_vsnprintf
 0.6%   1.3%   4.0KiB _malloc_init_hard_a0_locked
 0.6%   1.2%   3.6KiB _je_arena_palloc
 0.5%   1.0%   3.1KiB __mh_execute_header
 0.5%   1.0%   3.0KiB _je_rallocx
47.7% 100.0% 302.4KiB .text section size, the file size is 633.9KiB
```
### 2 queries
```bash
❯ bloaty target/release/with-rusqlite
     VM SIZE                                FILE SIZE
 --------------                          --------------
  47.1%   299Ki __TEXT,__text              299Ki  47.2%
  20.0%   127Ki String Table               127Ki  20.1%
  14.6%  93.1Ki Symbol Table              93.1Ki  14.7%
   4.0%  25.4Ki __TEXT,__eh_frame         25.4Ki   4.0%
   3.6%  22.8Ki __TEXT,__const            22.8Ki   3.6%
   2.4%  15.2Ki Export Info               15.2Ki   2.4%
   2.2%  13.9Ki __DATA,__const            13.9Ki   2.2%
   1.6%  10.1Ki __TEXT,__cstring          10.1Ki   1.6%
   0.6%  3.99Ki [Unmapped]                7.01Ki   1.1%
   1.0%  6.52Ki __TEXT,__gcc_except_tab   6.52Ki   1.0%
   0.5%  2.90Ki __TEXT,__unwind_info      2.90Ki   0.5%
   0.4%  2.68Ki Lazy Binding Info         2.68Ki   0.4%
   0.4%  2.59Ki __DATA,__common                0   0.0%
   0.3%  2.14Ki [None]                         0   0.0%
   0.3%  1.76Ki [10 Others]               1.32Ki   0.2%
   0.2%  1.43Ki Function Start Addresses  1.43Ki   0.2%
   0.2%  1.17Ki __TEXT,__stub_helper      1.17Ki   0.2%
   0.2%  1.02Ki Rebase Info               1.02Ki   0.2%
   0.2%    1000 __DATA,__data               1000   0.2%
   0.1%     972 Indirect Symbol Table        972   0.1%
   0.1%     944 __DATA,__la_symbol_ptr       944   0.1%
 100.0%   636Ki TOTAL                      633Ki 100.0%
```
```bash
❯ cargo bloat --release -n 10
 File  .text     Size Name
38.6%  80.9% 244.7KiB [978 Others]
 1.5%   3.1%   9.4KiB _stats_arena_print
 1.2%   2.5%   7.6KiB _je_stats_print
 1.2%   2.4%   7.3KiB std::sys_common::backtrace::output
 1.1%   2.4%   7.2KiB _je_mallocx
 1.1%   2.2%   6.7KiB _je_arena_boot
 0.9%   1.9%   5.8KiB _je_malloc_vsnprintf
 0.6%   1.3%   4.0KiB _malloc_init_hard_a0_locked
 0.6%   1.2%   3.6KiB _je_arena_palloc
 0.5%   1.0%   3.1KiB __mh_execute_header
 0.5%   1.0%   3.0KiB _je_rallocx
47.7% 100.0% 302.4KiB .text section size, the file size is 633.9KiB
```
### Dependencies
```bash
 File  .text     Size Name
30.8%  64.6% 195.2KiB std
14.8%  30.9%  93.5KiB [Unknown]
 1.6%   3.3%   9.9KiB rusqlite
 0.4%   0.9%   2.8KiB with_rusqlite
 0.1%   0.2%     752B libsqlite3_sys
 0.0%   0.1%     208B linked_hash_map
47.7% 100.0% 302.4KiB .text section size, the file size is 633.9KiB
```
## With Diesel
### 1 query
```bash
❯ bloaty target/release/with-diesel
     VM SIZE                                FILE SIZE
 --------------                          --------------
  48.5%   366Ki __TEXT,__text              366Ki  48.7%
  19.6%   147Ki String Table               147Ki  19.7%
  13.2%  99.5Ki Symbol Table              99.5Ki  13.2%
   4.4%  33.4Ki __TEXT,__eh_frame         33.4Ki   4.4%
   3.6%  27.3Ki __TEXT,__const            27.3Ki   3.6%
   2.5%  19.1Ki Export Info               19.1Ki   2.5%
   1.9%  14.6Ki __DATA,__const            14.6Ki   1.9%
   1.4%  10.7Ki __TEXT,__gcc_except_tab   10.7Ki   1.4%
   1.3%  10.1Ki __TEXT,__cstring          10.1Ki   1.3%
   0.8%  6.42Ki [Unmapped]                9.43Ki   1.3%
   0.5%  3.64Ki __TEXT,__unwind_info      3.64Ki   0.5%
   0.4%  3.06Ki [None]                         0   0.0%
   0.3%  2.59Ki __DATA,__common                0   0.0%
   0.3%  2.37Ki Lazy Binding Info         2.37Ki   0.3%
   0.2%  1.73Ki [10 Others]               1.31Ki   0.2%
   0.2%  1.69Ki Function Start Addresses  1.69Ki   0.2%
   0.1%  1.07Ki __TEXT,__stub_helper      1.07Ki   0.1%
   0.1%  1.06Ki Rebase Info               1.06Ki   0.1%
   0.1%    1000 __DATA,__data               1000   0.1%
   0.1%     892 Indirect Symbol Table        892   0.1%
   0.1%     864 __DATA,__la_symbol_ptr       864   0.1%
 100.0%   756Ki TOTAL                      752Ki 100.0%
```
```bash
❯ cargo bloat --release -n 10
 File  .text     Size Name
41.3%  83.8% 309.3KiB [1151 Others]
 1.3%   2.5%   9.4KiB _stats_arena_print
 1.0%   2.1%   7.6KiB _je_stats_print
 1.0%   2.0%   7.3KiB std::sys_common::backtrace::output
 1.0%   1.9%   7.2KiB _je_mallocx
 0.9%   1.8%   6.7KiB _je_arena_boot
 0.8%   1.6%   5.8KiB _je_malloc_vsnprintf
 0.5%   1.1%   4.0KiB <diesel::connection::statement_cache::StatementCache<DB, Statement>>::cached_statement
 0.5%   1.1%   4.0KiB <diesel::connection::statement_cache::StatementCache<DB, Statement>>::cached_statement
 0.5%   1.1%   4.0KiB _malloc_init_hard_a0_locked
 0.5%   1.0%   3.9KiB <diesel::connection::statement_cache::StatementCache<DB, Statement>>::cached_statement
49.3% 100.0% 369.2KiB .text section size, the file size is 749.0KiB
```
### 2 queries
```bash
❯ bloaty target/release/with-diesel
     VM SIZE                                FILE SIZE
 --------------                          --------------
  49.0%   377Ki __TEXT,__text              377Ki  49.2%
  19.4%   150Ki String Table               150Ki  19.5%
  13.0%   100Ki Symbol Table               100Ki  13.0%
   4.4%  34.1Ki __TEXT,__eh_frame         34.1Ki   4.4%
   3.5%  27.3Ki __TEXT,__const            27.3Ki   3.6%
   2.5%  19.6Ki Export Info               19.6Ki   2.5%
   1.9%  14.7Ki __DATA,__const            14.7Ki   1.9%
   1.5%  11.6Ki __TEXT,__gcc_except_tab   11.6Ki   1.5%
   1.3%  10.1Ki __TEXT,__cstring          10.1Ki   1.3%
   0.5%  3.59Ki [Unmapped]                6.60Ki   0.9%
   0.7%  5.79Ki __TEXT,__unwind_info      5.79Ki   0.8%
   0.5%  3.98Ki [None]                         0   0.0%
   0.3%  2.59Ki __DATA,__common                0   0.0%
   0.3%  2.37Ki Lazy Binding Info         2.37Ki   0.3%
   0.2%  1.75Ki [10 Others]               1.33Ki   0.2%
   0.2%  1.71Ki Function Start Addresses  1.71Ki   0.2%
   0.1%  1.07Ki __TEXT,__stub_helper      1.07Ki   0.1%
   0.1%  1.06Ki Rebase Info               1.06Ki   0.1%
   0.1%    1000 __DATA,__data               1000   0.1%
   0.1%     892 Indirect Symbol Table        892   0.1%
   0.1%     864 __DATA,__la_symbol_ptr       864   0.1%
 100.0%   772Ki TOTAL                      768Ki 100.0%
```
```bash
❯ cargo bloat --release -n 10
 File  .text     Size Name
41.9%  84.3% 321.5KiB [1167 Others]
 1.2%   2.5%   9.4KiB _stats_arena_print
 1.0%   2.0%   7.6KiB _je_stats_print
 1.0%   1.9%   7.3KiB std::sys_common::backtrace::output
 0.9%   1.9%   7.2KiB _je_mallocx
 0.9%   1.8%   6.7KiB _je_arena_boot
 0.8%   1.5%   5.8KiB _je_malloc_vsnprintf
 0.5%   1.1%   4.0KiB <diesel::connection::statement_cache::StatementCache<DB, Statement>>::cached_statement
 0.5%   1.1%   4.0KiB <diesel::connection::statement_cache::StatementCache<DB, Statement>>::cached_statement
 0.5%   1.0%   4.0KiB _malloc_init_hard_a0_locked
 0.5%   1.0%   3.9KiB <diesel::connection::statement_cache::StatementCache<DB, Statement>>::cached_statement
49.7% 100.0% 381.4KiB .text section size, the file size is 768.0KiB
```
### Dependencies
```bash
❯ cargo bloat --release --crates -n 10
 File  .text     Size Name
29.3%  58.9% 224.8KiB std
13.1%  26.5% 100.9KiB [Unknown]
 6.2%  12.5%  47.6KiB diesel
 0.4%   0.8%   3.1KiB libsqlite3_sys
 0.4%   0.7%   2.8KiB with_diesel
 0.3%   0.6%   2.1KiB migrations_internals
49.7% 100.0% 381.4KiB .text section size, the file size is 768.0KiB
```
