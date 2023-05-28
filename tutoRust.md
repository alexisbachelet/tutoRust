# Rust tutorial

# Git init

Init a git hub repo

```bash
git init
git remote add origin
git add .  # After gitignore
git commit -m "First commit"
git push -u orgin master  # Upstream branche.
```

Git ignore file:

```git
*
!*\
!*.*
```

To save password:

```bash
eval "$(ssh-agent -s)"
ssh-add myPrivateKey
```

Ignore all exept directory and file with an extension
