// Make sure to correctly set the library path when executing
// Compile: javac RustBridge.java
// Execute: java -Djava.library.path=../../target/release/ RustBridge

class RustBridge {
    static {
        System.loadLibrary("java_bridge"); // Load native
                                           // library
    }

    private static native float addTwoFloats(float a, float b); // Declare native method

    private static native String greetPerson(String personJSON); // Declare native method

    public static void main(String[] args) {
        System.out.println("Welcome to this Demo.");

        float result = addTwoFloats(4.0f, 2.0f);
        System.out.println("Result: " + result);

        Person p = new Person("Aaron", 23, null);
        System.out.println(greetPerson(p));
    }

    static class Person {
        public String name;
        public int age;
        public Person[] children;

        public Person(String name, int age, Person[] children) {
            this.name = name;
            this.age = age;
            this.children = children;
        }

        // Just for demonstration purposes
        // In real scenarios use something like https://github.com/google/gson
        public String toJSON() {
            String childrenString = "null";
            if (this.children != null) {
                String[] childrenParts = new String[this.children.length];

                for (int i = 0; i < this.children.length; i++) {
                    childrenParts[i] = this.children[i].toJSON();
                }
                childrenString = "\"" + String.join(",", childrenParts) + "\"";
            }
            return "{\"name\":" + "\"" + this.name + "\",\"age\":" + this.age + ",\"children\": " + childrenString
                    + "}";
        }
    }

    // Expose certain functions for (guarded) outside use
    public static String greetPerson(Person p) {
        return greetPerson(p.toJSON());
    }

}