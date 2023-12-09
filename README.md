# AfterDrop

`AfterDropBoxed` is normaly used to store it in a struct,
Can be used for other crates to do something when a thing is droped!

## Inspiration

- jai with defer keyword
- go with defer keyword

## Why

This is usefull for example if you want to do someting at the end of the function!
If you have a lot of branches in function,
 with more returns will be really difficult to do this,
 not really but you will need to call that function every time `return`!
But when using `?` rust operator that will return without you knowing, this is really usefull!
