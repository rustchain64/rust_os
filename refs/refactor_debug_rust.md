# Common Errors
Apart from memory leaks, which are unfortunate but don’t make the program vulnerable to attackers, there are two common types of bugs with more severe consequences:

When we accidentally continue to use a variable after calling deallocate on it, we have a so-called use-after-free vulnerability. Such a bug causes undefined behavior and can often be exploited by attackers to execute arbitrary code.
When we accidentally free a variable twice, we have a double-free vulnerability. This is problematic because it might free a different allocation that was allocated in the same spot after the first deallocate call. Thus, it can lead to a use-after-free vulnerability again.
These types of vulnerabilities are commonly known, so one might expect that people have learned how to avoid them by now. But no, such vulnerabilities are still regularly found, for example this use-after-free vulnerability in Linux (2019), that allowed arbitrary code execution. A web search like use-after-free linux {current year} will probably always yield results. This shows that even the best programmers are not always able to correctly handle dynamic memory in complex projects.
