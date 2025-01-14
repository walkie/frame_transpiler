#StateContextSm
    -interface-
    Inc : i32
    Next [arg:i32]

    -machine-
    $Init
        |>| -> (3 5) $Foo ^
    
    $Foo
        var x:i32 = 0
        
        |>| [a:i32 b:i32]
            log("a" a)
            log("b" b)
            x = a * b
            log("x" x)
            ^
        
        |<| [c:i32]
            log("c" c)
            x = x + c
            log("x" x)
            ^
        
        |Inc|
            x = x + 1
            log("x" x)
            ^(x)
        
        |Next| [arg:i32]
            var tmp = arg * 10  --- FIXME: Swapping this to 10 * arg causes a parse error!
            (10) -> (tmp) $Bar(x)
            ^

    $Bar [y:i32]
        
        var z:i32 = 0
        
        |>| [a:i32]
            log("a" a)
            log("y" y)
            z = a + y
            log("z" z)
            ^
        
        |Inc|
            z = z + 1
            log("z" z)
            ^(z)
    
    -actions-
    log [name:String val:i32]

    -domain-
    var tape:Log = `vec![]`
##
