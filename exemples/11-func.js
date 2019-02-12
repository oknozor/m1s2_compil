var x = 10;
var y = 10;
var z = 10;

function blob(n) {
    function bar(n) {
        return n + 1;

        function fizz(n) {
            1+1;
            return n + 1;
        }

        function buzz(n) {
            function bloof(n) {
                return n + 1;
            }

            return n + 1;
        }

        function bazz(n) {
            return n + 1;
        }
    }
    function fofo() {
        print("woof", 1);
        print("pouet");
    }

    buzz(n);
    x = x + n;
    return x;
};

pif(blob(1));
paf(blob(2));
pouf(blob(2));
