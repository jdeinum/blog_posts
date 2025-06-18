---
title: 
date: 
---

<h1 style="text-align: center;">Atomic Playbooks</h1>

## Overview

### Goal
Being able to have a remote process rollback all actions done, even if the
ssh connection fails.

### Requirements
1. No permanent agent running on the target device

### Ideas
1. Something similar to tmux and how you can create a separate process that will
   either ensure completion, or rolls back to the previous state
2. Store runs as files on the target device, reading them the next time the
   device has a playbook run on it. That way we can see what happened and then
    delete the file after.
