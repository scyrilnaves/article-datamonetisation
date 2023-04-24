# Substrate first results

## About the first tests

- If 5 times same test -> tests finish rate worst at last test
  - High variance, no consistency ? 
- Default [block size = 5MB](../../substrate-node/substrate-node-template/runtime/src/lib.rs#L138)

## Global overview

![results](./preliminary_tests.jpg)

## Change the blocksize

### 5MB no special changes

![test](imgs/5MB_no_changes.png)

### 1MB no special changes

![test](imgs/1MB.png)

### 10MB no special changes

![test](imgs/10MB.png)

### Conclusion

> The block size **don't change** the performance significantly.


## Change the state cache size

### 10MB no special changes

![test](imgs/10MB_2GB_state_cache.png)

### 5MB no special changes

![test](imgs/5MB_2GB_state_cache.png)

## 10 tests

![test](imgs/10_consecutive_tests.png)