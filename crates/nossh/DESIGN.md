# Design

The tools is an ssh tool for quickly finding your relevant ssh endpoints. Using a fuzzy finder.

```
$ nossh
  1. ratchet:22
  2. somegateway:222
# 3. git.front.kjuulh.io # This is the selected item
> git.fr
# pressed: Enter
git.front.kjuulh.io$: echo 'now at this machine'
```

Based on its own config
Based on ~/.ssh/config
Cache history
nossh can be used as just a normal ssh

