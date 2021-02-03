

fn function(){
    println!("called 'function()'");
}

mod cool {
    pub fn function() {
        println!("called 'cool::function()':", );
    }
}

mod my {
    fn function() {
        println!("called 'my::function()'" );
    }
    mod cool {
        pub fn function() {
            println!("called 'my::cool::function()'" );
        }
    }

    pub fn indirect_call() {
        print!("called my::indirect_call()', that \n");

        self::function();
        function();

        //use self to acsess module inside 'my':
        self::cool::function();

        //super refers to parent scope(outside 'my'module):
        super::function();

        {
        use crate::cool::function as root_function;
        root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
