contract A {
    constructor(int _x) {}
}

contract Test {
    function foo() public {
        new A({_x: 2});
    }
}