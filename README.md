# AfterDrop

`AfterDropBoxed` is normaly used to storeit in a struct,
 is not really use full because you will always need for a external crate to create a wrapper for that struct so you can implement a custom Drop!
But can be used for other crates to do something that thing is droped, but you define `AfterDropBoxed`

## Inspiration

- jai with defer keyword
- go with defer keyword

## Why

This is usefull for example if you want to do someting at the end of the function!
If you have a lot of branches in function,
 with more returns will be really difficult to do this,
 not really but you will need to call that function every time before drop!
But when using `?` rust operator that will return without you knowing, this is really usefull!
