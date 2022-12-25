class MyClass {
    public:
        MyClass(int myParam);
        [[nodiscard]] int getMyInt();
    private:
        int myInt;
};