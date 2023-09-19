# Data Engine
A high performance engine for batch processing data written as a Tauri App!

## Solution
Ever had to manually process 1K+ images for Machine Learning? Scrapping from the web is easy, 100K+ images can be grabbed in under an hour (assuming your scrapper is good to manuever rate limiting). However, it isn't fun at all to pop Photoshop/GIMP to edit 10K images is it?

This data engine is the solution. Manually processing images is no fun, so this engine opens an interface to label and cut your images! There are more features up ahead too!

## State
Currently, it is not production ready.
- The exports do not work. Should be patched relatively soon.
- It also is tied to UNIX-based as I had to use the /tmp directory for rendering cache.
- Performance Issues with blob urls: Should be patched soon when I use the file locations instead
- Label nodes do not currently work (should be fixed soon)

## Roadmap
- Patching all the listed above
- Node-based graph instead of a linear layer
- Additional Input nodes